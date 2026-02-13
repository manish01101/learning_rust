use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
// let atomic_counter = AtomicUsize::new(0);


fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // atomically increment the counter
            counter.fetch_add(1, Ordering::SeqCst); // sequentially constant
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
