use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();
    let mut threads = vec![];

    let arx = Arc::new(Mutex::new(rx));
    for i in 0..5 {
        let rx = arx.clone();
        threads.push(thread::spawn(move || loop {
            let rx = rx.lock().unwrap();
            if let Ok(it) = rx.recv() {
                drop(rx);
                sleep(Duration::from_millis(100));
                println!("worker {} received {}", i, it);
            } else {
                break;
            }
        }));
    }

    for i in 0..100 {
        tx.send(format!("msg {}", i)).unwrap();
    }

    drop(tx);

    for t in threads {
        t.join().unwrap();
    }
}
