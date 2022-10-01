use tsq::ThreadSafeQueue;
use std::thread::JoinHandle;
use std::sync::Arc;
use std::thread;

fn main() {
    let tsq = Arc::new(ThreadSafeQueue::new());
    let push_count = 5usize;

    let handles: Vec<JoinHandle<()>> = (0..push_count)
        .map(|i| {
            let tsqc = Arc::clone(&tsq);
            thread::spawn(move || {
                for _ in 0..100 {
                    tsqc.push(Some(i));
                    thread::yield_now();
                }
                
                tsqc.push(None);
            })
        })
        .collect();

    let tsqc = Arc::clone(&tsq);
    let counter = thread::spawn(move || {
        let mut none_count = 0;

        while none_count != push_count {
            match tsqc.pull() {
                Some(v) => println!("Got {}!", v),
                None => none_count += 1
            }
        }
    });

    handles.into_iter().for_each(|h| h.join().unwrap());
    counter.join().unwrap();
}
