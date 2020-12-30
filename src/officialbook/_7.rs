use crate::officialbook::_7::back_of_house::Appetizer;

/// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
pub mod front_of_house {
    struct Order {
        something: i32
    }

    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }

        fn seat_at_table() {
            // child module can see a non-public item of the parent module.
            super::Order { something: 1 };
        }
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad(String),
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub use crate::officialbook::_7::front_of_house::hosting;

/// front_of_house and this function are siblings so that no need to make the module public.
pub fn eat_at_restaurant() {
    // Absolute path
    crate::officialbook::_7::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    self::front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad("aaa".to_string());
}