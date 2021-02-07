#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use futures;
use futures::executor::block_on;
use std::thread;
use std::time::Duration;

pub fn main() {
    utils::println_file_name!();
    // execute_two_tasks();
    block_on(execute_two_tasks2());
}

fn execute_two_tasks() {
    let thread1 = thread::spawn(|| {
        println!("thread1 starts running");
        thread::sleep(Duration::from_millis(300));
        println!("thread1 finished");
    });
    let thread2 = thread::spawn(|| {
        println!("thread2 starts running");
        thread::sleep(Duration::from_millis(300));
        println!("thread2 finished");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}

async fn execute_two_tasks2() {
    async fn execute_task(id: u8) {
        println!("function starts running: {}", id);
        async_std::task::sleep(Duration::from_millis(500)).await;
        println!("function finished running: {}", id);
    }

    let future1 = execute_task(1);
    let future2 = execute_task(2);
    futures::join!(future1, future2);
}
