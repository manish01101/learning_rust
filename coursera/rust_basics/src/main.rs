use std::fs::File;
use std::io::Result;

fn main() {
    println!("Hello, world!");
    // scaler type -> int, float, bool, char
    let i = 5;
    // compound types => arr(same type), tuple(different type)
    let arr = [1, 2, 3, 4];
    let tup = (1, "two", false);
    println!("{:?}", tup);
    /*
    special types
        slices: ref to a contiguous seq. of ele in a collection and do not have ownership
        string: String and &str(string slice)
     */
    let slice = &arr[1..3];
    println!("arr_slice: {:?}", slice);

    let mut s = String::from("hi manish");
    s.push_str("how r u");
    println!("{}", s);
    let str = "hi manish";
    println!("{}", str);

    /* err handling
    Rust mainly uses Result, Option, and panic mechanisms.
    1. Recoverable Errors: Result<T, E>
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }

     */
    let file = File::open("data.txt");

    match file {
        Ok(f) => println!("File opened successfully"),
        Err(e) => println!("Failed to open file: {}", e),
    }
    /*Using ? Operator (Preferred) Propagates errors cleanly. */
    // fn open_file() -> Result<File> {
    //     let file = File::open("data.txt")?;
    //     Ok(file)
    // }

    /*2. Optional Values: Option<T> */
    fn find_number(x: i32) -> Option<i32> {
        if x > 0 {
            Some(x)
        } else {
            None
        }
    }
    match find_number(-1) {
        Some(n) => println!("Found: {}", n),
        None => println!("No value found"),
    }
    /* 3. Unrecoverable Errors: panic! */
    let v = vec![1, 2, 3];
    let i = 5;
    if i > v.len() {
        panic!("err, idx out of bound")
    }
    println!("{}", v[i]); // panic!
}


