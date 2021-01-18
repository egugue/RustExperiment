/// https://doc.rust-lang.org/book/ch15-03-drop.html
pub fn main() {
    utils::println_file_name!();
    check_drop();
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("about to drop: {:?}", self)
    }
}

/// drop method is invoked when a variable goes out of scope.
/// variables is dropped in the reverse order (guess because of the stack...)
fn check_drop() {
    utils::println_function_name!();
    let a = CustomSmartPointer { data: "1".to_string() };
    let b = CustomSmartPointer { data: "2".to_string() };
    let c = CustomSmartPointer { data: "3".to_string() };
    println!("CustomSmartPointers are created");
    // and prints the below.
    // about to drop: CustomSmartPointer { data: "3" }
    // about to drop: CustomSmartPointer { data: "2" }
    // about to drop: CustomSmartPointer { data: "1" }
}

