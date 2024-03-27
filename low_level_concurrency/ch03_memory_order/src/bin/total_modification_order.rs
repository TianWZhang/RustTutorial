use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

static X: AtomicI32 = AtomicI32::new(0);

fn a1() {
    X.fetch_add(5, Ordering::Relaxed);
}

fn a2() {
    X.fetch_add(10, Ordering::Relaxed);
}

fn b() {
    let a = X.load(Ordering::Relaxed);
    let b = X.load(Ordering::Relaxed);
    let c = X.load(Ordering::Relaxed);
    let d = X.load(Ordering::Relaxed);
    // Threads cannot observe any values from X that
    // are inconsistent with the total modification order.
    // Even if there's more than one possible order of modification
    // for an atomic variable, all threads will agree on a single order.
    println!("{} {} {} {}", a, b, c, d);
}

fn main() {
    thread::scope(|s| {
        s.spawn(a1);
        s.spawn(a2);
        s.spawn(b);
    });
}
