use std::collections::HashMap;

use super::{loader_trait::Loader, saved_file::SavedFile};

#[derive(Clone)]
pub struct InMemoryLoader  {
    storage : HashMap<String, SavedFile>
}

impl Loader for InMemoryLoader {
    fn init() -> Self {
        InMemoryLoader { storage : HashMap::new() }
    }

    fn exists(&self, name : &String) -> bool {
        self.storage.contains_key(name)
    }

    fn insert_new(&mut self, name : &String, mime : &String) {
        self.storage.insert(name.clone(), SavedFile::new(Vec::new(), mime.clone()));
    }

    fn get_mut(&mut self, name : &String) -> Option<&mut SavedFile> {
        self.storage.get_mut(name)
    }

    fn load(&self, name : &String) -> Option<&SavedFile> {
        self.storage.get(name)
    }

    fn delete(&mut self, name : &String) -> bool {
        self.storage.remove(name).is_some()
    }
}
