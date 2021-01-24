use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    utils::println_file_name!();
    channel();
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
