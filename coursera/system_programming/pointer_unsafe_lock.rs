fn main() {
    let r: *const i32 = &10;
    unsafe {
        println!("value: {}", *r); // dereferencing
    }

    let mut num = 5;
    let a = &mut num as *mut i32;
    // let dereferenced_value = *a; // get err: dereference of raw pointer is unsafe and requires unsafe function or block
    let dereferenced_value = unsafe { *a };
    println!("dereferenced value: {}", dereferenced_value);
}
