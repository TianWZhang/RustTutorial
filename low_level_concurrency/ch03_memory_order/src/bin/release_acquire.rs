use std::{
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    thread,
    time::Duration,
};

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        DATA.store(123, Ordering::Relaxed);
        println!("the address of underlying integer: {:p}", DATA.as_ptr());
        READY.store(true, Ordering::Release); // Everything from before this Release-store ..
    });

    while !READY.load(Ordering::Acquire) {
        // .. is visible after this Acquire-load loads true.
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("the address of underlying integer: {:p}", DATA.as_ptr());
    println!("{}", DATA.load(Ordering::Relaxed));
}
