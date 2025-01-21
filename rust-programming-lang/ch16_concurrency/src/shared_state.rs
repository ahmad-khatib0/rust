use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn shared_state() {
    // Using Mutexes to Allow Access to Data from One Thread at a Time

    let m = Mutex::new(5);
    {
        // The call to lock would fail if another thread holding the lock panicked.
        // In that case, no one would ever be able to get the lock, so we’ve chosen
        // to unwrap and have this thread panic if we’re in that situation.
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // Sharing a Mutex<T> Between Multiple Threads
    // atomics work like primitive types but are safe to share across threads.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
