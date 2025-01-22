mod linkedlist;
use linkedlist::{LinkedList, generate_random_key};

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