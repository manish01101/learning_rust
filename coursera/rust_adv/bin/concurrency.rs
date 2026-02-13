use std::sync::Mutex;
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi num {} from spawned thread", i);
        }
    });
    for i in 1..=5 {
        println!("hi num {} from main thread", i);
    }
    handle.join().unwrap();

    let counter = Mutex::new(0);
    {
        let mut num = counter.lock().unwrap();
        *num += 6;
    }
    println!("counter {:?}", counter);
}
