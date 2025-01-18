// RefCell<T> and the Interior Mutability Pattern
// Mutating the value inside an immutable value is the interior mutability pattern
//

// A Use Case for Interior Mutability: Mock Objects
pub trait Messenger {
    fn send(&self, msg: &str);
}

/// A library to keep track of how close a value is to a
/// maximum value and warn when the value is at certain levels
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
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

// we want to test the behavior of the set_value method

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // struct MockMessenger {
    //     sent_messages: Vec<String>,
    // }

    // impl MockMessenger {
    //     fn new() -> MockMessenger {
    //         MockMessenger {
    //             sent_messages: vec![],
    //         }
    //     }
    // }

    // impl Messenger for MockMessenger {
    // We can’t modify the MockMessenger to keep track of the messages because the send method
    // takes an immutable reference to self. We also can’t take the suggestion from the error text
    // to use &mut self instead because then the signature of send wouldn’t match the signature
    // in the Messenger trait definition
    // fn send(&self, message: &str) {
    //     self.sent_messages.push(String::from(message));
    // }
    // }

    // #[test]
    // fn it_sends_an_over_75_percent_warning_message() {
    //     let mock_messenger = MockMessenger::new();
    //     let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    //     limit_tracker.set_value(80);
    //     assert_eq!(mock_messenger.sent_messages.len(), 1);
    // }

    // This is a situation in which interior mutability can help
    //
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // borrow_mut to get a mutable reference to the value inside the RefCell<Vec<String>>,
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }

        // Creating two mutable references in the same scope will panic at runtime
        // error:  'already borrowed: BorrowMutError'
        // fn send(&self, message: &str) {
        //     let mut one_borrow = self.sent_messages.borrow_mut();
        //     let mut two_borrow = self.sent_messages.borrow_mut();
        //     one_borrow.push(String::from(message));
        //     two_borrow.push(String::from(message));
        // }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        // borrow to get an immutable reference to the vector
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub fn ref_cell() {
    // Allowing Multiple Owners of Mutable Data with Rc<T> and RefCell<T>:
    // Using Rc<RefCell<i32>> to create a List that we can mutate
    let value = Rc::new(RefCell::new(5));
    // We need to clone value so both a and value have ownership of the inner 5 value
    // rather than transferring ownership from value to a or having a borrow from value.
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
