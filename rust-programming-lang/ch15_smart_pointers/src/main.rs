use std::ops::Deref;
mod rc;
mod ref_cell;
mod ref_cycle;
mod weak;

// 2- Using Box<T> to Get a Recursive Type with a Known Size
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use weak::weak;

use crate::List::{Cons, Nil};

fn main() {
    // Using Box<T> to Point to Data on the Heap
    // Boxes allow you to store data on the heap rather than the stack.

    let b = Box::new(5);
    println!("b = {b}");

    // 1- Enabling Recursive Types with Boxes
    // error: recursive type `List` has infinite size
    // insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
    // #[derive(Debug)]
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }
    // The error shows this type “has infinite size.” The reason is that we’ve defined List with a
    // variant that is recursive: it holds another value of itself directly. As a result, Rust can’t
    // figure out how much space it needs to store a List value.
    //
    // Indirection means that instead of storing a value directly, we should change the data
    // structure to store the value indirectly by storing a pointer to the value instead.

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Treating Smart Pointers Like Regular References with Deref
    // Using the dereference operator to follow a reference to an i32 value
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // use *y to follow the reference to the value it’s pointing to (hence dereference)
    assert_eq!(5, *y);
    // Using Box<T> Like a Reference instead of a reference
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // The main difference between y and prev y is that here we set y to be an instance of a box
    // pointing to a copied value of x rather than a reference pointing to the value of x

    // Defining Our Own Smart Pointer
    // The Box<T> type is ultimately defined as a tuple struct with one
    // element, so Listing MyBox defines a type in the same way
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // err before impl Deref: type `MyBox<{integer}>` cannot be dereferenced

    // Our MyBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type
    // Implementing the Deref Trait
    impl<T> Deref for MyBox<T> {
        // defines an associated type for the Deref trait to use. Associated types are a
        // slightly different way of declaring a generic parameter,
        type Target = T;
        // The reason the deref method returns a reference to a value, and that the plain dereference
        // outside the parentheses in *(y.deref()) is still necessary, has to do with the ownership
        // system. If the deref method returned the value directly instead of a reference to the value,
        // the value would be moved out of self. We don’t want to take ownership of the inner value inside
        // MyBox<T> in this case or in most cases where we use the dereference operator.
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // Implicit Deref Coercions with Functions and Methods
    let m = MyBox::new(String::from("Rust"));
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    // Calling hello with a reference to a MyBox<String> value, which works because of deref coercion
    // Because we implemented the Deref trait on MyBox<T>, Rust can turn &MyBox<String> into &String
    // by calling deref. The standard library provides an implementation of Deref on String that returns
    // a string slice, and this is in the API documentation for Deref. Rust calls deref again to turn
    // the &String into &str, which matches the hello function’s definition.
    hello(&m);
    // If Rust didn’t implement deref coercion, we would have to write the code:
    hello(&(*m)[..]);

    // Running Code on Cleanup with the Drop Trait
    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    // c.drop(); // err: explicit destructor calls not allowed
    // Rust doesn’t let us call drop explicitly because Rust would still automatically call drop on
    // the value at the end of main. This would cause a double free error because Rust would be
    // trying to clean up the same value twice.
    // variables will be dropped in the reverse order of their creation, so d was dropped before c.
    println!("CustomSmartPointers created.");

    // We can’t disable the automatic insertion of drop when a value goes out of scope,
    // and we can’t call the drop method explicitly. So, if we need to force a value to
    // be cleaned up early, we use the std::mem::drop function.
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    rc::rc();
    ref_cell::ref_cell();
    ref_cycle::ref_cycle();
    weak::weak();
}
