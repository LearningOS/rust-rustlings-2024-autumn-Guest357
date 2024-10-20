// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DNE
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Mutex<Queue>>, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let qc1 = Arc::clone(&q);
    let qc2 = Arc::clone(&q);
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    thread::spawn(move || {
        let q = qc1.lock().unwrap();
        for val in &q.first_half {
            let tx = tx1.lock().unwrap(); // 获取锁
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let q = qc2.lock().unwrap();
        for val in &q.second_half {
            let tx = tx2.lock().unwrap(); // 获取锁
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Mutex::new(Queue::new()));
    let tx = Arc::new(Mutex::new(tx)); // 使用 Arc 和 Mutex 包装 Sender
    let queue_length = queue.lock().unwrap().length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
