use rand::Rng;
use std::cmp::Ordering;
use std::{io, u32}; // prelude

fn main() {
    println!("Guess the number!"); // println! called a macro

    // rand is local to the current thread of execution and is seeded by the operating system
    let secret_number = rand::thread_rng().gen_range(1..=100); // start..=end

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // In Rust, variables are immutable by default
        let mut guess = String::new();

        io::stdin()
            //  like variables, references are immutable by default. Hence,
            //  you need to write &mut guess rather than &guess to make it mutable.
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadow the previous value of guess with a new one
        // We switch from an expect call to a match expression to move from
        // crashing on an error to handling the error.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // .expect("Please type a number!");

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    let x = 6;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
}
