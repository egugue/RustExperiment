pub fn main() {
    utils::println_file_name!();
    box_stores_on_heap();
    recursive_type::main();
}

/// https://doc.rust-lang.org/book/ch15-01-box.html#using-a-boxt-to-store-data-on-the-heap
fn box_stores_on_heap() {
    utils::println_function_name!();
    // storing a simple data such as i32 is not appropriate, this code is just an example.
    let b = Box::new(5);
    println!("b = {}", b);
}

mod recursive_type {

    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }
    //
    // the above code cannot compile
    //
    // enum List {
    // ^^^^^^^^^ recursive type has infinite size
    //     Cons(i32, List),
    //               ---- recursive without indirection
    // help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
    // Cons(i32, Box<List>),
    //           ^^^^    ^

    use self::List::*;

    #[derive(Debug)]
    enum List {
        // Box<T> is a pointer to the data on the heap. This means Rust can know the size a Box takes up.
        Cons(i32, Box<List>),
        Nil,
    }

    pub fn main() {
        let list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    3,
                    Box::new(Nil),
                )),
            )),
        );

        println!("const list = {:?}", list);
    }
}