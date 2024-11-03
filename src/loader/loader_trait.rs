use super::saved_file::SavedFile;
use axum::body::Bytes;

pub trait Loader {
    fn init() -> Self;
    fn exists(&self, name : &String) -> bool;
    fn save(&mut self, path : &String, data : Bytes, mime_type : String);
    fn load(&self, name : &String) -> Option<&SavedFile>;
    fn len(&self) -> usize;
}