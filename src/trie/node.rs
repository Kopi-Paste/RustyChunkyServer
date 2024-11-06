use std::collections::HashMap;
use std::iter::Iterator;
use std::hash::Hash;

/// Inner struct of one node in the trie
pub struct Node<K, V> where K : Eq + Hash + Copy {
    /// Hash map mapping a letter to a child node
    children : HashMap<K, Node<K, V>>,
    /// Optional value associated with the given key ending in this node
    payload : Option<V>
}

impl<K, V> Node<K, V> where K : Eq + Hash + Copy {
    /// Initializes new node
    pub fn new() -> Self {
        Node::<K, V> { children : HashMap::new(), payload : None }
    }

    /// Returns whether this node contains a value (is end of a key)
    pub fn is_end(&self) -> bool {
        self.payload.is_some()
    }

    /// Sets held data in this node
    pub fn set_data(&mut self, data : V) {
        self.payload = Some(data);
    }

    /// Removes held data in this node
    pub fn remove_data(&mut self) {
        self.payload = None;
    }

    /// Removes a son associated with given letter if it does not have any children, returns true if removed, else false
    pub fn remove_if_possible(&mut self, letter : &K) -> bool {
        if let Some(son) = self.children.get(letter) {
            if son.children.is_empty() {
                self.children.remove(letter);
                return true;
            }
        }
        false
    }

    /// Returns data from this node
    pub fn get_data(&self) -> Option<&V> {
        self.payload.as_ref()
    }

    /// Returns mutable data from this node
    pub fn get_mut_data(&mut self) -> Option<&mut V> {
        self.payload.as_mut()
    }

    /// Returns a node associated with given letter, if it does not exist, new node is created
    pub fn get_or_init(&mut self, letter : &K) -> &mut Node<K, V> {
        self.children.entry(*letter).or_insert_with(Node::new)
    }

    /// Returns a node associated with given letter
    pub fn get_opt(&self, letter : &K) -> Option<&Node<K, V>> {
        self.children.get(letter)
    }

    /// Returns a mutable node associated with given letter
    pub fn get_opt_mut(&mut self, letter : &K) -> Option<&mut Node<K, V>> {
        self.children.get_mut(letter)
    }

    /// Returns the children of this node
    pub fn get_children(&self) -> impl Iterator<Item = (&K, &Node<K, V>)> {
        self.children.iter()
    }
}
