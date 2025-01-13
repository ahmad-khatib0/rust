struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Rust does include functionality to print out debugging information, but we have to explicitly
// opt in to make that functionality available for our struct. To do that, we add the outer attribute
// #[derive(Debug)] just before the struct definition,
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Each struct is allowed to have multiple impl blocks
impl Rectangle {
    // The &self is actually short for self: &Self. Within an impl block
    // Methods can take ownership of self, borrow self immutably, as we’ve done
    // here, or borrow self mutably, just as they can any other parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //  associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    // we can choose to give a method the same name as one of the struct’s fields.
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("Hello, world!");

    // Creating Instances from Other Instances with Struct Update Syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // user1 is no longer available, but If we had given user2 new String values for both email and
    // username, and thus only used the active and sign_in_count values from user1, then user1 would
    // still be valid after creating user2. Both active and sign_in_count are types that implement
    // the Copy trait,

    // Using Tuple Structs Without Named Fields to Create Different Types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields
    struct AlwaysEqual;
    let subject = AlwaysEqual;

    // An Example Program Using Structs
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    // Refactoring with Tuples
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    // Refactoring with Structs: Adding More Meaning

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );

    // Adding Useful Functionality with Derived Traits
    // println!("rect1 is {}", rect1); // `Rectangle` doesn't implement `std::fmt::Display`
    // println!("rect1 is {:?}", rect1);
    // Putting the specifier :? inside the curly brackets tells println! we want to use an
    // output format called Debug. (this would result in an error if we didn't add the derive)

    println!("rect1 is {:?}", rect1);
    // rect1 is Rectangle { width: 30, height: 50 }
    println!("rect1 is {:#?}", rect1);
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }

    let scale = 2;
    // dbg! Prints and returns the value of a given expression
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // We don’t want dbg! to take ownership of rect1, so we use a reference to rect1
    dbg!(&rect1);
    // [src/main.rs:100:16] 30 * scale = 60
    // [src/main.rs:105:5] &rect1 = Rectangle {
    //     width: 60,
    //     height: 50,
    // }

    // Defining Methods
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Methods with More Parameters
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // Field Init Shorthand
        email,
        sign_in_count: 1,
    }
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
