use std::thread;
use std::time::Duration;

pub fn main() {
    utils::println_file_name!();
    closure_can_catch_outer_values();
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    // generate_workout_1(simulated_user_specified_value, simulated_random_number);
    // generate_workout_2(simulated_user_specified_value, simulated_random_number);
    generate_workout_3(simulated_user_specified_value, simulated_random_number);
    fn_traits();
    fn_traits_with_arg();
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

/// https://doc.rust-lang.org/book/ch13-01-closures.html#storing-closures-using-generic-parameters-and-the-fn-traits
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
            Some(v) => v
        }
    }
}

fn generate_workout_3(intensity: u32, random_number: u32) {
    utils::println_function_name!();

    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
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

/// https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-the-environment-with-closures
fn closure_can_catch_outer_values() {
    let value = 1;
    let closure = || value;
    println!("closure can catch outer value: {}", closure());

    fn function_cannot_catch_outer_values() {
        // cannot compile
        // println!("{}", value)
    }
}

/// The variety of fn traits
fn fn_traits() {
    utils::println_function_name!();

    fn fn_once<F>(func: F) where F: FnOnce() -> String {
        println!("FnOnce {}", func());
        // invoking twice cannot compile
        // println!("FnOnce {}", func());
    }
    fn fn_<F>(func: F) where F: Fn() -> String {
        println!("Fn  {}", func());
    }
    fn fn_mut<F>(mut func: F) where F: FnMut() -> String {
        println!("FnMut  {}", func());
        println!("FnMut  {}", func());
    }

    let mut mut_s = "".to_string();
    fn_once(|| {
        mut_s.push('1');
        format!("mut_s = {}", mut_s)
    });
    fn_(|| {
        // cannot compile because Fn receives immutable references.
        // mut_s.push('2');
        format!("mut_s = {}", mut_s)
    });
    fn_mut(|| {
        mut_s.push('3');
        format!("mut_s = {}", mut_s)
    });
}

/// The variety of fn traits which take an arg.
fn fn_traits_with_arg() {
    utils::println_function_name!();

    fn fn_once<F>(func: F) where F: FnOnce(String) -> String {
        let s = "FnOnce: ".to_string();
        println!("{}", func(s));
        // println!("FnOnce {}", func()); // invoking twice cannot compile
        // println!("{}", s); // cannot compile because s moved
    }
    fn fn_<F>(func: F) where F: Fn(String) -> String {
        let s = "Fn: ".to_string();
        println!("{}", func(s));
        // println!("{}", s); cannot compile because s moved
    }
    fn fn_mut<F>(mut func: F) where F: FnMut(&mut String) -> &String {
        let mut arg = "FnMut: ".to_string();
        println!("{}", func(&mut arg));
        println!("{}", func(&mut arg));
    }

    fn_once(|mut x| {
        x.push('a');
        x
    });

    fn_(|s| s);

    fn_mut(|s| {
        s.push('a');
        s
    });
}

/// move keyword makes captured values move
fn move_keyword_closure() {
    {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        // println!("can't use x here: {:?}", x);
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }

    {
        fn fn_<F>(func: F) where F: Fn() -> bool { func(); }

        let s = "to move".to_string();
        fn_(|| { s == "".to_string() });
        println!("s = {}", s);

        fn_(move || { s == "".to_string() });
        // println!("s = {}", s); // cannot compile because s moved
    }
}
