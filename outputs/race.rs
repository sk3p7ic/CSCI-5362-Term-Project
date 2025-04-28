use std::sync::{Arc, Mutex};
use std::thread;

const NUM_THREADS: usize = 1000000;

// Shared variable
struct SharedCounter {
    counter: i32,
}

impl SharedCounter {
    fn new() -> Self {
        SharedCounter { counter: 0 }
    }

    fn increment(&mut self) {
        self.counter += 1;
    }
}

fn main() {
    let shared_counter = Arc::new(Mutex::new(SharedCounter::new()));
    let mut handles = vec![];

    // Create threads
    for _ in 0..NUM_THREADS {
        let counter_clone = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            num.increment();
        });
        handles.push(handle);
    }

    // Join threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final value of counter
    let final_counter = shared_counter.lock().unwrap().counter;
    println!("Final counter value: {}", final_counter);
}