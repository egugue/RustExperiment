use std::rc::Rc;
use std::thread;

pub fn main() {
    utils::println_file_name!();
    send_trait();
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
        println!("a tatic pointer is Send: {}", STATIC_POINTER);

        println!("Rc type is not Send");
        // rc.borrow();
    });

    handle.join().unwrap();
}
