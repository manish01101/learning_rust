// Message Passing between threads using Channels.
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel:
    // tx = transmitter (sender)
    // rx = receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // Send the value down the channel.
            // send() returns a Result, so unwrap() handles potential errors.
            tx.send(val).unwrap();

            // Sleep for 1 second to simulate work
            thread::sleep(Duration::from_secs(1));
        }
        // When this closure ends, 'tx' is dropped, which closes the channel.
    });

    // The main thread treats 'rx' like an iterator.
    // It waits here for values. It stops when the channel closes.
    for received in rx {
        println!("Got: {}", received);
    }
}
