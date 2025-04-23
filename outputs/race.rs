use std::thread;

const NUM_THREADS: usize = 1_000_000;

// Shared variable
static mut COUNTER: i32 = 0;

// Thread function
fn increment_counter() {
    unsafe {
        // Increment the counter
        COUNTER += 1;
    }
}

fn main() {
    let mut handles = vec![];

    // Create threads
    for _ in 0..NUM_THREADS {
        let handle = thread::spawn(increment_counter);
        handles.push(handle);
    }

    // Join threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final value of counter
    unsafe {
        println!("Final counter value: {}", COUNTER);
    }
}