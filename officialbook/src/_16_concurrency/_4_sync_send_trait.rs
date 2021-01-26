use std::cell::Cell;
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

pub fn main() {
    utils::println_file_name!();
    send_trait();
    sync_trait();
}

const STATIC_POINTER: &'static str = "static pointer";

fn send_trait() {
    utils::println_function_name!();

    let primitive = 1;
    let string: String = String::from("str");
    let pointer: &i32 = &primitive;
    let rc: Rc<i32> = Rc::new(1);

    let handle = thread::spawn(move || {
        println!("primitive types are Send: {}", primitive);
        println!("almost every Rust type is Send: {}", string);

        println!("a raw pointer is not Send");
        // pointer;
        println!("a static pointer is Send: {}", STATIC_POINTER);

        println!("Rc type is not Send");
        // rc.borrow();
    });

    handle.join().unwrap();
}

/// see https://doc.rust-lang.org/std/marker/trait.Sync.html for the details.
fn sync_trait() {
    utils::println_function_name!();

    let primitive = 1;
    check_sync(primitive);

    let pointer: &i32 = &1;
    check_sync(pointer);

    check_sync(STATIC_POINTER);

    let string: String = String::from("str");
    check_sync(string);

    let mutex: Mutex<i32> = Mutex::new(1);
    check_sync(mutex);

    let rc: Rc<i32> = Rc::new(1);
    // check_sync(rc);

    // > Types that are not Sync are those that have "interior mutability" in a non-thread-safe form, such as Cell and RefCell.
    let cell: Cell<i32> = Cell::new(1);
    // check_sync(cell);
}

fn check_sync<T>(v: T)
where
    T: Sync,
{
}
