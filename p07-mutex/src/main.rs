use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(0));
    let mut ts = vec![];

    for _ in 0..10 {
        let m = m.clone();
        let t = thread::spawn(move || {
            let mut x = m.lock().unwrap();
            *x += 1;
        });
        ts.push(t);
    }

    for t in ts {
        t.join().unwrap()
    }

    println!("m = {}", m.lock().unwrap());
}
