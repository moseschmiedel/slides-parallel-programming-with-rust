use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    const THREADS: u64 = 10000;
    let y = Arc::new(0);

    for _ in 0..THREADS {
        let y = Arc::clone(&y);
        threads.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            *y += 1;
        }));
    }

    for th in threads {
        let _ = th.join();
    }

    println!("y = {}", y);
}
