use std::{collections::VecDeque, sync::{Condvar, Mutex}, thread, time::Duration};

fn main() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        // consuming thread
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    // Unlocking, waiting and relocking is all done by the wait method.
                    // It takes a MutexGuard that proves we've locked the mutex. It first
                    // unlocks the mutex and goes to sleep. Later, when woken up, it relocks
                    // the mutex and returns a new MutexGuard.
                    q = not_empty.wait(q).unwrap();
                }
            };
            drop(q);
            dbg!(item);
        });

        // producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
