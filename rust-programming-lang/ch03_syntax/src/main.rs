fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 6

    let spaces = "   ";
    let spaces = spaces.len();

    // let mut spaces = "  ";
    // spaces = spaces.len(); // not allowed

    // let guess = "42".parse().expect("Not a number!"); // annotation is required
    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0; //      f64  // double precision
    let y: f32 = 3.0; // f32  // single-precision
    println!("{spaces} {guess} {x} {y}");

    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5; // remainder

    let t = true;
    let f: bool = false; // with explicit type annotation

    // The Character Type (using single quotes)
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!(
        "{sum} {difference} {product} {quotient} {truncated} {remainder} {c} {z} {t} {f} {heart_eyed_cat} "
    );

    tupples();

    // array ( arrays in Rust have a fixed length.)
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // the same as writing let a = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];

    // let x = (let y = 6);

    let y = {
        let x = 3;
        x + 1 // no simicolon as you notice
    };

    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // let number = if condition { 5 } else { "six" }; // error
    println!("The value of number is: {number}");

    loops()
}

fn tupples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    // use pattern matching to destructure a tuple value
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn loops() {
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
