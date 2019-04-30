use std::sync::mpsc;
use std::thread;

fn base() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("message")).unwrap();
        tx.send(String::from("message")).unwrap();
    });

    let mut message = rx.recv().unwrap();
    println!("{}", message);
    message = rx.recv().unwrap();
    println!("{}", message);
}

fn try_recv() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("message")).unwrap();
    });

    loop {
        let message = rx.try_recv();
        match message {
            Ok(v) => {
                println!("{}", v);
                break;
            },
            Err(e) => {
                println!("null {}", e);
            }
        }
    }
}

//fn move_send() {
//    let (tx, rx) = mpsc::channel();
//
//    thread::spawn(move || {
//        let v = String::from("hello world");
//        tx.send(v).unwrap();
//        println!("{}", v);
//    });
//
//    let message = rx.recv().unwrap();
//    println!("{}", message);
//
//}

fn send_multi_val() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap();
        }
    });

    for i in rx {
        println!("{}", i);
    }
}

fn multi_sender() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap();
        }
    });
    thread::spawn(move || {
        for i in 0..5 {
            tx1.send(i).unwrap();
        }
    });

    for i in rx {
        println!("{}", i);
    }
}

fn main() {
    multi_sender();
}