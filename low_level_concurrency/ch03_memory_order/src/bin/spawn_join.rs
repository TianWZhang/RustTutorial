use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

static X: AtomicI32 = AtomicI32::new(0);

fn f() {
    let x = X.load(Ordering::Relaxed);
    assert!(x == 1 || x == 2);
}

fn main() {
    X.store(1, Ordering::Relaxed);
    // Spawning a thread creates a happens-before relationship between
    // what happened before the spawn() call, and the new thread.
    let t = thread::spawn(f);
    X.store(2, Ordering::Relaxed);
    // Joining a thread creates a happens-before relationship between
    // the joined thread and what happens after the join() call.
    t.join().unwrap();
    X.store(3, Ordering::Relaxed);
}
