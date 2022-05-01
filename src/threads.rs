// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

use std::thread;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {

    let thread1 = thread::spawn( move || {
        msg_hello()
    });
    
    let thread2 = thread::spawn( move || { 
        msg_thread()
    });

    let thread3 = thread::spawn( move || {
        msg_excited()
    });

    let msg1 = thread1.join().expect("Failed to join msg one");
    let msg2 = thread2.join().expect("Failed to join msg two");
    let msg3 = thread3.join().expect("Failed to join msg three");

    println!("{}{}{}", msg1, msg2, msg3);
}