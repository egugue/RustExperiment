use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    utils::println_file_name!();
    channel();
    // ownership_can_prevent_unexpected_error();
    multiple_values();
}

fn channel() {
    utils::println_function_name!();

    // mpsc means Multi-producer Single-Consumer
    // tx and rx mean transmitter and receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = "hi".to_string();
        thread::sleep(Duration::from_secs(1));
        tx.send(val).unwrap(); //should handle an error properly in a real application.
    });

    assert_eq!(rx.try_recv().is_err(), true);
    println!("before receive");
    let result = rx.recv();
    println!("after receive. value = {}", result.unwrap());
}

/// Rust's ownership system can prevent an unexpected error which is common in multi-thread programming.
fn ownership_can_prevent_unexpected_error() {
    utils::println_function_name!();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = "hi".to_string();
        tx.send(val);

        // cannot compile because `val` was moved when invoking send method.
        // println!("val is {} on worker thread", val);
    });

    let received = rx.recv().unwrap();
    println!("val is {} on main thread", received);
}

fn multiple_values() {
    utils::println_function_name!();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let values = vec!["a", "b", "c"];
        for v in values {
            println!("sending {}", v);
            tx.send(v);
            thread::sleep(Duration::from_millis(300));
        }
        println!("wait 1 second");
        thread::sleep(Duration::from_secs(1));
        println!("worker thread has been executed.");
    });

    for v in rx {
        println!("received {}", v);
    }
    println!("all values are received.");
}
