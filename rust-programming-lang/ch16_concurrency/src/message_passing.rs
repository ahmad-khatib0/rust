use std::{sync::mpsc, thread, time::Duration};

pub fn message_passing() {
    // Using Message Passing to Transfer Data Between Threads
    // “Do not communicate by sharing memory; instead, share memory by communicating.”

    // mpsc: multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    // the sending end or the transmitter, the receiving end

    thread::spawn(move || {
        let val = String::from("Hi");
        // the send will return err. we’re calling unwrap to panic in case of an error.
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    // Channels and Ownership Transference
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {val}"); // error: borrow of moved value
    });
    let received = rx.recv().unwrap();
    println!("Got: {received}");

    // Sending Multiple Values and Seeing the Receiver Waiting
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    // Creating Multiple Producers by Cloning the Transmitter
    let (tx, rx) = mpsc::channel();
    // we call clone on the transmitter. This will give us a new
    // transmitter we can pass to the first spawned thread.
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
