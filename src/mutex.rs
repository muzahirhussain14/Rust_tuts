/*
    Mutex in Rust
    For Mutex we use parking_lot crate.
    Any shared variable is cloned and copied into the thread.
    Mutex on its own cannot be cloned. Therefore we wrap in around the Arc which enables the cloning of the mutex.
*/

use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

struct Counter(usize);


fn main()
{
    let counter = Counter(0);
    let shared_counter = Arc::new(Mutex::new(counter));

    let thread_1_counter = shared_counter.clone();
    let thread_2_counter = shared_counter.clone();

    let thread_1 = thread::spawn( move || {

        let mut counter = thread_1_counter.lock();
        counter.0 += 1;
    });
    let thread_2 = thread::spawn( move || {

        let mut counter = thread_2_counter.lock();
        counter.0 += 1;
    });

    let _ = thread_1.join().and_then(|_| thread_2.join());
    println!("Counter: {}", shared_counter.lock().0);
}