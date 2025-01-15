// Validating References with Lifetimes

use std::fmt::Display;

pub fn lifetimes() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // &i32         // a reference
    // &'a i32      // a reference with an explicit lifetime
    // &'a mut i32  // a mutable reference with an explicit lifetime

    // Using the longest fn with a ref to String values that have different concrete lifetimes
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // Next, let’s try an example that shows that the lifetime of the
    // reference in result must be the smaller lifetime of the two arguments.
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");
    // error:  `string2` does not live long enough
    // The error shows that for result to be valid for the println!
    // statement, string2 would need to be valid until the end of the outer scope.

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // The Static Lifetime
    // One special lifetime we need to discuss is 'static, which denotes that the affected
    // reference can live for the entire duration of the program. All string literals have
    // the 'static lifetime, which we can annotate as follows:
    let s: &'static str = "I have a static lifetime.";
    // The text of this string is stored directly in the program’s binary,
    // which is always available. Therefore, the lifetime of all string literals is 'static.
}

// Preventing Dangling References with Lifetimes

// At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of
// 'a but that it refers to memory with a lifetime of 'b. The program is rejected because 'b is
// shorter than 'a: the subject of the reference doesn’t live as long as the reference.
// fn borrow_checker_scopes() {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {r}");   //          |
// }                         // ---------+

// error: missing lifetime specifier
// help: this function's return type contains a borrowed value, but
//       the signature does not say whether it is borrowed from `x` or `y`
//
// The help text reveals that the return type needs a generic lifetime parameter on it because Rust
// can’t tell whether the reference being returned refers to x or y. Actually, we don’t know either,
// because the if block in the body of this function returns a reference to x and the else block
// returns a reference to y!.. so we can’t look at the scopes to determine whether the reference
// we return will always be valid.
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// We want the signature to express the following constraint: the returned reference will be valid
// as long as both the parameters are valid. This is the relationship between lifetimes of the
// parameters and the return value. We’ll name the lifetime 'a and then add it to each reference,
// So: The longest function definition specifying that all the references in
//     the signature must have the same lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// The way in which you need to specify lifetime parameters depends on what your function is doing.
// E,g, if we changed the implementation of the longest function to always return the first parameter
// rather than the longest string slice, we wouldn’t need to specify a lifetime on the y parameter
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// error: cannot return reference to local variable `result`
// the return value lifetime is not related to the lifetime of the parameters at all.
// fn longest3<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// Lifetime Annotations in Struct Definitions:
struct ImportantExcerpt<'a> {
    //  A struct that holds a reference, requiring a lifetime annotation
    part: &'a str,
}

// Lifetime Annotations in Method Definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
// The lifetime parameter declaration after impl and its use after the type name are required, but we’re
// not required to annotate the lifetime of the reference to self because of the first elision rule.
// Here is an example where the third lifetime elision rule applies:
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
// There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self
// and announcement their own lifetimes. Then, because one of the parameters is &self, the return type
// gets the lifetime of &self, and all lifetimes have been accounted for.

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
fn longest_with_an_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
