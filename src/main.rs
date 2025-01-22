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
fn generate_random_key() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(KEY_LENGTH)
        .map(char::from)
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

// Node structure to hold key-value pairs
struct Node<K, V> {
    key: K,
    value: V,
    next: Option<Box<Node<K, V>>>,
}

// LinkedList structure
struct LinkedList<K, V> {
    head: Option<Box<Node<K, V>>>,
}

/// A linked list implementation where each node contains a key-value pair.
///
/// # Type Parameters
/// - `K`: The type of the key (10-digit uppercase random alphanumeric String, e.g., "A7B2C9D4E8")
/// - `V`: The type of the value (any type implementing Display trait)
///
/// # Example
/// ```
/// let mut list = LinkedList::<String, String>::new();
/// list.insert(generate_random_key(), String::from("John"));
/// ```
impl<K: Display, V: Display> LinkedList<K, V> {
    /// Creates a new, empty `LinkedList`.
    ///
    /// # Returns
    /// A new `LinkedList` instance with no elements.
    fn new() -> Self {
        LinkedList { head: None }
    }

    /// Inserts a new key-value pair into the linked list.
    ///
    /// # Arguments
    /// - `key`: The key to insert (10-digit uppercase random alphanumeric).
    /// - `value`: The value to insert.
    ///
    /// # Example
    /// ```
    /// list.insert(generate_random_key(), String::from("John"));
    /// ```
    fn insert(&mut self, key: K, value: V) {
        let new_node = Box::new(Node {
            key,
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    /// Displays all the key-value pairs in the linked list.
    ///
    /// This method prints each key-value pair in the list to the standard output.
    /// Keys will be displayed in 10-digit uppercase alphanumeric format.
    fn display(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("Key: {}, Value: {}", node.key, node.value);
            current = &node.next;
        }
    }
}

fn main() {
    let mut list = LinkedList::<String, String>::new();
    
    // Insert some key-value pairs with random 10-digit uppercase keys
    list.insert(generate_random_key(), String::from("John"));
    list.insert(generate_random_key(), String::from("30"));
    list.insert(generate_random_key(), String::from("New York"));
    
    // Display the list
    println!("Linked List contents:");
    list.display();
}