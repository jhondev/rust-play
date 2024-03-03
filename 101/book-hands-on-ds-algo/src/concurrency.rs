use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn threading() {
    let x = 10;
    let handle = thread::spawn(move || {
        println!("Hello from a thread, the number is {}", x);
    });
    handle.join().unwrap();
}

pub fn channels() {
    const N: i32 = 10;
    let (tx, rx) = channel();
    let handles = (0..N).map(|i| {
        let _tx = tx.clone();
        thread::spawn(move || {
            // don't use the result
            let _ = _tx.send(i).unwrap();
        })
    });
    // close all threads
    for h in handles {
        h.join().unwrap();
    }
    // receive N times
    let numbers: Vec<i32> = (0..N).map(|_| rx.recv().unwrap()).collect();
    println!("channels | {:?}", numbers);
}

pub fn shared_state() {
    let v = Arc::new(Mutex::new(vec![]));
    let handles = (0..10).map(|i| {
        let numbers = Arc::clone(&v);
        thread::spawn(move || {
            let mut vector = numbers.lock().unwrap();
            (*vector).push(i);
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("shared state | {:?}", *v.lock().unwrap());
}
