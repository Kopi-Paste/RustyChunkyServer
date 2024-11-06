use tokio::sync::RwLock;
use super::saved_file::SavedFile;

pub trait Loader {
    fn init() -> Self;
    fn exists(&self, name : &String) -> bool;
    fn insert_new(&mut self, name : &String, mime : &String);
    fn load(&self, name : &String) -> Option<& RwLock<SavedFile>>;
    fn delete(&mut self, name : &String);
    fn get_keys_for_prefix(&self, prefix : &String) -> Vec<String>;
}
