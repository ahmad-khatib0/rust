use std::{thread, time::Duration};

pub fn basics() {
    // Creating a New Thread with spawn
    thread::spawn(|| {
        for i in 0..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        // force a thread to stop its execution for a short duration, allowing a different thread to run
        thread::sleep(Duration::from_millis(1));
    }
    // hi number 1 from the main thread!
    // hi number 0 from the spawned thread!
    // hi number 2 from the main thread!
    // hi number 1 from the spawned thread!
    // hi number 3 from the main thread!
    // hi number 2 from the spawned thread!
    // hi number 4 from the main thread!
    // hi number 3 from the spawned thread!
    // hi number 4 from the spawned thread!
    //
    // even though we told the spawned thread to print until i is 9,
    // it only got to 5 before the main thread shut down.

    // Waiting for All Threads to Finish Using join Handles
    // A JoinHandle<T> is an owned value that, when we call the join method on it,
    // will wait for its thread to finish.

    println!("\n \n \n \n \n \n \n");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();
    // putting it here will will wait for the spawned thread to finish and then run its for loop,

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // Using move Closures with Threads
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); // error: use of moved value: `v`
    handle.join().unwrap();
}
