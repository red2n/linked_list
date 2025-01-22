use std::fmt::Display;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

const KEY_LENGTH: usize = 10;

/// Generates a random 10-digit uppercase alphanumeric string.
///
/// # Returns
/// A String containing 10 random uppercase alphanumeric characters
///
/// # Example
/// ```
/// let key = generate_random_key(); // e.g. "A7B2C9D4E8"
/// ```
pub fn generate_random_key() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(KEY_LENGTH)
        .map(char::from)
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

// Node structure to hold key-value pairs
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    pub next: Option<Box<Node<K, V>>>,
}

// LinkedList structure
pub struct LinkedList<K, V> {
    head: Option<Box<Node<K, V>>>,
}

impl<K: Display, V: Display> LinkedList<K, V> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let new_node = Box::new(Node {
            key,
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn display(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("Key: {}, Value: {}", node.key, node.value);
            current = &node.next;
        }
    }
}