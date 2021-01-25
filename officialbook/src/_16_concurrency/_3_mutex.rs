use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

pub fn main() {
    utils::println_file_name!();
    mutex_on_single_thread();
    // mutex_on_multiple_thread_with_rc();
    mutex_on_multiple_thread_with_arc();
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
