use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

// ------------------ One-shot channel ----------------------
pub struct OneShotSender<T> {
    channel: Arc<OneShotChannel<T>>,
}

impl<T> OneShotSender<T> {
    // `send` takes `self` by value to make sure it can only be called once. Otherwise after setting the
    // `ready` flag, the receiver might read the message at any point, which could race with a second attempt
    // to send a message.
    pub fn send(self, t: T) {
        unsafe { (*self.channel.message.get()).write(t) };
        self.channel.ready.store(true, Ordering::Release);
    }
}

pub struct OneShotReceiver<T> {
    channel: Arc<OneShotChannel<T>>,
}

struct OneShotChannel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T: Send> Sync for OneShotChannel<T> {}

pub fn one_shot_channel<T>() -> (OneShotSender<T>, OneShotReceiver<T>) {
    let a = Arc::new(OneShotChannel {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
    });
    (
        OneShotSender { channel: a.clone() },
        OneShotReceiver { channel: a.clone() },
    )
}

impl<T> OneShotReceiver<T> {
    // This function is only used for indicative purposes.
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Ordering::Relaxed)
    }

    /// Panics if no message is available yet, or if the message was already consumed.
    pub fn recv(self) -> T {
        // `recv` takes `self` by value to make sure it can only be called once. Otherwise,
        // calling `recv()` more than once results in two copies of the message, even if T
        // does not implement `Copy` and thus cannot safely be copied.
        if !self.channel.ready.swap(false, Ordering::Acquire) {
            panic!("no message available");
        }
        // SAFETY: We've just checked and reset the `ready` flag.
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

impl<T> Drop for OneShotChannel<T> {
    fn drop(&mut self) {
        // The `ready` flag indicates whether there's a not-yet-received message in the
        // cell that needs to be dropped.
        // An object can only be dropped if it is fully owned by whichever thread is dropping
        // it, with no outstanding borrows. The `AtomicBool::get_mut()` method takes an exclusive
        // reference, proving that atomic access is unnecessary.
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_one_shot_channel() {
        thread::scope(|s| {
            let (tx, rx) = one_shot_channel();
            let t = thread::current();
            s.spawn(move || {
                tx.send("hello world");
                t.unpark();
            });
            while !rx.is_ready() {
                thread::park();
            }
            assert_eq!(rx.recv(), "hello world");
        });
    }
}
