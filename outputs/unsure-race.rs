use std::sync::Arc;
use std::thread;

const NUM_THREADS: usize = 1_000_000;

// Shared variable
struct SharedCounter {
    counter: usize,
}

fn increment_counter(shared: Arc<SharedCounter>) {
    // Increment the counter
    shared.counter += 1;
}

fn main() {
    let shared = Arc::new(SharedCounter { counter: 0 });
    let mut handles = vec![];

    // Create threads
    for _ in 0..NUM_THREADS {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            increment_counter(shared_clone);
        });
        handles.push(handle);
    }

    // Join threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final value of counter
    println!("Final counter value: {}", shared.counter);
}