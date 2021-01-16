
pub fn main() {
    utils::println_file_name!();
    box_stores_on_heap();
}

/// https://doc.rust-lang.org/book/ch15-01-box.html#using-a-boxt-to-store-data-on-the-heap
fn box_stores_on_heap() {
    utils::println_function_name!();
    // storing a simple data such as i32 is not appropriate, this code is just an example.
    let b = Box::new(5);
    println!("b = {}", b);
}
