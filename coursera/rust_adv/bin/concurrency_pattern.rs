use std::thread;
use threadpool::ThreadPool;
// threadpool

fn main() {
    let pool = ThreadPool::new(4); // create a threadpool with 4 thread;
    for _ in 0..8 {
        pool.execute(|| {
            println!("Task is processed by a thread");
        })
    }
    pool.join();

    // running in parallel
    let handles: Vec<_> = (0..4)
        .map(|i| {
            thread::spawn(move || {
                println!("Task {} is running in parallel", i);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
