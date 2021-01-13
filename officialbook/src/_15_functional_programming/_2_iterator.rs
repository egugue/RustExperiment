pub fn main() {
    utils::println_file_name!();
    calling_order();
}

fn calling_order() {
    utils::println_function_name!();

    // it prints
    // x1 = 10, x2 = 11, x3 = 11.
    // x1 = 20, x2 = 21, x3 = 21.
    // x1 = 30, x2 = 31, x3 = 31.
    let vec = vec![10, 20, 30];
    vec.iter()
        .map(|x| {
            print!("x1 = {}, ", x);
            x + 1
        })
        .filter(|x| {
            print!("x2 = {}, ", x);
            true
        })
        .for_each(|x| println!("x3 = {}.", x));
}
