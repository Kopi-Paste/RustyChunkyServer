use tokio::sync::RwLock;

use crate::trie::trie::Trie;

use super::{loader_trait::PrefixLoader, saved_file::SavedFile};

/// An implementation of prefix loader with all data being held in RAM
pub struct InMemoryLoader  {
    /// As a storage we are using a prefix tree mapping char slices onto saved files behind read-write locks
    storage : Trie<char, RwLock<SavedFile>>
}

/// Converts a string to a char slice
macro_rules! as_slice {
    ($x:expr) => {
        &$x.chars().collect::<Vec<_>>()[..]
    };
}

impl PrefixLoader for InMemoryLoader {
    /// Initzializes empty storage
    fn init() -> Self {
        InMemoryLoader { storage : Trie::init() }
    }

    /// Returns whether file with given name exists
    /// This is O(n) where n is the length of string
    /// In context of saved strings this is O(1)
    fn exists(&self, name : &String) -> bool {
        self.storage.contains(as_slice!(name))
    }

    /// Initializes new file eith no data and given mime type
    /// This is O(n) where n is the length of string
    /// In context of saved strings this is O(1)
    fn insert_new(&mut self, name : &String, mime : &String) {
        self.storage.insert(as_slice!(name), RwLock::new(SavedFile::new(Vec::new(), mime.clone())));
    }

    /// Obtains a RWLock onto saved file from storage
    /// This is O(n) where n is the length of string
    /// In context of saved strings this is O(1)
    fn load(&self, name : &String) -> Option<& RwLock<SavedFile>> {
        self.storage.get_for_string(as_slice!(name))
    }

    /// Removes a file from storage if it exists, returns true if removed, else false
    /// This is O(n) where n is the length of string
    /// In context of saved strings this is O(1)
    fn delete(&mut self, name : &String) -> bool {
        self.storage.delete(as_slice!(name))
    }
    
    /// Returns all keys starting with given prefix
    /// This is O(n) where n is the length of the response
    /// In context of saved strings this is O(n * l), where n is number of strings and l is the longest string
    fn get_keys_for_prefix(&self, prefix : &String) -> Vec<String> {
        self.storage.get_keys_for_prefix(as_slice!(prefix)).iter().map(|str| String::from_iter(str.iter())).collect()
    }
}
