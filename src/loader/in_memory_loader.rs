use std::collections::HashMap;

use axum::body::Bytes;

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

    fn save(&mut self, path : &String, data : Bytes, mime_type : String) {
        if !self.exists(path) {
            self.storage.insert(path.clone(), SavedFile::new(data, mime_type));
            return;
        }

        let existing_data = self.storage.get_mut(path).unwrap();
        existing_data.extend( data);
    }

    fn load(&self, name : &String) -> Option<&SavedFile> {
        return self.storage.get(name);
    }

    fn len(&self) -> usize {
        self.storage.len()
    }
}