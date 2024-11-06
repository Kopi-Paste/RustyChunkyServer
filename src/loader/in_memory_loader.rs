use tokio::sync::RwLock;

use crate::trie::trie::Trie;

use super::{loader_trait::Loader, saved_file::SavedFile};

pub struct InMemoryLoader  {
    storage : Trie<char, RwLock<SavedFile>>
}

macro_rules! as_slice {
    ($x:expr) => {
        &$x.chars().collect::<Vec<_>>()[..]
    };
}

impl Loader for InMemoryLoader {
    fn init() -> Self {
        InMemoryLoader { storage : Trie::init() }
    }

    fn exists(&self, name : &String) -> bool {
        self.storage.contains(as_slice!(name))
    }

    fn insert_new(&mut self, name : &String, mime : &String) {
        self.storage.insert(as_slice!(name), RwLock::new(SavedFile::new(Vec::new(), mime.clone())));
    }

    fn load(&self, name : &String) -> Option<& RwLock<SavedFile>> {
        self.storage.get_for_string(as_slice!(name))
    }

    fn delete(&mut self, name : &String) -> bool {
        self.storage.delete(as_slice!(name))
    }
    
    fn get_keys_for_prefix(&self, prefix : &String) -> Vec<String> {
        self.storage.get_keys_for_prefix(as_slice!(prefix)).iter().map(|str| String::from_iter(str.iter())).collect()
    }
}
