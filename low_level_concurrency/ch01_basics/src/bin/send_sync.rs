use std::{cell::Cell, marker::PhantomData, sync::Mutex, thread, time::Duration};

// X is not sync
#[allow(dead_code)]
struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>,
}

#[allow(dead_code)]
struct Y {
    p: *mut i32,
}

unsafe impl Send for Y {}
unsafe impl Sync for Y {}

fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard);
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    // The into_inner method takes ownership of the mutex, which guarantees that
    // nothing else can have a reference to the mutex anymore, making locking unnecessary.
    assert_eq!(n.into_inner().unwrap(), 1000);
}
