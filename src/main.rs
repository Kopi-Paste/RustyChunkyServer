use std::{sync::Arc, usize};
use crate::path::Path;
use async_stream::stream;
use axum::{
    body::{Body, Bytes}, extract::{path, State}, http::{Error, HeaderMap, Response, StatusCode}, response::IntoResponse, routing::{delete, get, put}, Router
};
use futures::StreamExt;
use loader::{in_memory_loader::InMemoryLoader, loader_trait::Loader};
use tokio::sync::RwLock;

mod loader;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let loader = InMemoryLoader::init();
    let shared_state = Arc::new(RwLock::new(loader));

    // build our application with a route
    let app = Router::new()
        .route("/*path", get(on_get_handler))
        .route("/*path", put(on_put_handler))
        .route("/*path", delete(on_delete_handler))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn on_get_handler(Path(path) : Path<String>, State(state) : State<Arc<RwLock<InMemoryLoader>>>) -> Result<impl IntoResponse, (StatusCode, String)> {
  println!("GET called on {}", path);
  let state_clone = state.clone();
  let loader = state.read().await;

  if !loader.exists(&path) {
    return Err((StatusCode::NOT_FOUND, format!("Path {} does not exist", path)));
  }

  let mime = loader.load(&path).unwrap().read().await.get_mime().clone();

  let stream = stream! {
    let mut sent_bytes = 0 as usize;
    let chunk_size = 8192 as usize;
    let loader = state_clone.read().await;
    loop {
      if let Some(data) = loader.load(&path) {
        let len = data.read().await.get_data().len();
        if sent_bytes + chunk_size > len {
          yield Ok::<_, Error>(Bytes::copy_from_slice(&data.read().await.get_data()[sent_bytes..len]));
          break;
        }
        yield Ok::<_, Error>(Bytes::copy_from_slice(&data.read().await.get_data()[sent_bytes..sent_bytes + chunk_size]));
        sent_bytes += chunk_size;
      }
      else {
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

async fn on_put_handler(Path(path) : Path<String>, State(state) : State<Arc<RwLock<InMemoryLoader>>>, headers : HeaderMap, payload : Body) -> Result<impl IntoResponse, (StatusCode, String)> {
  println!("PUT called on {}", path);
  let mut stream = payload.into_data_stream();
  let mime_type = headers.get("Content-Type").map_or("text/plain", |header| header.to_str().unwrap());

  let mut writing_lock_guard = state.write().await;
  writing_lock_guard.insert_new(&path, &mime_type.to_string());
  drop(writing_lock_guard);
  
  while let Some(chunk) = stream.next().await {
    match chunk {
      Ok(data) => {
        let writing_lock_guard = state.read().await;
        if let Some(modified_entry) = writing_lock_guard.load(&path) {
          modified_entry.write().await.extend(data);
        }
        else {
          return Err((StatusCode::GONE, "Uploaded path was deleted".to_string()));
        }
      }
      Err(e) => {
        println!("Error: {}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error writing the file".to_string()));
      }
    }
  }
  Ok(())
}

async fn on_delete_handler(Path(path) : Path<String>, State(state) : State<Arc<RwLock<InMemoryLoader>>>) -> Result<impl IntoResponse, (StatusCode, String)> {
  println!("DELETE called on {}", path);
  if state.write().await.delete(&path) {
    Ok(())
  }
  else {
    Err((StatusCode::NOT_FOUND, format!("Path {} was not found", path)))
  }
}
