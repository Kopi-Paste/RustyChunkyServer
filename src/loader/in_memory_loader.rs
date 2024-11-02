use std::collections::HashMap;

use axum::body::Bytes;

use super::loader_trait::Loader;

#[derive(Clone)]
pub struct InMemoryLoader  {
    storage : HashMap<String, Vec<u8>>
}

impl Loader for InMemoryLoader {
    fn init() -> Self {
        InMemoryLoader { storage : HashMap::new() }
    }

    fn exists(&self, name : &String) -> bool {
        self.storage.contains_key(name)
    }

    fn save(&mut self, path : &String, data : Bytes) {
        if !self.exists(path) {
            self.storage.insert(path.clone(), Vec::new());
        }

        self.storage.get_mut(path).unwrap().extend( data.to_vec());
    }

    fn load(&mut self, name : &String) -> Bytes {
        Bytes::from(self.storage.get(name).unwrap_or(&Vec::new()).clone())
    }

    fn len(&self) -> usize {
        self.storage.len()
    }
}