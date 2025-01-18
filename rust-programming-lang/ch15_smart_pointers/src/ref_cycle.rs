use std::cell::RefCell;
use std::rc::Rc;

use self::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    // instead of having the ability to modify the i32 value as we did in Listing 15-24, we
    // want to modify the List value a Cons variant is pointing to.
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn ref_cycle() {
    // Reference Cycles Can Leak Memory, Creating A Reference Cycles
    let a2 = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a2));
    println!("a next item = {:?}", a2.tail());

    let b2 = Rc::new(Cons(10, RefCell::new(Rc::clone(&a2))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a2));
    println!("b initial rc count = {}", Rc::strong_count(&b2));
    println!("b next item = {:?}", b2.tail());

    if let Some(link) = a2.tail() {
        *link.borrow_mut() = Rc::clone(&b2);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b2));
    println!("a rc count after changing a = {}", Rc::strong_count(&a2));
    // Uncomment the next line to see that we have a cycle; it will overflow the stack.
    // println!("a next item = {:?}", a2.tail());
}
