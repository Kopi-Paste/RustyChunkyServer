use std::collections::HashMap;

use tokio::sync::RwLock;

use super::{loader_trait::Loader, saved_file::SavedFile};

pub struct InMemoryLoader  {
    storage : HashMap<String, RwLock<SavedFile>>
}

impl Loader for InMemoryLoader {
    fn init() -> Self {
        InMemoryLoader { storage : HashMap::new() }
    }

    fn exists(&self, name : &String) -> bool {
        self.storage.contains_key(name)
    }

    fn insert_new(&mut self, name : &String, mime : &String) {
        self.storage.insert(name.clone(), RwLock::new(SavedFile::new(Vec::new(), mime.clone())));
    }

    fn load(&self, name : &String) -> Option<& RwLock<SavedFile>> {
        self.storage.get(name)
    }

    fn delete(&mut self, name : &String) -> bool {
        self.storage.remove(name).is_some()
    }
}
