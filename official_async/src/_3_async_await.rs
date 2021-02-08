use futures::executor::block_on;
use futures::Future;

pub fn main() {
    utils::println_file_name!();
    block_on(async_block());
    async_block_in_sync_fn();
    block_on(future_is_lazy());
    block_on(reference_in_async_block());
    block_on(const_reference());
    block_on(async_block_and_move());
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

async fn borrow_x(x: &u8) -> u8 {
    *x
}

async fn borrow_x_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}

fn reference_in_async_block() -> impl Future<Output = u8> {
    utils::println_function_name!();

    // let x = 1;
    // borrow_x(&x); // ERROR: `x` does not live long enough

    async {
        let x: u8 = 1;
        borrow_x(&x).await
    }
}

const CONST_X: u8 = 1;

fn const_reference() -> impl Future<Output = u8> {
    utils::println_function_name!();
    borrow_x(&CONST_X)
}

async fn async_block_and_move() {
    println!("------ async_block_and_move -----");

    let keyword = "keyword".to_string();
    let future1 = async {
        println!("first print: {}", keyword);
    };
    let future2 = async {
        println!("second print {}", keyword);
    };
    futures::join!(future1, future2);

    let move_future1 = async move {
        println!("move first print: {}", keyword);
    };

    // this cannot compile because `keyword` moved to use move_future1
    // let move_future2 = async move {
    //     println!("move second print: {}", keyword);
    // };

    // this cannot compile because `keyword` moved to use move_future1
    // futures::join!(future1, move_future1);

    // futures::join!(move_future1);
}
