use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping {} !", self.data);
    }
}

fn multi_thread() {
    let (tx, rx) = mpsc::channel();

    let tx_1 = tx.clone();
    let _ = thread::spawn(move || {
        let val = vec![
            String::from("1-a"),
            String::from("1-b"),
            String::from("1-c"),
            String::from("1-d"),
        ];
        for i in val {
            tx_1.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    let _ = thread::spawn(move || {
        let val = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ];
        for i in val {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    for r in rx {
        println!("{:?}", r);
    }
}

fn mut_d() {
    let count = Arc::new(Mutex::new(0));

    let mut handlers = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&count);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("result: {}", *count.lock().unwrap());
}

fn main() {
    let _ = thread::spawn(|| multi_thread());
    let _ = thread::spawn(|| mut_d());
}
