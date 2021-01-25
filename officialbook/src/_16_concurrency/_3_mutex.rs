use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

pub fn main() {
    utils::println_file_name!();
    mutex_on_single_thread();
    // mutex_on_multiple_thread_with_rc();
    mutex_on_multiple_thread_with_arc();
    try_to_cause_deadlock();
}

fn mutex_on_single_thread() {
    utils::println_function_name!();

    let m = Mutex::new(5);
    assert_eq!(*m.lock().unwrap(), 5);

    {
        let mut num: MutexGuard<i32> = m.lock().unwrap();
        // MutexGuard implements Deref and DerefMut.
        *num += 1;
    }

    assert_eq!(*m.lock().unwrap(), 6);
}

fn mutex_on_multiple_thread_with_rc() {
    utils::println_function_name!();

    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            // This cannot compile.
            // Rc cannot be used between thread because it doesn't implement Send trait.
            // let mut num = counter.lock().unwrap();
            // *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn mutex_on_multiple_thread_with_arc() {
    utils::println_function_name!();

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

/// this is code to cause deadlock which is not listed in the official doc.
/// referenced by the [`Mutex`] official API doc.
fn try_to_cause_deadlock() {
    utils::println_function_name!();

    let shared_counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::with_capacity(10);
    for _ in 0..10 {
        let counter = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    let mut num: MutexGuard<i32> = shared_counter.lock().unwrap();
    *num = 10;
    // If the below drop code is not executed, the MutexGuard named `num` does not drop until this function ends.
    // This means the main thread keeps a mutex lock until this function ends.
    // But `join()` will be executed just below and the associated thread will try to acquire another mutex lock.
    // This situation will cause a deadlock.
    drop(num);

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *shared_counter.lock().unwrap());
}
