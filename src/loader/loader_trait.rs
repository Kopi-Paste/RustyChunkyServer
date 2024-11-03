use super::saved_file::SavedFile;

pub trait Loader {
    fn init() -> Self;
    fn exists(&self, name : &String) -> bool;
    fn insert_new(&mut self, name : &String, mime : &String);
    fn get_mut(&mut self, name : &String) -> Option<&mut SavedFile>;
    fn load(&self, name : &String) -> Option<&SavedFile>;
    fn delete(&mut self, name : &String) -> bool;
}
