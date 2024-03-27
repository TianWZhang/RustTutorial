use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        // Safety: Nothing else is accessing DATA because the READY flag is not set yet.
        unsafe { DATA = 123 };
        READY.store(true, Ordering::Release); // Everything from before this Release-store ..
    });

    while !READY.load(Ordering::Acquire) {
        // .. is visible after this Acquire-load loads true.
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    // Safety: Nothing is mutating DATA because the READY flag is set.
    println!("{}", unsafe { DATA });
}
