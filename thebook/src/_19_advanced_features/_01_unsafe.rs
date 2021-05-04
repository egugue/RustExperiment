pub fn main() {
    // borrow_checker_in_unsafe_block();
    dereference_raw_pointer();
}

fn borrow_checker_in_unsafe_block() {
    let a = "a".to_string();
    unsafe {
        let b = a;
        // cannot compile because move occurs
        // println!("a = {}, b = {}", a, b);
    }
}

fn dereference_raw_pointer() {
    // we don't need unsafe keyword if creating a raw pointer
    let mut num = 5;
    let immutable = &num as *const i32;
    let mutable = &mut num as *mut i32;
    println!(
        "immutable raw pointer location = {:?}, mutable raw pointer location = {:?}",
        immutable, mutable
    );
    unsafe {
        // but need if dereference a raw pointer
        println!(
            "immutable raw pointer value = {:?}, mutable raw pointer value  = {:?}", *immutable, *mutable
        );
    }

    let immutable: *const i32 = 1 as *const i32;
    let mutable: *mut i32 = 2 as *mut i32;
    println!("immutable = {:?}, mutable = {:?}", immutable, mutable);
    // unsafe {
    // the below code will error with this message
    // Process finished with exit code 139 (interrupted by signal 11: SIGSEGV)
    // println!("immutable = {:?}, mutable = {:?}", *immutable, *mutable);
    // }

    // raw pointers are allowed to ignore the borrowing rules
    let mut num = 5;
    let immutable = &num as *const i32;
    let mutable = &mut num as *mut i32;
    unsafe {
        *mutable = 3;
        // will print 3
        println!(
            "immutable raw pointer value = {:?}, mutable raw pointer value  = {:?}",
            *immutable, *mutable
        );
    }
}
