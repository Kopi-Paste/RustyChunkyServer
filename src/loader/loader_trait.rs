use tokio::sync::RwLock;
use super::saved_file::SavedFile;

/// A general trait of a loader of files with prefix listing enabled
pub trait PrefixLoader {
    /// Initializes empty loader
    fn init() -> Self;
    /// Returns whether file with given name exists
    fn exists(&self, name : &String) -> bool;
    /// Inserts a new file with given name and file type
    fn insert_new(&mut self, name : &String, mime : &String);
    /// Obtains a RW lock on file with given name
    fn load(&self, name : &String) -> Option<& RwLock<SavedFile>>;
    /// Deletes a file, returns tru if deletion was succesful (file exits), else false
    fn delete(&mut self, name : &String) -> bool;
    /// Returns file names with given prefix
    fn get_keys_for_prefix(&self, prefix : &String) -> Vec<String>;
}
