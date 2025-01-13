enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// define methods on enums.
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// Patterns That Bind to Values
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// error: non-exhaustive patterns: `None` not covered
// Matches in Rust are exhaustive: we must exhaust every last possibility
// in order for the code to be valid.
// fn plus_one2(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }
//

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // above is a good way, but we can put data directly into each enum variant.
    enum IpAddr2 {
        V4(String),
        V6(String),
    }
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    // a better version alos
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    // the IpAddr3 is like defining the following structs
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    let m = Message::Write(String::from("hello"));
    m.call();

    // The Option Enum and Its Advantages Over Null Values
    let some_num = Some(5); // Option<i32>
    let some_str = Some("e"); // Option<&str>
    let absent_number: Option<i32> = None; // Option<i32>

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // error: cannot add `Option<i8>` to `i8`
    // this error message means that Rust doesn’t understand how to add an i8 and an Option<i8>,
    // because they’re different types In other words, you have to convert an Option<T> to a T before
    // you can perform T operations with it. Generally, this helps catch one of the most common issues
    // with null: assuming that something isn’t null when it actually is.

    dbg!(value_in_cents2(Coin2::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Catch-All Patterns and the _ Placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // using _ Placeholder
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // using the unit value (the empty tuple type )
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // Concise Control Flow with if let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // instead we could write this in a shorter way (don't care about Catch-All) using if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // In other words, you can think of if let as syntax sugar for a match
    // that runs code when the value matches one pattern and then ignores all other values.

    let mut count = 0;
    let coin = Coin2::Quarter(UsState::Alaska);
    match coin {
        Coin2::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // using let if else
    let coin = Coin2::Quarter(UsState::Alaska);
    if let Coin2::Quarter(state2) = coin {
        println!("State quarter from {:?}!", state2);
    } else {
        count += 1;
    }
}
