use std::sync::{Arc, Mutex};
use std::sync::Semaphore;
use std::thread;
use std::time::Duration;

const BUF_SIZE: usize = 7;

type BufItem = i32;

struct Buffer {
    items: [BufItem; BUF_SIZE],
    in_index: usize,
    out_index: usize,
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            items: [-1; BUF_SIZE],
            in_index: 0,
            out_index: 0,
        }
    }

    fn insert_item(&mut self, item: BufItem) {
        self.items[self.in_index] = item;
        self.in_index = (self.in_index + 1) % BUF_SIZE;
    }

    fn remove_item(&mut self) -> BufItem {
        let item = self.items[self.out_index];
        self.out_index = (self.out_index + 1) % BUF_SIZE;
        item
    }
}

fn main() {
    println!("Hello, World!");

    let empty = Arc::new(Semaphore::new(BUF_SIZE));
    let full = Arc::new(Semaphore::new(0));
    let mutex = Arc::new(Mutex::new(Buffer::new()));

    let empty_clone = empty.clone();
    let full_clone = full.clone();
    let mutex_clone = mutex.clone();

    thread::spawn(move || {
        for i in 0..10 {
            empty_clone.acquire();
            {
                let mut buffer = mutex_clone.lock().unwrap();
                buffer.insert_item(i);
            }
            full_clone.release();
            thread::sleep(Duration::from_millis(100));
        }
    });

    let mutex_clone = mutex.clone();
    let full_clone = full.clone();
    
    thread::spawn(move || {
        for _ in 0..10 {
            full_clone.acquire();
            let item;
            {
                let mut buffer = mutex_clone.lock().unwrap();
                item = buffer.remove_item();
            }
            empty.release();
            println!("Removed item: {}", item);
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::sleep(Duration::from_secs(3));
}