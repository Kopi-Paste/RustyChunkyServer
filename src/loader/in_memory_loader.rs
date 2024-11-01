use std::collections::HashMap;

use super::loader_trait::Loader;

#[derive(Clone)]
pub struct InMemoryLoader<T>  {
    storage : HashMap<String, T>
}

impl<T> Loader<T> for InMemoryLoader<T> {
    fn init() -> Self {
        InMemoryLoader { storage : HashMap::new() }
    }

    fn save(&mut self, path : &String, data : T) {
        self.storage.insert(path.clone(), data);
    }

    fn load(& self, name : &String) -> Option<&T> {
        self.storage.get(name)
    }

    fn len(&self) -> usize {
        self.storage.len()
    }
}