pub fn slices() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5

    s.clear();
    // this empties the String, making it equal to "" word still has the value 5 here, but there's
    // no more string that we could meaningfully use the value 5 with. word is now totally i nvalid!
    println!("{word}");

    // String Slices
    // A string slice is a reference to part of a String, and it looks like this:
    let mut s = String::from("hello world");
    // Rather than a reference to the entire String, hello is a reference to a portion of the String
    let hello = &s[0..5];
    let world = &s[6..11];

    let len = s.len();

    let slice = &s[0..2]; // is equal to:
    let slice = &s[..2];

    let slice = &s[3..len]; // is equal to:
    let slice = &s[3..];

    let slice = &s[0..len]; // is equal to:
    let slice = &s[..];

    let word = first_word_slice(&s);
    // s.clear(); // error: cannot borrow `s` as mutable because it is also borrowed as immutable
    // Rust disallows the mutable reference in clear and the immutable reference in word
    // from existing at the same time
    println!("{word}"); // hello

    // String Literals as Slices
    // The type of s here is &str: it’s a slice pointing to that specific point of the binary.
    // This is also why string literals are immutable; &str is an immutable reference.
    let s = "Hello, world!";
    // s = "can not change the variable (immutable reference)"

    // A more experienced Rustacean would write the signature shown in prev
    // first_word func as the following instead,  because it allows us to use
    // the same function on both &String values and &str values:
    // fn first_word(s: &str) -> &str {

    // Other Slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3])
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // enumerate returns as tuple of index and a ref to the item
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
