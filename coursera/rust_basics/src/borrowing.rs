/*
borrow check enforce rust's borrowing rules during compilation

Borrowing With Smart Pointers
Borrowing with smart pointers in Rust can provide additional capabilities and safety guarantees when managing ownership and references to data. Smart pointers such as Box, Rc, and RefCell (and their thread-safe equivalents Arc and Mutex) are commonly used to enable borrowing patterns that go beyond the simple ownership model.  

Rust prevents cyclic references in smart pointers primarily through its ownership and borrowing system, along with specific smart pointer types such as Rc<T> and Weak<T>. However cyclic references are majorly prevented by using Weak references.

1. Box<T>
Box<T> is a heap-allocated smart pointer that provides ownership of data. It is useful for scenarios where you need to ensure data is heap-allocated, such as for recursive data structures.
fn main() {
    let b = Box::new(5);
    let b_ref: &i32 = &b;
    println!("b: {}, b_ref: {}", b, b_ref);
}

2. Rc<T> and RefCell<T>
Rc<T> is a reference-counted smart pointer that enables multiple ownership in single-threaded contexts. RefCell<T> allows for interior mutability, meaning you can mutate data even if it is immutable from the outside.
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    let value = Rc::new(RefCell::new(5));
    let value1 = Rc::clone(&value);
    let value2 = Rc::clone(&value);
    *value1.borrow_mut() += 1;
    *value2.borrow_mut() += 1;
    println!("Value: {}", value.borrow());
}

3. Arc<T> and Mutex<T>
For multi-threaded contexts, Arc<T> (atomic reference counting) and Mutex<T> (mutual exclusion) are used to ensure thread-safe shared ownership and mutation.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let value = Arc::new(Mutex::new(5));
    
    let handles: Vec<_> = (0..10).map(|_| {
        let value = Arc::clone(&value);
        thread::spawn(move || {
            let mut num = value.lock().unwrap();
            *num += 1;
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Value: {}", *value.lock().unwrap());
}

*/

fn main() {
    let mut a = 2;
    let b = &mut a; // mutable reference or borrowing
    *b = 10;
    println!("a is {}", a);

    let mut c = 4;
    let mut d = &c; //  immutable reference or borrowing
    *d = 3;
    println!("c is {}", c);
}
