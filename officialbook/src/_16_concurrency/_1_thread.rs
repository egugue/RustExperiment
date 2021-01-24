use std::thread;
use std::time::Duration;

pub fn main() {
    utils::println_file_name!();
    // create_thread_with_spawn();
    use_move_keyword_to_capture_outer_values();
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

fn use_move_keyword_to_capture_outer_values() {
    utils::println_function_name!();

    let values = vec![1, 2, 3];

    // To capture `values`, we should use move keyword to capture it.
    let handle = thread::spawn(move || {
        println!("values = {:?} on worker thread", values);
    });

    // This cannot compile because `values` has already moved into the above closure.
    // println!("values = {:?} on main thread", values);

    handle.join().unwrap();
}
