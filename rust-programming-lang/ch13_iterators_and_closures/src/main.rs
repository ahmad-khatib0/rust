use std::{thread, time::Duration};
mod fn_traits;
mod iterators;
mod iterators_perf;
mod minigrep_improved;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expencive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    // let add_one_v4 = |x| x + 1;

    let example_closure = |x| x;

    // including the following two lines at the same time will result in an error
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    // 1- Defining and calling a closure that captures an immutable reference
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);

    only_borrows();
    println!("After calling closure: {:?}", list);

    // 2- Defining and calling a closure that captures a mutable reference
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // error: cannot borrow `list` as immutable because it is also borrowed as mutable:
    // println!("After calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // 3- Using move to force the closure for the thread to take ownership of list
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    // we need to specify that list should be moved into the closure by putting the move keyword
    // The new thread might finish before the rest of the main thread finishes, or the main thread
    // might finish first. If the main thread maintains ownership of list but ends before the new
    // thread and drops list, the immutable reference in the thread would be invalid.
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // Moving Captured Values Out of Closures and the Fn Traits
    fn_traits::fn_traits();

    iterators::iterators();

    minigrep_improved::minigrep_improved();

    iterators_perf::iterators_perf();
}
