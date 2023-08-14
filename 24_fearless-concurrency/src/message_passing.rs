use std::{sync::mpsc, thread, time::Duration};

pub fn message_passing() {
    // Using Message Passing to Transfer Data Between Threads
    let (tx1, rx1) = mpsc::channel();
    let tx3 = tx1.clone();

    thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();
    });

    let received = rx1.recv().unwrap();
    println!("Got: {}", received);

    // Sending Multiple Values and Seeing the Receiver Waiting
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in values {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx2 {
        println!("Got: {}", received);
    }

    // Creating Multiple Producers by Cloning the Transmitter
    thread::spawn(move || {
        let values = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in values {
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx1 {
        println!("Got: {}", received);
    }
}
