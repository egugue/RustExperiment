pub fn main() {
    utils::println_file_name!();
    dereference_operator();
    my_smart_pointer::main();
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
    use std::ops::{Deref, DerefMut};

    /// unlike Box<T>, MyBox<T> does not store data on the heap.
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> Self {
            Self(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    pub fn main() {
        like_reference();
        implicit_deref_coercions();
        deref_coercion_rules();
    }

    fn like_reference() {
        utils::println_function_name!();
        let x = 5;
        let y = MyBox::new(x);

        // MyBox can be dereferenced because MyBox implements Deref.
        assert_eq!(x, *y);

        // The above code is equivalent to this.
        assert_eq!(x, *(y.deref()));

        // deref returns a reference because we don't want take ownership of the data.
        let deref: &i32 = y.deref();
    }

    fn implicit_deref_coercions() {
        fn println_str(str: &str) {
            println!("this is {}", str)
        }

        let s = MyBox::new("mybox".to_string());

        // deref coercion makes &MyBox<String> to be treated as &str.
        // &MyBox<String> --deref--> &String --deref--> &str
        println_str(&s);
        println_str(s.deref().deref());

        // if Rus doesn't have deref coercion feature, the below must be passed in.
        let s: &str = &(*s)[..];
    }

    struct MutBox<T>(T);

    impl<T> Deref for MutBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for MutBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    // https://doc.rust-lang.org/book/ch15-02-deref.html#how-deref-coercion-interacts-with-mutability
    fn deref_coercion_rules() {
        // MyBox
        {
            let m: MyBox<String> = MyBox::new("a".to_string());
            let s: &String = &m;

            // cannot compile because MyBox does not implement DerefMut
            let mut m: MyBox<String> = MyBox::new("a".to_string());
            // let s: &mut String = &mut m;

            let mut m: MyBox<String> = MyBox::new("a".to_string());
            let s: &String = &m;
        }

        // MutBox
        {
            let m: MutBox<String> = MutBox("a".to_string());
            let s: &String = &m;

            // can compile because MutBox implements DerefMut
            let mut m: MutBox<String> = MutBox("a".to_string());
            let s: &mut String = &mut m;

            let mut m: MyBox<String> = MyBox::new("a".to_string());
            let s: &String = &m;

            // cannot compile. The official books says
            // > Rust will also coerce a mutable reference to an immutable one.
            // > But the reverse is not possible: immutable references will never coerce to mutable references.
            let m: MutBox<String> = MutBox("a".to_string());
            // let s: &mut String = &mut m;
        }
    }
}
