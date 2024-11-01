use std::sync::Arc;

use crate::path::Path;

mod loader;

use axum::{
    body::Bytes, extract::{multipart::Field, path, Multipart, State}, http::StatusCode, response::{IntoResponse, Response}, routing::{get, put}, Router
};
use loader::{in_memory_loader::InMemoryLoader, loader_trait::Loader};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let loader = InMemoryLoader::<Bytes>::init();
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

async fn load_file(name : &String, loader : &mut InMemoryLoader<Bytes>) -> Option<Bytes> {
  return loader.load(name).cloned();
}

// basic handler that responds with a static string
async fn on_get_handler(Path(path) : Path<String>, State(state) : State<Arc<Mutex<InMemoryLoader<Bytes>>>>) -> Result<Response, (StatusCode, String)> {
  println!("GET called on {}", path);
  if let Some(response) = load_file(&path, &mut *state.lock().await).await {
    Ok(response.into_response())
  }
  else {
    Err((StatusCode::NOT_FOUND, format!("{} not found on server", &path)))
  }
}

async fn handle_field(field : Field<'_>, path : &str, file_count : i32, file_handler : &mut InMemoryLoader<Bytes>) -> () {
  let field_name = field.name().unwrap_or("Not set");
  if field_name != "File" {
    return;
  }
  
  let filename = field.file_name().map(|name| name.to_string()).unwrap_or_else(|| format!("file{}", file_handler.len()));
  println!("Saving file {} to path {}", filename, path);

  // Read the field’s data bytes
  match field.bytes().await {
    Ok(data) => {
      file_handler.save(&path.to_string(), data);
    }
    Err(e) => {
      println!("Error reading bytes for file {}: {}", file_count, e);
    }
  }
}

async fn on_put_handler(Path(path) : Path<String>, State(state) : State<Arc<Mutex<InMemoryLoader<Bytes>>>>, mut multipart : Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
  let mut file_count = 0;
  println!("PUT called on {}", path);
  loop {
    // Process each field in the multipart form data
    match multipart.next_field().await  {
      Ok(field) => {
        if let Some(field_unwrapped) = field  {
          handle_field(field_unwrapped, &path, file_count, &mut *state.lock().await).await;
          file_count += 1;
        }
        else {
          return Ok(())
        }
      }
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error when handling field number {} {}", file_count, e)));
      }
    }
  }
}