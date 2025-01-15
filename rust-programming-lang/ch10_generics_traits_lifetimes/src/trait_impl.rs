use std::fmt::Display;

use crate::{Summary, Tweet};

// Traits as Parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
// The impl Trait syntax works for straightforward cases but is actually
// syntax sugar for a longer form known as a trait bound; it looks like this:
pub fn notify2<T: Summary>(item: &T) {
    // This longer form is equivalent to the notify func
    println!("Breaking news! {}", item.summarize());
}

// The impl Trait syntax is convenient and makes for more concise code in simple cases,
// Using impl Trait is appropriate if we want this function to allow item1 and item2 to
// have different types:
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {} // or
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}
//

// Specifying Multiple Trait Bounds with the + Syntax
// pub fn notify(item: &(impl Summary + Display)) {}   // or
// pub fn notify<T: Summary + Display>(item: &T) {

// Clearer Trait Bounds with where Clauses:
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { } // or
//
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
// T: Display + Clone,
// U: Clone + Debug,
// { }

// Returning Types That Implement Traits:
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// you can only use impl Trait if you’re returning a single type
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
