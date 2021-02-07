use futures::executor::block_on;
use futures::Future;
use std::thread;
use std::time::Duration;

pub fn main() {
    utils::println_file_name!();
    block_on(async_block());
    async_block_in_sync_fn();
    future_is_lazy();
    block_on(future_is_lazy());
}

async fn async_fn() -> u8 {
    println!("async fn");
    5
}

fn async_block() -> impl Future<Output = u8> {
    async {
        println!("async block");
        let x = async_fn().await;
        x + 5
    }
}

/// `async` block is allowed inside `async` function
fn async_block_in_sync_fn() {
    utils::println_function_name!();
    // async {
    //     println!("async block");
    // }

    // async {
    //     println!("async block");
    // }
    // .await;

    // this can compile but Rust warns with this message.
    // > warning: unused implementer of `futures::Future` that must be used
    async {
        println!("async block");
    };
}

async fn future_is_lazy() {
    utils::println_function_name!();

    async fn one() {
        println!("one");
    }
    fn two() -> impl Future<Output = ()> {
        async { println!("two") }
    }

    // Future is lazy so Future is not executed unless we use .await (or poll)
    // one();
    // two();
    // thread::sleep(Duration::from_secs(1)); // even if using sleep

    // prints one and two
    one().await;
    two().await;
}
