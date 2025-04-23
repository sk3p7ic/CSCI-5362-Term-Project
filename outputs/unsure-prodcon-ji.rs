use std::sync::{Arc, Mutex, Condvar};
use std::thread;

const BUF_SIZE: usize = 7;

type BufItem = i32;

struct Buffer {
    items: Vec<BufItem>,
    empty: usize,
    full: usize,
    capacity: usize,
}

impl Buffer {
    fn new(capacity: usize) -> Self {
        Buffer {
            items: Vec::with_capacity(capacity),
            empty: capacity,
            full: 0,
            capacity,
        }
    }

    fn insert_item(&mut self, item: BufItem, cvar: &Condvar) {
        while self.empty == 0 {
            cvar.wait();
        }
        self.items.push(item);
        self.empty -= 1;
        self.full += 1;
        cvar.notify_all();
    }

    fn remove_item(&mut self, cvar: &Condvar) -> BufItem {
        while self.full == 0 {
            cvar.wait();
        }
        let item = self.items.remove(0);
        self.full -= 1;
        self.empty += 1;
        cvar.notify_all();
        item
    }
}

fn main() {
    println!("Hello, World!");

    let buffer = Arc::new(Mutex::new(Buffer::new(BUF_SIZE)));
    let cvar = Arc::new(Condvar::new());

    // In a multithreaded scenario, we would create producer and consumer threads here

    // Example of an insert and remove operations (in a single-threaded manner)
    {
        let mut buffer = buffer.lock().unwrap();
        buffer.insert_item(10, &cvar);
        let item = buffer.remove_item(&cvar);
        println!("Removed item: {}", item);
    }
}