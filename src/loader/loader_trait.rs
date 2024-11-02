use axum::body::Bytes;

pub trait Loader {
    fn init() -> Self;
    fn exists(&self, name : &String) -> bool;
    fn save(&mut self, name : &String, data : Bytes);
    fn load(&mut self, name : &String) -> Bytes;
    fn len(&self) -> usize;
}