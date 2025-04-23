use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const BUF_SIZE: usize = 7;

type BufItem = i32;

struct Buffer {
    items: [Option<BufItem>; BUF_SIZE],
    empty: usize,
    full: usize,
    mutex: Arc<Mutex<()>>,
}

impl Buffer {
    fn new() -> Buffer {
        Buffer {
            items: [None; BUF_SIZE],
            empty: BUF_SIZE,
            full: 0,
            mutex: Arc::new(Mutex::new(())),
        }
    }

    fn insert_item(&mut self, item: BufItem) {
        let _lock = self.mutex.lock().unwrap(); // Mutex lock for safety
        while self.empty == 0 {
            drop(_lock); // Unlock before sleeping
            thread::sleep(Duration::from_millis(100)); // Simulate waiting
            let _lock = self.mutex.lock().unwrap(); // Re-lock
        }
        self.items[self.full % BUF_SIZE] = Some(item);
        self.full += 1;
        self.empty -= 1;
    }

    fn remove_item(&mut self) -> Option<BufItem> {
        let _lock = self.mutex.lock().unwrap(); // Mutex lock for safety
        while self.full == 0 {
            drop(_lock); // Unlock before sleeping
            thread::sleep(Duration::from_millis(100)); // Simulate waiting
            let _lock = self.mutex.lock().unwrap(); // Re-lock
        }
        let item = self.items[self.full % BUF_SIZE].take();
        self.full -= 1;
        self.empty += 1;
        item
    }
}

fn main() {
    println!("Hello, World!");
    let buffer = Arc::new(Mutex::new(Buffer::new()));

    let buffer_clone = Arc::clone(&buffer);
    
    let producer = thread::spawn(move || {
        for i in 0..10 {
            buffer_clone.lock().unwrap().insert_item(i);
            println!("Inserted: {}", i);
        }
    });

    let consumer = thread::spawn(move || {
        for _ in 0..10 {
            let item = buffer.lock().unwrap().remove_item();
            println!("Removed: {:?}", item);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}