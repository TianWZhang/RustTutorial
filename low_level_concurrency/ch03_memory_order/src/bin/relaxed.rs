use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.store(10, Ordering::Relaxed); // (1)
    Y.store(20, Ordering::Relaxed); // (2)
}

fn b() {
    let y = Y.load(Ordering::Relaxed); // (3)
    let x = X.load(Ordering::Relaxed); // (4)
                                       // The output can be 0 20, even though there is no possible
                                       // globally consistent order of the four operations that would
                                       // result in this outcome.
                                       // When (3) is executed, there is no happens-before relationship
                                       // with (2), which means it could load either 0 or 20.
                                       // The important and counter-intuitive thing to understand is that
                                       // operation (3) loading 20 does not result in a happens-before relationship
                                       // with (2), even though that value is the one stored by (2).
                                       // From the perspective of the thread executing b, operations (1) and (2) might
                                       // appear to happen in the opposite order.
    println!("{} {}", x, y);
}

fn main() {
    thread::scope(|s| {
        s.spawn(a);
        s.spawn(b);
    });
}
