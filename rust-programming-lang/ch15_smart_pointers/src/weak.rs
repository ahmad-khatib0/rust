// Preventing Reference Cycles Using Weak<T>
//
// As an example, rather than using a list whose items know only about the next item, we’ll create
// a tree whose items know about their children items and their parent items.
//
// Because the value that Weak<T> references might have been dropped,
// to do anything with the value that a Weak<T> is pointing to you must
// make sure the value still exists. Do this by calling the upgrade method

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn weak() {
    // Creating a Tree Data Structure: A Node with Child Nodes
    // To start, we’ll build a tree with nodes that know about their child nodes.

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,

        // Adding a Reference from a Child to Its Parent
        parent: RefCell<Weak<Node>>,
    }

    // leaf now has two owners: leaf and branch.
    let leaf = Rc::new(Node {
        value: 3,
        // A node will be able to refer to its parent node but doesn’t own its parent.
        // leaf starts out without a parent, so we create a new, empty Weak<Node> reference instance
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = None   => Because leaf starts without a parent

    let branch = Rc::new(Node {
        value: 5,
        // because branch doesn’t have a parent node. We still have
        // leaf as one of the children of branch.
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // modify leaf to give it a Weak<Node> reference to its parent
    // Rc::downgrade function to create a Weak<Node> reference to branch
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // Visualizing Changes to strong_count and weak_count
    //

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // leaf strong = 1, weak = 0

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        // branch strong = 1, weak = 1

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        // leaf strong = 2, weak = 0
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf parent = None
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // leaf strong = 1, weak = 0
}
