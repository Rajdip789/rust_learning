//Write a funciton that takes a vector of tuples (tuple containing a key and a value) and returns a hashmap build from the tuples of the vector

use std::collections::HashMap;

fn get_key_value_map(pairs: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut map = HashMap::new();

    for (key, value) in pairs {
        map.insert(key, value);
    }

    map
}

pub fn run() {
    let pairs = vec![
        (String::from("Rajdip"), 1), 
        (String::from("Sandip"), 2)
    ];

    let grouped_pairs = get_key_value_map(pairs);

    for (key, value) in grouped_pairs {
        println!("Key: {} Value: {}", key, value);
    }
}