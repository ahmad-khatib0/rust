mod slices;

fn main() {
    println!("Hello, world!");

    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{s}");

    // this is simple, x and y are on the stack
    let x = 5;
    let y = x;

    // When we assign s1 to s2, the String data is copied, meaning we copy the pointer,
    // the length, and the capacity that are on the stack. We do not copy the data on
    // the heap that the pointer refers to.
    let s1 = String::from("hello");
    // To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer
    // valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope
    // so in this way rust prevent what is known as a double free error
    let s2 = s1;

    // println!("{s1}, world!"); // will result in error (borrow of moved value: `s1`)
    // If you’ve heard the terms shallow copy and deep copy while working with other languages,
    // the concept of copying the pointer, length, and capacity without copying the data probably
    // sounds like making a shallow copy. But because Rust also invalidates the first
    // variable, instead of being called a shallow copy, it’s known as a move.

    // If we do want to deeply copy the heap data of the String, not just the stack data
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // why the following works without clone? obviously because these vars lives on the stack
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    ownership_in_functions();

    ownership_in_functions_with_return();

    let s1 = String::from("hello");
    let (s2, len) = calculate_length_v1(s1);
    println!("The length of '{s2}' is {len}.");

    // References and Borrowing (References refer to some value without taking ownership)
    let s1 = String::from("hello");
    let len = calculate_length_v2(&s1);
    println!("The length of '{s1}' is {len}."); // see that s1 still valid

    let mut s = String::from("hello");
    mutable_ref(&mut s);

    // Mutable references have one big restriction: if you have a mutable
    // reference to a value, you can have no other references to that value.
    // This code that attempts to create two mutable references to s will fail:
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;  // the subsequent assignment using r1 is not allowed
    // (cannot borrow `s` as mutable more than once at a time)
    println!("{r1}");

    // As always, we can use curly brackets to create a new scope,
    // allowing for multiple mutable references, just not simultaneous ones:
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems
    let r2 = &mut s;

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // Rust enforces a similar rule for combining mutable and immutable references.
    // so cannot have a mutable reference while we have an immutable one to the same value.
    // This code results in an error:
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{r1}, {r2}, and {r3}");
    // Users of an immutable reference don’t expect the value to suddenly change out from under them!
    // However, multiple immutable references are allowed because no one who is just reading
    // the data has the ability to affect anyone else’s reading of the data.
    //

    // this is a fix for the previous problem
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.
    let r3 = &mut s; // no problem
    println!("{r3}");

    // Dangling References
    let reference_to_nothing = dangle();

    // The Slice Type
    slices::slices()
}

fn ownership_in_functions() {
    // Passing a variable to a function will move or copy, just as assignment does
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // because i32 implements the Copy trait, x does NOT move
                   // into the function, so it's okay to use x afterward

    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    // some_string comes ⬆️ into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes ⬆️ into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
  //
  //

fn ownership_in_functions_with_return() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back,
                                       // which also moves its return value into s3

    // s2 is not valid here
    println!("{s1} {s3}");
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length_v1(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn calculate_length_v2(s: &String) -> usize {
    // s.push_str("something"); // Attempting to modify a borrowed value will result in error
    s.len()
}

fn mutable_ref(some_string: &mut String) {
    some_string.push_str(", world");
}

// missing lifetime specifier
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // s will be deallocated. But we tried to return a reference to it.
// } // Here, s goes out of scope and is dropped, so its memory goes away. Danger!

fn dangle() -> String {
    let s = String::from("Hello");
    s
}
