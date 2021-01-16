pub fn main() {
    utils::println_file_name!();
    box_stores_on_heap();
    recursive_type::main();
    dereference_operator();
    my_smart_pointer::main();
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

fn dereference_operator() {
    utils::println_function_name!();
    let x: i32 = 5;
    let y: &i32 = &x;
    assert_eq!(x, *y);
    // assert_eq!(x, y); // cannot compile x(i32) and y(&i32) are different types.

    // Box can be used like a reference.
    let x: i32 = 5;
    let y: Box<i32> = Box::new(x);
    assert_eq!(x, *y);
}

mod my_smart_pointer {
    use std::ops::Deref;

    /// unlike Box<T>, MyBox<T> does not store data on the heap.
    struct MyBox<T>(T);

    impl <T> MyBox<T> {
        fn new(x: T) -> Self {
            Self(x)
        }
    }

    impl <T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    pub fn main() {
        utils::println_function_name!();
        let x = 5;
        let y = MyBox::new(x);

        // MyBox can be dereferenced because MyBox implements Deref.
        // assert_eq!(x, *y);

        // The above code is equivalent to this.
        assert_eq!(x, *(y.deref()));

        // deref returns a reference because we don't want take ownership of the data.
        let deref: &i32 = y.deref();
    }
}
