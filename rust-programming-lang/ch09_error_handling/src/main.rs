use std::{
    fs::{self, File},
    io::{self, Error, ErrorKind, Read},
};

// Changing main to return Result<(), E> allows the use of the ? operator on Result values.
// The Box<dyn Error> type is a trait object, it means  “any kind of error.”
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;
//     Ok(())
// }
//

fn main() {
    // println!("Hello, world!");

    // panic!("crash and burn");

    // Recoverable Errors with Result
    let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // };

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Alternatives to Using match with Result<T, E> (closures)
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // Propagating Errors
    let result = read_username_from_file();

    match result {
        Ok(content) => println!("read_username_from_file file content: {content}"),
        Err(err) => println!("{}", err.to_string()),
    };

    // A Shortcut for Propagating Errors: The ? Operator
    let result = read_username_from_file2();
    match result {
        Ok(content) => println!("read_username_from_file2 file content: {content}"),
        Err(err) => println!("{}", err.to_string()),
    };

    // Attempting to use the ? in the main function that returns () won’t compile.
    // let greeting_file = File::open("hello.txt")?;

    // let last_char =
}

fn read_username_from_file() -> Result<String, io::Error> {
    let user_name_result = File::open("hello.txt");
    let mut username_file = match user_name_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/// A function that returns errors to the calling code using the ? operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    // using ? the error value gets propagated to the calling code.
    Ok(username)
}

/// A Shorter function that returns errors to the calling code using the ? operator
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

/// A Even Shorter function that returns errors to the calling code
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Using the ? operator on an Option<T> value
fn last_char_of_first_line(text: &str) -> Option<char> {
    // If text is the empty string, this call to next will return None, in which case
    // we use ? to stop and return None
    text.lines().next()?.chars().last()
}
