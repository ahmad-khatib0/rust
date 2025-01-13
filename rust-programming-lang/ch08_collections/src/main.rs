use std::collections::HashMap;

fn main() {
    let vec: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}"); // 7

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    let first = &v[0];
    // v.push(6); // cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("The first element is: {first}");

    for i in &v {
        println!("{i}");
    }

    // iterate over mutable references
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // the * dereference operator to get to the value in i before we can use the += operator
        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Dropping a Vector Drops Its Elements
    let v = vec![1, 2, 3, 4];

    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string(); // &str -> String
    let s = String::from("initial contents"); // same as previous line

    // strings are UTF-8 encoded, so we can include any properly encoded data in them
    let hello = String::from("مكيلع ملاسلا");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("םוֹלשָׁ");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 is {s2}");
    // If the push_str method took ownership of s2, we wouldn’t be able to print its value

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be u sed

    // The + operator uses the add method, whose signature looks something like this:
    // fn add(self, s: &str) -> String {
    // Note: the second param is &str, but when we concatenated s2 it worked, why?

    // The reason we’re able to use &s2 in the call to add is that the compiler can
    // coerce the &String argument into a &str. When we call the add method, Rust uses
    // a deref coercion, which here turns &s2 into &s2[..]

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("hello");
    // let h = s1[0]; //  the type `String` cannot be indexed by `{integer}`
    //  Rust strings don’t support indexing. But why not?

    let hello = String::from("Hola");
    println!("{}", hello.len()); // is 4 bytes long,

    let hello = String::from("Здравствуйте");
    println!("{}", hello.len()); // is 24 bytes long, (not 12)
                                 // Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value

    // Bytes and Scalar Values and Grapheme Clusters!
    // it is stored as a vector of u8 values that looks like this:
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // 18 bytes
    //
    // If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look:
    // ['न', 'म', 'स', '◌् ', 'त', '◌े ']
    // There are six char values here, but the fourth and sixth are not letters: they’re diacritics
    // that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get
    // what a person would call the four letters that make up the Hindi word:
    // ["न", "म", "स्", "ते"]

    // Slicing Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // first four bytes of the string.

    // let s = &hello[0..1];  you can't do this

    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Storing Keys with Associated Values in Hash Maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash Maps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point: println!("{}", field_name);
    // map.insert(&field_name, &field_value);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map2 = HashMap::new();
    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map2);
    // note that iterating over a hash map happens in an arbitrary order.
}
