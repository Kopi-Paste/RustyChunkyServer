use std::{sync::Arc, usize};

use crate::path::Path;

mod loader;

use axum::{
    body::{to_bytes, Body, BodyDataStream, Bytes}, extract::{path, State}, http::{HeaderValue, StatusCode}, response::{IntoResponse, Response}, routing::{get, put}, Router
};
use futures::StreamExt;
use loader::{in_memory_loader::InMemoryLoader, loader_trait::Loader};
use tokio::sync::Mutex;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let loader = InMemoryLoader::init();
    let shared_state = Arc::new(Mutex::new(loader));

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/*path", get(on_get_handler))
        // `POST /users` goes to `create_user`
        .route("/*path", put(on_put_handler))
        .with_state(shared_state);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn on_get_handler(Path(path) : Path<String>, State(state) : State<Arc<Mutex<InMemoryLoader>>>) -> Result<impl IntoResponse, (StatusCode, String)> {
  println!("GET called on {}", path);

  let loader = &mut *state.lock().await;

  if !loader.exists(&path) {
    return Err((StatusCode::NOT_FOUND, format!("{} not found on server", &path)))
  }

  let response = loader.load(&path);
  println!("Returning response");
    Ok(Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "video/mp4")
    .body(Body::from(response))
    .unwrap())
}

async fn on_put_handler(Path(path) : Path<String>, State(state) : State<Arc<Mutex<InMemoryLoader>>>, mut payload : Body) -> Result<impl IntoResponse, (StatusCode, String)> {
  println!("PUT called on {}", path);
  let mut stream = payload.into_data_stream();
  while let Some(chunk) = stream.next().await {
    match chunk {
      Ok(data) => {
        state.lock().await.save(&path.to_string(), data);
      }
      Err(e) => {
        println!("Error: {}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error writing the file".to_string()));
      }
    }
  }
  Ok(())
}