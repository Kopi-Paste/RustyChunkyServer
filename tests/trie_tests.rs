use std::collections::HashMap;

use rand::{distributions::Alphanumeric, Rng};
use http_server::trie::trie::Trie;
use tracing_subscriber::fmt::init;

macro_rules! as_slice {
    ($x:expr) => {
        &$x.chars().collect::<Vec<_>>()[..];
    };
}

fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng(); // Create a random number generator
    // Generate a string with the specified length using Alphanumeric characters
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char) // Sample from Alphanumeric and cast to char
        .collect() // Collect into a String
}

#[test]
fn basic_test() {
    let mut trie = Trie::<char, i32>::init();
    trie.insert(as_slice!("aaaaaa"), 1);
    trie.insert(as_slice!("aaaaab"), 2);
    trie.insert(as_slice!("aaaaac"), 3);
    trie.insert(as_slice!("aaaaad"), 4);
    trie.insert(as_slice!("aabaad"), 5);
    println!("{:?}", trie.get_keys_for_prefix(as_slice!("aa")));
    println!("{:?}", trie.get_keys_for_prefix(as_slice!("aab")));
    println!("{:?}", trie.get_keys_for_prefix(as_slice!("aaaa")));
    println!("{:?}", trie.get_keys_for_prefix(as_slice!("aaaaa")));
}

#[test]
fn test_trie() {
    let mut trie = Trie::<char, i32>::init();
    let mut held_strings = HashMap::<String, i32>::new();
    let mut to_delete = Vec::<String>::new();

    // Insert 200 000 random strings
    for i in 0..200000 {
        let rand_str = generate_random_string(20);
        trie.insert(as_slice!(rand_str), i);
        if rand::random::<bool>() {
            to_delete.push(rand_str.clone());
        }
        held_strings.insert(rand_str, i);
    }

    // Remove cca half of them
    for string_to_delete in to_delete {
        trie.delete(as_slice!(string_to_delete));
        held_strings.remove(&string_to_delete);
    }

    // Insert another 200 000 random strings
    for i in 0..200000 {
        let rand_str = generate_random_string(20);
        trie.insert(as_slice!(rand_str), i);
        held_strings.insert(rand_str, i);
    }

    for _ in 0..5000 {
        let rand_prefix = generate_random_string(3);

        let mut mine = trie.get_keys_for_prefix(as_slice!(rand_prefix)).iter().map(|word| word.iter().collect::<String>()).collect::<Vec<_>>();
        let mut theirs = held_strings.keys().filter(|string| string.starts_with(&rand_prefix)).cloned().collect::<Vec<_>>();

        mine.sort();
        theirs.sort();

        assert_eq!(mine, theirs);

        for string in mine {
            assert_eq!(trie.get_for_string(as_slice!(string)), held_strings.get(&string));
        }
    }
}

#[test]
fn simple_rand_test() {
    let mut trie = Trie::<char, (i32, i32)>::init();

    let mut prefixes = Vec::<String>::new();

    for i in 0..100 {
        let prefix = generate_random_string(3);
        for j in 0..100 {
            let suffix = generate_random_string(15);
            trie.insert(as_slice!(format!("{prefix}{suffix}")), (i, j));
        }
        prefixes.push(prefix);
    }

    for (index, prefix) in prefixes.iter().enumerate() {
        let matched_keys = trie.get_keys_for_prefix(as_slice!(prefix));
        assert_eq!(matched_keys.len(), 100);
        for key in matched_keys {
            assert_eq!(trie.get_for_string(&key).unwrap().0, index as i32);
        }
    }
}
