enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use self::List::{Cons, Nil};
use std::rc::Rc;

pub fn rc() {
    // Rc<T>, the Reference Counted Smart Pointer
    let a2 = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b2 = Cons(3, Rc::clone(&a2));
    let c2 = Cons(4, Rc::clone(&a2));

    let a3 = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a3));
    let b3 = Cons(3, Rc::clone(&a3));
    println!("count after creating b = {}", Rc::strong_count(&a3));
    {
        let c3 = Cons(4, Rc::clone(&a3));
        println!("count after creating c = {}", Rc::strong_count(&a3));
    }
    println!(
        "count after c goes out of scope = {}",
        Rc::strong_count(&a3)
    );
}
