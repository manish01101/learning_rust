use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);

    let handle1 = thread::spawn(move || {
        let mut num = counter1.lock().unwrap();
        *num += 1;
    });

    let handle2 = thread::spawn(move || {
        let mut num = counter2.lock().unwrap();
        *num += 1;
    });
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("counter: {:?}", counter.lock().unwrap());

    // combining arc mutes
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for _ in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut vec = data.lock().unwrap();
            vec.push(4);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("result {:?}", *data.lock().unwrap());
}
