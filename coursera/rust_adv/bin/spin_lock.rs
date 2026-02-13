use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc; // Import Arc
use std::thread;

struct Spinlock {
    lock: AtomicBool,
}

impl Spinlock {
    fn new() -> Self {
        Spinlock {
            lock: AtomicBool::new(false),
        }
    }

    fn lock(&self) {
        // compare_exchange takes (current, new, success_order, failure_order).
        // We loop while it returns Err (meaning the lock was already true/busy).
        while self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // Optional: std::hint::spin_loop(); 
            // Adding a spin_loop hint is good practice here to reduce CPU usage.
        }
    }

    fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}

fn main() {
    // 1. Wrap the Spinlock in an Arc to put it on the heap
    let spinlock = Arc::new(Spinlock::new());

    // 2. Clone the Arc. This increases the reference count.
    // We give this clone to the thread.
    let spinlock_clone = Arc::clone(&spinlock);

    let handle = thread::spawn(move || {
        // The thread now owns 'spinlock_clone', so it's safe!
        spinlock_clone.lock();
        println!("Thread acquired lock");
        spinlock_clone.unlock();
    });

    handle.join().unwrap();
}

