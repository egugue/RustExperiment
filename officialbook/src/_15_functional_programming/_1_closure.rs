use std::thread;
use std::time::Duration;

pub fn main() {
    utils::println_file_name!();
    closure_can_catch_outer_values();
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    // generate_workout_1(simulated_user_specified_value, simulated_random_number);
    generate_workout_2(simulated_user_specified_value, simulated_random_number);
}

fn closure_can_catch_outer_values() {
    let value = 1;
    let closure = || value;
    println!("closure can catch outer value: {}", closure());
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout_1(intensity: u32, random_number: u32) {
    utils::println_function_name!();
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

/// https://doc.rust-lang.org/book/ch13-01-closures.html#refactoring-with-closures-to-store-code
fn generate_workout_2(intensity: u32, random_number: u32) {
    utils::println_function_name!();

    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        let count = expensive_closure(intensity);
        println!(
            "Today, do {} pushups!",
            count
        );
        println!(
            "Next, do {} situps!",
            count
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

/// a variety of how to define a closure
/// https://doc.rust-lang.org/book/ch13-01-closures.html#closure-type-inference-and-annotation
fn how_to_define_closure(x: u32) -> u32 {
    let add_one = |x: u32| -> u32 { x + 1 };
    let closure = |x: u32| { x + 1 };
    // let closure = |x| -> u32  { x + 1 }; // error[E0282]: type annotations needed
    let closure = || -> u32 { 1 };
    let closure = || { 1 };
    let closure = || 1;
    x + 1
}

/// https://doc.rust-lang.org/book/ch13-01-closures.html#closure-type-inference-and-annotation
fn infer_signature() {
    let closure = |x| x;
    closure("a");
    // cannot compile because the compiler inferred that the arg type and the return type are &str.
    // closure(1);
}