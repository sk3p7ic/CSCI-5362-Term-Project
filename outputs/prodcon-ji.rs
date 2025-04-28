use std::sync::{Mutex, Arc};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const BUF_SIZE: usize = 7;

type BufItem = i32;

struct Buffer {
    items: Vec<BufItem>,
    empty: usize,
    full: usize,
    mutex: Mutex<()>,
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            items: Vec::with_capacity(BUF_SIZE),
            empty: BUF_SIZE,
            full: 0,
            mutex: Mutex::new(),
        }
    }

    fn insert_item(&mut self, item: BufItem) {
        {
            let _lock = self.mutex.lock().unwrap();
            while self.empty == 0 {
                // Wait for space - busy-waiting (for simplicity, would use condition variables in real code)
                drop(_lock);
                thread::sleep(Duration::from_millis(10));
                let _lock = self.mutex.lock().unwrap();
            }
            self.items.push(item);
            self.empty -= 1;
            self.full += 1;
        }
    }

    fn remove_item(&mut self) -> Option<BufItem> {
        let item = {
            let _lock = self.mutex.lock().unwrap();
            while self.full == 0 {
                // Wait for items - busy-waiting (for simplicity, would use condition variables in real code)
                drop(_lock);
                thread::sleep(Duration::from_millis(10));
                let _lock = self.mutex.lock().unwrap();
            }
            self.full -= 1;
            self.empty += 1;
            self.items.remove(0)
        };
        Some(item)
    }
}

fn main() {
    println!("Hello, World!");

    let buffer = Arc::new(Mutex::new(Buffer::new()));

    // Example of inserting and removing items in threads could go here

    let _buffer = Arc::clone(&buffer);
    thread::spawn(move || {
        let mut buf = _buffer.lock().unwrap();
        buf.insert_item(1);
    });

    let _buffer = Arc::clone(&buffer);
    thread::spawn(move || {
        let mut buf = _buffer.lock().unwrap();
        if let Some(item) = buf.remove_item() {
            println!("Removed item: {}", item);
        }
    });

    thread::sleep(Duration::from_secs(1));  // Give threads time to finish
}