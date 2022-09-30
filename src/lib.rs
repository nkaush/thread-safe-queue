use std::sync::{Condvar, Mutex, MutexGuard};
use std::collections::VecDeque;

pub struct ThreadSafeQueue<T> {
    m: Mutex<VecDeque<T>>,
    cv: Condvar
}

impl<T> ThreadSafeQueue<T> {
    pub fn new() -> Self {
        Self {
            m: Mutex::new(VecDeque::new()),
            cv: Condvar::new()
        }
    }

    pub fn pull(&self) -> T {
        let mut guard: MutexGuard<VecDeque<T>> = self.cv.wait_while(
                self.m.lock().unwrap(), 
                |q| q.is_empty()
            ).unwrap();

        // unwrap since we are guaranteed there is at least 1 element
        guard.pop_front().unwrap()
    }

    pub fn push(&self, value: T) {
        let mut guard: MutexGuard<VecDeque<T>> = self.m.lock().unwrap();
        guard.push_back(value);
        self.cv.notify_one();
    }
}