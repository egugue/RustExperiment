pub fn main() {
    utils::println_file_name!();
    calling_order();
    iterator_demonstration();
    iter_methods();
    closure_capturing_environment();
}

/// each iterator adapter calls in the order of definition because iterators are lazy.
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

/// https://doc.rust-lang.org/book/ch13-02-iterators.html#the-iterator-trait-and-the-next-method
fn iterator_demonstration() {
    utils::println_function_name!();

    let vec = vec![10, 20, 30];
    let mut iter = vec.iter();
    assert_eq!(iter.next(), Some(&10));
    assert_eq!(iter.next(), Some(&20));
    assert_eq!(iter.next(), Some(&30));
    assert_eq!(iter.next(), None);
    assert_eq!(vec, vec![10, 20, 30])
}

// the variety of iter creation
fn iter_methods() {
    utils::println_function_name!();

    let vec = vec!["a".to_string()];
    let mut iter = vec.into_iter();
    assert_eq!(iter.next(), Some("a".to_string()));
    // cannot compile because vec moved to iter due to into_iter call
    // assert_eq!(vec, vec!["a".to_string()]);

    // define as mut to call iter_mut
    let mut vec = vec!["a".to_string()];
    let mut iter = vec.iter_mut();
    iter.next().unwrap().push('b');
    assert_eq!(vec, vec!["ab"]);
}

/// https://doc.rust-lang.org/book/ch13-02-iterators.html#using-closures-that-capture-their-environment
fn closure_capturing_environment() {
    utils::println_function_name!();
    let nums = vec![1, 2, 3, 4, 5];
    let condition = 2;
    let nums: Vec<_> = nums.into_iter().filter(|x| x % condition == 0)
        .collect();
    assert_eq!(nums, vec![2, 4])
}