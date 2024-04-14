use std::io::stdin;

fn main() {
    println!("Hello, world!");

    // Rust variables default to being immutable , mut means mmutable
    let mut my_name = String::new();

    // &mut : “Borrow” the variable; allowing changes to be made to your variable by the called function.
    // Prefixing a variable with an ampersand (&) creates a reference to the variable
    // ( THIS IS ALSO CALLED BORROWING ) you’re lending the variable to the function you are calling.
    // Lending with &mut permits the borrowing function to mutate your variable
    stdin()
        .read_line(&mut my_name)
        .expect("failed to read line");

    // You expect the read_line function to work correctly. If it doesn’t, your program
    // will crash. Rust is returning a Result object, and you are checking that the
    // function worked by calling expect.
}
