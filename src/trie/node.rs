use std::collections::HashMap;
use std::iter::Iterator;
use std::hash::Hash;

pub struct Node<K, V> where K : Eq + Hash + Copy {
    children : HashMap<K, Node<K, V>>,
    payload : Option<V>
}

impl<K, V> Node<K, V> where K : Eq + Hash + Copy {
    pub fn new() -> Self {
         Node::<K, V> { children : HashMap::new(), payload : None }
    }

    pub fn is_end(&self) -> bool {
        self.payload.is_some()
    }

    pub fn set_data(&mut self, data : V) {
        self.payload = Some(data);
    }

    pub fn remove_data(&mut self) {
        self.payload = None;
    }

    pub fn remove_if_possible(&mut self, letter : &K) -> bool {
        if let Some(son) = self.children.get(letter) {
            if son.children.is_empty() {
                self.children.remove(letter);
                return true;
            }
        }
        false
    }
    pub fn get_data(&self) -> Option<&V> {
        self.payload.as_ref()
    }

    pub fn get_mut_data(&mut self) -> Option<&mut V> {
        self.payload.as_mut()
    }

    pub fn get_or_init(&mut self, letter : &K) -> &mut Node<K, V> {
        self.children.entry(*letter).or_insert_with(Node::new)
    }

    pub fn get_opt(&self, letter : &K) -> Option<&Node<K, V>> {
        self.children.get(letter)
    }

    pub fn get_opt_mut(&mut self, letter : &K) -> Option<&mut Node<K, V>> {
        self.children.get_mut(letter)
    }

    pub fn get_number_of_sons(&self) -> usize {
        self.children.len()
    }

    pub fn get_children(&self) -> impl Iterator<Item = (&K, &Node<K, V>)> {
        self.children.iter()
    }
}
