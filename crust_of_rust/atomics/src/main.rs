use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread::{self, spawn},
};

pub struct MyMutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

// implement Sync for MyMutex so that we can concurrently access it from multiple threads
// T is Send bound because the inner value can be taken from multiple threads.
// T is not Sync bound because we never concurrently access the inner value from multiple
// threads at the same time.
unsafe impl<T: Send> Sync for MyMutex<T> {}

impl<T> MyMutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            v: UnsafeCell::new(t),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        /*** the 1st iteration
        // wait for the lock to become unlocked
        while self.locked.load(Ordering::Relaxed) {}

        // maybe another thread runs here
        // then both threads are going to see unlocked and get a
        // mutable reference to the same value, which is UB

        // pretend that the thread gets preempted here
        thread::yield_now();

        // lock it again so that no one else can get the lock
        self.locked.store(true, Ordering::Relaxed);
        */

        // No other thread gets to modify the value in between when we look at it and when
        // we change it. There's no space between the load and the store. There's just one
        // atomic operation that's performed on the memory location we're operating under.
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol: stay in S when locked, which means having the memory location in the
            // shared state while the lock is still held by others to avoid the ownership bouncing.
            // The moment it changes, then do we go back to doing the expensive compare_exchange where we
            // try to get exclusive access. If we fail to get the lock again, we fall back to doing the
            // read-only loop.
            while self.locked.load(Ordering::Relaxed) {
                // pretend that the thread gets preempted here
                thread::yield_now();
            }
        }

        // Safety: we hold the lock, therefore we can create a mutable reference
        // no other thread cna be in the critical section at the same time
        let res = f(unsafe { &mut *self.v.get() });

        // unlock it so that other threads can access the value
        self.locked.store(false, Ordering::Release);
        res
    }
}

fn main() {
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn(move || {
        x.store(true, Ordering::Release);
    });
    let _ty = spawn(move || {
        y.store(true, Ordering::Release);
    });

    let t1 = spawn(move || {
        while !x.load(Ordering::Acquire) {}
        if y.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    let t2 = spawn(move || {
        while !y.load(Ordering::Acquire) {}
        // Even though t2 will synchronize with ty, there's no requirement that it sees
        // any particular operation that happended to x. Because there's no happens before
        // relationship between the store in tx and the load down here.
        // This load is allowed to see any previous value of x.
        if x.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();

    let z = z.load(Ordering::SeqCst);
    println!("z: {}", z);
    // z == 2 is possible: tx, ty, t1, t2
    // z == 1 is possible: tx, t1, ty, t2

    // Can we find some execution of threads where the outcome 0 is possible?
    // We have a couple of restrictions.
    //  t1 must run "after" tx, if tx hasn't run and t1 tries to run, it's gonna spin in a loop and
    //  so at some point, it's gonna be preempted.
    //  t2 must run "after" ty
    // It seems impossible.

    //         t2    t1, t2
    // MO(x): false, true
    //         t1    t1, t2
    // MO(y): false, true
    // t1 observes the value of x from acquire-load, which means it will see all operations that happened
    // before the corresponding release-store in tx. There is no operation prior to the store, but if there 
    // were, we would be guaranteed to see them because we're synchronizing with tx. That means t1 is allowed
    // to see any value of y regardless of whether ty has run or not. Even if ty has run in world clock time,
    // it doesn't matter. The memory system is allowed to still show y == false to t1. 
}

#[test]
fn test_mymutex() {
    let l: &'static _ = Box::leak(Box::new(MyMutex::new(0)));
    let handles: Vec<_> = (0..100)
        .map(|_| {
            thread::spawn(move || {
                for _ in 0..1000 {
                    l.with_lock(|v| {
                        *v += 1;
                    })
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(l.with_lock(|v| *v), 100000);
}

#[test]
fn too_relaxed() {
    use std::sync::atomic::AtomicUsize;
    // casting: Box::leak() returns a static mutable reference, which I couldn't move into two threads
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let t1 = thread::spawn(move || {
        let r1 = y.load(Ordering::Relaxed);
        x.store(r1, Ordering::Relaxed);
        r1
    });
    let t2 = thread::spawn(move || {
        // It's possible for r2 to be 42 even though the store of 42 happens
        // after r2 is read.
        // This load of x is allowed to see any value ever stored to x, including 42.
        let r2 = x.load(Ordering::Relaxed);
        y.store(42, Ordering::Relaxed);
        r2
    });

    // modification order = MO
    // MO(x) = 0 42
    // MO(y) = 0 42

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();
    // This is a possible execution of this program.
    // r1 == r2 == 42

    println!("r1: {}", r1);
    println!("r2: {}", r2);
}


#[test]
fn test_seqcst() {
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn(move || {
        x.store(true, Ordering::SeqCst);
    });
    let _ty = spawn(move || {
        y.store(true, Ordering::SeqCst);
    });

    let t1 = spawn(move || {
        while !x.load(Ordering::SeqCst) {}
        if y.load(Ordering::SeqCst) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    let t2 = spawn(move || {
        while !y.load(Ordering::SeqCst) {}
        if x.load(Ordering::SeqCst) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();

    // If in t1, we end up with x is true and y is true, then in t2 when we see 
    // y is true, x must be true otherwise that would be inconsistent with the memory
    // ordering that these sequentially consistent operations saw.

    let z = z.load(Ordering::SeqCst);
    assert_ne!(z, 0);
}
