use std::{
    cell::UnsafeCell, marker::PhantomData, mem::MaybeUninit, sync::atomic::{AtomicBool, Ordering}, thread::{self, Thread}
};

pub struct OneShotSender<'a, T> {
    channel: &'a OneShotChannel<T>,
    receiving_thread: Thread
}

impl<T> OneShotSender<'_, T> {
    // `send` takes `self` by value to make sure it can only be called once. Otherwise after setting the
    // `ready` flag, the receiver might read the message at any point, which could race with a second attempt
    // to send a message.
    pub fn send(self, t: T) {
        unsafe { (*self.channel.message.get()).write(t) };
        self.channel.ready.store(true, Ordering::Release);
        self.receiving_thread.unpark();
    }
}

// OneShotReceiver cannot be send between threads.
pub struct OneShotReceiver<'a, T> {
    channel: &'a OneShotChannel<T>,
    _no_send: PhantomData<*const ()>
}

impl<T> OneShotReceiver<'_, T> {
    // This function is only used for indicative purposes.
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Ordering::Relaxed)
    }

    pub fn recv(self) -> T {
        // `recv` takes `self` by value to make sure it can only be called once. Otherwise,
        // calling `recv()` more than once results in two copies of the message, even if T
        // does not implement `Copy` and thus cannot safely be copied.
        while !self.channel.ready.swap(false, Ordering::Acquire) {
            thread::park();
        }
        // SAFETY: We've just checked and reset the `ready` flag.
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

pub struct OneShotChannel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T: Send> Sync for OneShotChannel<T> {}

impl<T> OneShotChannel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    // Once the sender and receiver objects both cease to exist, the mutable borrow expires and the
    // compiler happily lets the OneShotChannel object be borrowed again by a second call to `split()`.
    // Exclusively borrowing and splitting borrows can be a powerful tool for forcing correctness.
    pub fn split(&mut self) -> (OneShotSender<T>, OneShotReceiver<T>) {
        *self = Self::new();
        (
            OneShotSender { channel: self, receiving_thread: thread::current() },
            OneShotReceiver { channel: self, _no_send: PhantomData },
        )
    }
}

impl<T> Drop for OneShotChannel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_shot_channel_borrowing() {
        // Simply put the channel in a local variable, avoiding the overhead of allocating memory.
        let mut channel = OneShotChannel::new();
        thread::scope(|s| {
            let (tx, rx) = channel.split();
            s.spawn(move || {
                tx.send("hello world");
            });
            assert_eq!(rx.recv(), "hello world");
        });
    }
}
