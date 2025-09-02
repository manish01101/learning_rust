use std::fs;
enum Result<T, E> {
    Ok(T),
    Err(E),
}
enum Option<T> {
    // used for null value
    None,
    Some(T),
}

fn find_first_a(s: String) -> Option<usize> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index);
        }
    }
    return None;
}

fn main() {
    let res = fs::read_to_string("a.txt");

    match res {
        Ok(file_content) => {
            println!("file content: {}", file_content);
        }
        Err(error) => {
            println!("error throw: {}", error);
        }
    }
    println!("hi there");

    // option enum for null value
    let my_string = String::from("raman");
    match find_first_a(my_string) {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }
}
