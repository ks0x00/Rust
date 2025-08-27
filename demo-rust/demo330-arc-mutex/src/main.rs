use std::sync::{Arc, Mutex};
use std::thread;

// Mutex = Mutual exclusion
// Arc = atomically Reference Counted
// demo220-rc와 demo230-arc 참조
// https://doc.rust-kr.org/ch16-03-shared-state.html
// Arc-Mutex는 Java의 synchronized와 유사하다.

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 10 threads
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
