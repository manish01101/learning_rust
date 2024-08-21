fn main() {
    let ss = "manish"; // string literal-immutable
    println!("{ss}");
    let mut s = String::from("hello"); // string obj
    s.push_str(" there");
    println!("{s}");

    // copy trait => for stack variable
    // drop trait => free memory

    {
        let s1 = give_ownership();
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
    } // s3 goes out of scope and is dropped, s2 was moved, s1 goes out of scope and is dropped

    // REFERENCES
    let s = String::from("value");
    let len = cal_len(&s);
    println!("len is {len}");
    let mut ss = String::from("hi there");
    calculate_len(&mut ss);
    println!("ss is {ss}");

    // DANGLING REFERENCES
    // let ref = dangle();

    // STRING SLICING
    let s = String::from("hello world");

    let hello = &s[0..5]; // or &s[..5]; // both are equal
    let world = &s[6..11];
    let last = &s[3..]; // from 3 to last
    let all = &s[..]; // all ele is included
    println!("hello = {hello}, world = {world}");
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string // some_string is returned and moves out to the calling fn
}
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling fn
}
fn cal_len(s: &String) -> usize {
    s.len()
}
fn calculate_len(s: &mut String) {
    s.push_str(", hello");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// SLICE TYPE
fn first_word(s: &String) -> usize {
    // return a byte index val
    let bytes = s.as_bytes(); // convert string to an arr of bytes
    for (i, &item) in bytes.iter().enumerate() {
        // iter creates an iterator over the arr of bytes
        if item == b' ' {
            // byte literal
            return i;
        }
    }
    s.len()
}
fn first_word_final(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
