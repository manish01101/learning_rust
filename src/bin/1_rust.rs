use chrono::{Local, Utc};
use std::{collections::HashMap, fs};

fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    // option
    let my_str = String::from("manish");
    match find_first_a(my_str) {
        Some(index) => println!("a is at index: {}", index),
        None => println!("a is not found"),
    }

    // result
    let file_result = fs::read_to_string("a.txt");
    match file_result {
        Ok(file_content) => {
            println!("file read successfully: {:?}", file_content);
        }
        Err(error) => {
            println!("failed to read file: {:?}", error);
        }
    }

    // external crate
    let now = Utc::now();
    println!("current date and time in Utc: {}", now);
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("formatted date and time: {}", formatted);
    let local = Local::now();
    println!("current date and time in local: {}", local);


    // collections
    // vectors
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:#?}", vec);
    vec.remove(1);
    println!("{:#?}", vec);
    println!("length is: {:#?}", vec.len());
    // Initialising using rust macros
    let v = vec![1, 6, 3];
    println!("{:#?}", v);

    //hashmaps
    let mut users: HashMap<String, i32> = HashMap::new();
    users.insert(String::from("manish"), 21);
    let user1 = users.get("manish");
    let user2 = users.get("man");
    println!("{:?}", user1);
    println!("{:?}", user2);
    println!("{}", user1.unwrap());
    // println!("{}", user2.unwrap());
}
