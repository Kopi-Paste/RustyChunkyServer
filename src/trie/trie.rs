use std::hash::Hash;

use super::node::Node;

/// Prefix tree or "trie"
/// Acts like a map (key-value structure) and also enables to obtain all keys with given prefix
/// Keys are expected to be slices, K is the inner type of a slice to use
/// V is the value type
pub struct Trie<K, V> where K : Copy + Eq + Hash {
    /// A root of the prefix tree
    root : Node<K, V>,
}


impl<K, V> Trie<K, V> where K : Copy + Eq + Hash {
    /// Initializes empty trie
    pub fn init() -> Self {
        Trie { root : Node::<K, V>::new() }
    }

    /// Inserts a value with given key
    pub fn insert(&mut self, key : &[K], value : V) {
        key.iter().fold(&mut self.root, |node, letter| node.get_or_init(letter)).set_data(value);
    }

    /// Returns a node for given key
    fn get_node_for_string(&self, key : &[K]) -> Option<&Node<K, V>> {
        key.iter().try_fold(&self.root, |current_node, letter| current_node.get_opt(letter))
    }

    /// Returns a mutable node for given key
    fn get_mut_node_for_string(&mut self, key : &[K]) -> Option<&mut Node<K, V>> {
        key.iter().try_fold(&mut self.root, |current_node, letter| current_node.get_opt_mut(letter))
    }

    /// Returns a value for given key
    pub fn get_for_string(&self, key : &[K]) -> Option<&V> {
        self.get_node_for_string(key).and_then(|node| node.get_data())
    }

    /// Returns a mutable value for given key
    pub fn get_mut_for_string(&mut self, key : &[K]) -> Option<&mut V> {
        self.get_mut_node_for_string(key).and_then(|node| node.get_mut_data())
    }

    /// Returns whether given key exists in the trie
    pub fn contains(&self, key : &[K]) -> bool {
        self.get_for_string(key).is_some()
    }

    /// Deletes a value associated within given key, returns true if there was something deleted, else false
    pub fn delete(&mut self, key : &[K]) -> bool {
        let mut return_value = false;
        
        fn delete_rec<K : Eq + Hash + Copy, V>(node : &mut Node<K, V>, deleted : &[K], return_value : &mut bool) -> bool {
            if deleted.is_empty() {
                node.remove_data();
                *return_value = true;
                return true;
            }
            let letter = deleted.first().unwrap();
            if let Some(next_node) = node.get_opt_mut(letter) {
                if delete_rec(next_node, &deleted[1..], return_value){
                    return node.remove_if_possible(letter);
                }
            }
            return false;
        }

        delete_rec(&mut self.root, key, &mut return_value);
        return_value
    }


    /// Returns all keys in trie starting with a given prefix
    pub fn get_keys_for_prefix(&self, prefix : &[K]) -> Vec<Vec<K>> {
        fn get_strings_rec<K : Eq + Hash + Copy, V>(node : &Node<K, V>, prefix : Vec<K>) -> Vec<Vec<K>> {
            let mut returned_values = Vec::new();


            node.get_children().for_each(|(letter, child)| {
                let mut prefix_for_child = prefix.clone();
                prefix_for_child.push(*letter);
                returned_values.extend(get_strings_rec(child, prefix_for_child))
            });

            if node.is_end() {
                returned_values.push(prefix);
            }

            returned_values
        }

        if let Some(node) = self.get_node_for_string(prefix) {
            return get_strings_rec(node, prefix.to_vec());
        }
        else {
            return Vec::new();
        }
    }
}