use std::fmt::Result;
use std::io::Result as IoResult;
// Bringing HashMap into scope in an idiomatic way (structs, enums, and other items)
use std::collections::HashMap;

use crate::front_of_house::hosting;

// Bringing the add_to_waitlist function into scope with use, which is unidiomatic
use crate::front_of_house::hosting::add_to_waitlist;

// Using Nested Paths to Clean Up Large use Lists
// use std::cmp::Ordering;
// use std::io; // instead of

use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write; // instead of

// use std::io::{self, Write}; // use instead

// The Glob Operator
use std::collections::*;

// crate
//   └── front_of_house
//     ├── hosting
//     |   │
//     |   ├── add_to_waitlist
//     |   │
//     |   └── seat_at_table
//     └── serving
//         ├── take_order
//         ├── serve_order
//         └── take_payment

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    fn seat_at_table() {}

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        //  by using super at the start of the path. This is like starting a
        //  filesystem path with the .. syntax.
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // the default for enum variants (fields) is to be public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Bringing a module into scope with use
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed to see or
    // modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1, 2);
}

mod customer {
    pub fn eat_at_restaurant() {
        // Bringing a module into scope with use dose not work here
        // A use statement only applies in the scope it’s in:
        // hosting::add_to_waitlist();
    }
}

// fn function1() -> Result {
//     // --snip--
// }
// fn function2() -> IoResult<> {
//     // --snip--
// }

// Re-exporting Names with pub use
// the module that import this carte will have the ability to use this import
// as if it was imported in that importing program
// pub use crate::front_of_house::hosting;
//
