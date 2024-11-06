use std::hash::Hash;

use super::node::Node;

pub struct Trie<K, V> where K : Copy + Eq + Hash {
    root : Node<K, V>,
}


impl<K, V> Trie<K, V> where K : Copy + Eq + Hash {
    pub fn init() -> Self {
        Trie { root : Node::<K, V>::new() }
    }

    pub fn insert(&mut self, key : &[K], value : V) {
        key.iter().fold(&mut self.root, |node, letter| node.get_or_init(letter)).set_data(value);
    }

    fn get_node_for_string(&self, key : &[K]) -> Option<&Node<K, V>> {
        key.iter().try_fold(&self.root, |current_node, letter| current_node.get_opt(letter))
    }

    fn get_mut_node_for_string(&mut self, key : &[K]) -> Option<&mut Node<K, V>> {
        key.iter().try_fold(&mut self.root, |current_node, letter| current_node.get_opt_mut(letter))
    }

    pub fn get_for_string(&self, key : &[K]) -> Option<&V> {
        self.get_node_for_string(key).and_then(|node| node.get_data())
    }

    pub fn get_mut_for_string(&mut self, key : &[K]) -> Option<&mut V> {
        self.get_mut_node_for_string(key).and_then(|node| node.get_mut_data())
    }

    pub fn contains(&self, key : &[K]) -> bool {
        self.get_for_string(key).is_some()
    }

    pub fn delete(&mut self, key : &[K]) {
        fn delete_rec<K : Eq + Hash + Copy, V>(node : &mut Node<K, V>, deleted : &[K]) -> bool {
            if deleted.is_empty() {
                node.remove_data();
                return true;
            }
            let letter = deleted.first().unwrap();
            match node.get_opt_mut(letter) {
                Some(next_node) => {
                    if delete_rec(next_node, &deleted[1..]){
                        node.remove_if_possible(letter);
                        return true;
                    }
                    return false;
                },
                None => return false
            }
        }

        delete_rec(&mut self.root, key);
    }



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