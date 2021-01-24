use std::thread;
use std::time::Duration;

pub fn main() {
    utils::println_file_name!();
    create_thread_with_spawn();
}

fn create_thread_with_spawn() {
    utils::println_function_name!();

    let handle = thread::Builder::new()
        .name("worker".to_string())
        .spawn(|| {
            for i in 0..20 {
                println!(
                    "number {} on {}",
                    i,
                    thread::current().name().unwrap_or_default(),
                );
                thread::sleep(Duration::from_micros(1));
            }
        })
        .expect("failed");

    for i in 0..5 {
        println!(
            "number {} on {}",
            i,
            thread::current().name().unwrap_or_default(),
        );
        thread::sleep(Duration::from_micros(1));
    }

    // wait for the worker thread to complete.
    handle.join().expect("couldn't join on the worker thread");
}
