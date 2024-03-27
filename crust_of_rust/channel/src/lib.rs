pub mod one_shot_channel;
pub mod one_shot_channel_borrowing;

use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

pub struct MySender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for MySender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);
        Self {
            // not using self.shared.clone()
            // Imagine that the inner type also implemented Clone, Rust won't know whether
            // this call is supposed to clone the Arc or the thing inside the Arc, because Arc
            // dereferences to the inner type.
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for MySender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_one();
        }
    }
}

impl<T> MySender<T> {
    pub fn send(&mut self, t: T) {
        self.shared.inner.lock().unwrap().queue.push_back(t);
        self.shared.available.notify_one();
    }
}

pub struct MyReceiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> MyReceiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        // protect Condvar from spurious wakeups
        loop {
            match inner.queue.pop_front() {
                // Because there's only one receiver, any time we take the lock, we might as
                // well steal all the items that have been queued up rather than just steal one.
                Some(t) => {
                    std::mem::swap(&mut self.buffer, &mut inner.queue);
                    return Some(t);
                }
                None if inner.senders == 0 => return None,
                // the OS doesn't guarantee that you wake up for a reason
                None => inner = self.shared.available.wait(inner).unwrap(),
            }
        }
    }
}

impl<T> Iterator for MyReceiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

pub fn channel<T>() -> (MySender<T>, MyReceiver<T>) {
    let shared = Arc::new(Shared {
        inner: Mutex::new(Inner {
            queue: VecDeque::new(),
            senders: 1,
        }),
        available: Condvar::new(),
    });
    (
        MySender {
            shared: shared.clone(),
        },
        MyReceiver {
            shared: shared.clone(),
            buffer: VecDeque::new(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }
}
