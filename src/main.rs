use crate::path::Path;
use async_stream::stream;
use axum::{
    body::{Body, Bytes},
    extract::{path, State},
    http::{HeaderMap, Method, Response, StatusCode},
    response::IntoResponse,
    routing::{any, delete, get, put},
    Router,
};
use futures::StreamExt;
use loader::{in_memory_loader::InMemoryLoader, loader_trait::PrefixLoader};
use std::{io::{Error, ErrorKind}, sync::Arc, usize};
use tokio::sync::RwLock;

mod loader;
mod trie;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let loader = InMemoryLoader::init();
    let shared_state = Arc::new(RwLock::new(loader));

    // routing
    let app = Router::new()
        .route("/*path", get(on_get_handler))
        .route("/*path", put(on_put_handler))
        .route("/*path", delete(on_delete_handler))
        // any is for custom method LIST
        .route("/*path", any(on_any_handler))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn on_get_handler(
    Path(path): Path<String>,
    State(state): State<Arc<RwLock<InMemoryLoader>>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("GET called on {}", path);
    // Clones a state (ARC is cheaply clonable) to use in stream
    let state_clone = state.clone();
    let loader = state.read().await;

    // If file does not exist, return 404
    if !loader.exists(&path) {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Path {} does not exist", path),
        ));
    }

    // Clone MIME type to send it into response
    let mime = loader.load(&path).unwrap().read().await.get_mime().clone();

    // This createas a stream of data for chunked response
    let stream = stream! {
        let mut sent_bytes = 0 as usize;
        let chunk_size = 8192 as usize;
        loop {
            let loader = state_clone.read().await;
            if let Some(data) = loader.load(&path) {
                let len = data.read().await.get_data().len();
                if sent_bytes + chunk_size > len {
                    // Final chunk send
                    yield Ok::<_, Error>(Bytes::copy_from_slice(&data.read().await.get_data()[sent_bytes..]));
                    break;
                }
                yield Ok::<_, Error>(Bytes::copy_from_slice(&data.read().await.get_data()[sent_bytes..sent_bytes + chunk_size]));
                sent_bytes += chunk_size;
            }
            else {
                yield Err::<_, Error>(Error::new(ErrorKind::BrokenPipe, "Content was deleted"));
                break;
            }
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", mime)
        .header("Transfer-Encoding", "chunked")
        .body(Body::from_stream(stream))
        .unwrap())
}

async fn on_put_handler(
    Path(path): Path<String>,
    State(state): State<Arc<RwLock<InMemoryLoader>>>,
    headers: HeaderMap,
    payload: Body,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("PUT called on {}", path);
    // Data stream
    let mut stream = payload.into_data_stream();

    // If mime unset we go with text/plain
    let mime_type = headers
        .get("Content-Type")
        .map_or("text/plain", |header| header.to_str().unwrap());

    // We only need to get the write guard on RW lock when inserting into storage, then only a read lock is enough
    {
        let mut writing_lock_guard = state.write().await;
        writing_lock_guard.insert_new(&path, &mime_type.to_string());
    }  // Drop writing_lock_guard here

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(data) => {
                let writing_lock_guard = state.read().await;
                if let Some(modified_entry) = writing_lock_guard.load(&path) {
                    // Add data from chunk into saved file
                    modified_entry.write().await.extend(data.to_vec());
                } else {
                    return Err((StatusCode::GONE, "Uploaded path was deleted".to_string()));
                }
            }
            Err(e) => {
                // This could be some error in reading the request
                println!("Error: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error writing the file".to_string(),
                ));
            }
        }
    }
    Ok(())
}

async fn on_delete_handler(
    Path(path): Path<String>,
    State(state): State<Arc<RwLock<InMemoryLoader>>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("DELETE called on {}", path);
    if state.write().await.delete(&path) {
        Ok(())
    }
    else {
        Err((
            StatusCode::NOT_FOUND,
            format!("Path {} does not exist", path),
        ))
    }
}

async fn on_any_handler(
    Path(mut path): Path<String>,
    method: Method,
    State(state): State<Arc<RwLock<InMemoryLoader>>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if method.to_string() != "LIST" {
        return Err((
            StatusCode::METHOD_NOT_ALLOWED,
            "Method not allowed".to_string(),
        ));
    }

    println!("LIST called on {}", path);
    
    if path.ends_with('*') {
        path.truncate(path.len() - 1);  // Remove the star
        let body_data = state.read().await.get_keys_for_prefix(&path).join("\r\n");
        Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::from(body_data))
        .unwrap())
    } else if state.read().await.exists(&path) {
        let body_data = path;   
        Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::from(body_data))
        .unwrap())
    }
    else {
        Err((
            StatusCode::NOT_FOUND,
            "Given path does not exist, end path with * to enable prefix search".to_string(),
        ))
    }
}
