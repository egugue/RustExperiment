pub fn main() {
    utils::println_file_name!();
}

fn immutable_data_cannot_borrow_as_mutable() {
    // let x = 1;
    // let y = &mut x;
    //
    // 6 |     let x = 1;
    //   |         - help: consider changing this to be mutable: `mut x`
    // 7 |     let y = &mut x;
    //   |             ^^^^^^ cannot borrow as mutable
}

/// A Use Case for Interior Mutability: Mock Objects
/// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#a-use-case-for-interior-mutability-mock-objects
mod mock {
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cell::RefCell;

        struct MockMessenger {
            sent_messages: Vec<String>,
        }

        impl Messenger for MockMessenger {
            fn send(&self, msg: &str) {
                // cannot compile because Messenger trait requires immutable self reference.
                // This situation can solve using RefCell.
                // self.sent_messages.push(msg.to_string())
            }
        }

        struct MockMessenger2 {
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger2 {
            fn new() -> MockMessenger2 {
                MockMessenger2 {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        impl Messenger for MockMessenger2 {
            fn send(&self, msg: &str) {
                // RefCell enables the hold immutable reference to borrow as mutable.
                self.sent_messages.borrow_mut().push(msg.to_string());
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger2::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

            limit_tracker.set_value(80);

            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }
    }
}
