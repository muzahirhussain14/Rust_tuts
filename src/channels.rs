/*
Channels --- Message passing between threads
*/


use std::thread;
use crossbeam_channel::unbounded;

enum Message {
    PrintData(String),
    Sum(i64, i64),
    Quit
}

fn main() {

    let (sender, receiver) = unbounded();
    
    let worker = thread::spawn(move || loop {
            match receiver.recv() {
                Ok(msg) => match msg  {
                    Message::PrintData(data) => println!("Data Received: {}", data),
                    Message::Sum(a, b) => println!("{} + {} = {}", a, b, a+b),
                    Message::Quit => {
                        println!("Thread Terminating");
                        break
                    },
                },
                _ => {
                    print!("Didn't receive or unable to read the message");
                    break;
                }
            }
        }
    );

    sender.send(Message::PrintData("Hello from the main Thread".to_owned()));
    sender.send(Message::Sum(10, 21));
    sender.send(Message::Quit);

    worker.join();
}