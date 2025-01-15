use std::cmp::PartialOrd;

pub fn generics() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest number is {result}");

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };
    println!("p.x = {}", float.x());

    let both_integer = Point1 { x: 5, y: 10 };
    let both_float = Point1 { x: 1.0, y: 4.0 };
    let integer_and_float = Point1 { x: 5, y: 4.0 };

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c

    // monomorphization
    let integer = Some(5);
    let float = Some(5.0);
    // When Rust compiles this code, it performs monomorphization. During that process, the compiler
    // reads the values that have been used in Option<T> instances and identifies two kinds of
    // Option<T>: one is i32 and the other is f64. As such, it expands the generic definition of
    // Option<T> into two definitions specialized to i32 and f64, thereby replacing the generic
    // definition with the specific ones. it looks like:
    // enum Option_i32 {
    //     Some(i32),
    //     None,
    // }
    // enum Option_f64 {
    //     Some(f64),
    //     None,
    // }
    // let integer = Option_i32::Some(5);
    // let float = Option_f64::Some(5.0);

    // Because Rust compiles generic code into code that specifies the type in each instance,
    // we pay no runtime cost for using generics. When the code runs,
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// only applies to a struct with a particular concrete type for the generic type parameter T
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point1<T, U> {
    x: T,
    y: U,
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

// A method that uses generic types different from its struct’s definition
impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// We can only use types whose values can be ordered. To enable comparisons, the standard library
// has the std::cmp::PartialOrd trait that you can implement on types, this example will compile,
// because the standard library implements PartialOrd on both i32 and char.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
