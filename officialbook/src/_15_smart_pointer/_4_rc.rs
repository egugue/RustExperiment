use std::rc::Rc;

pub fn main() {
    utils::println_file_name!();
    rc_can_be_connected_with_others();
    prints_number_of_pointers()
}

enum InvalidMultiConnectedList {
    Cons(i32, Box<InvalidMultiConnectedList>),
    Nil,
}

fn cons_cannot_be_connected_with_others() {
    // use self::InvalidMultiConnectedList::{Cons, Nil};
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

/// Actually a.clone() can be used. But Rust's convention is to use Rc::clone().
/// Because if we find clone() method, we usually guess it does deep copy.
/// To reduce misunderstanding, we should use Rc::clone() explicitly.
///
/// https://doc.rust-lang.org/book/ch15-04-rc.html#using-rct-to-share-data
fn rc_can_be_connected_with_others() {
    use self::List::{Cons, Nil};
    utils::println_function_name!();

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}

fn prints_number_of_pointers() {
    use self::List::{Cons, Nil};
    utils::println_function_name!();

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
