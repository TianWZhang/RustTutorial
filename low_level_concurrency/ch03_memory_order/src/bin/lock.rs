use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED.swap(true, Ordering::Acquire) == false {
        // Safety: We hold the exclusive lock, so nothing else is accessing DATA.
        unsafe { DATA.push('!') };
        LOCKED.store(false, Ordering::Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });
    assert_eq!(100, unsafe { DATA.len() });
}
