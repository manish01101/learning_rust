/*
string slice : &str
- borrowed reference, does not own the data
- immutable
- reference to data managed elsewhere

owned string: String
- owns the data, heap allocated
- mutable
- manages its own data, including allocation and deallocation
*/

fn main() {
    let str_slice = "hello manish";
    // &str to String
    let own_str = str_slice.to_string();

    // String to &str
    let new_str_slice = own_str.as_str();

    println!("string slice: {}", str_slice);
    println!("owned str: {}", own_str);
    println!("new str slice: {}", new_str_slice);
}