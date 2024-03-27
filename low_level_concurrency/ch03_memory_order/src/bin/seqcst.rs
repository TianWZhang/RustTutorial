use std::{sync::atomic::{AtomicBool, Ordering}, thread};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);
static mut S: String = String::new();

fn main() {
    let a = thread::spawn(|| {
        // Both threads first set their own atomic boolean to true to warn the 
        // other thread that they are about to access S, and then check the other's
        // atomic boolean to see if they can safely access S without causing a data race.

        // Virtually all real-world uses of SeqCst involve a similar pattern of a store that
        // must be globally visible before a subsequent load on the same thread. For these
        // situations a potentially more efficient alternative is to instead use relaxed
        // operation in combination with a SeqCst fence.
        A.store(true, Ordering::SeqCst);
        if !B.load(Ordering::SeqCst) {
            unsafe { S.push('!') };
        }
    });
    let b = thread::spawn(|| {
        B.store(true, Ordering::SeqCst);
        if !A.load(Ordering::SeqCst) {
            unsafe { S.push('!') };
        }
    });
    a.join().unwrap();
    b.join().unwrap();
    println!("{}", unsafe { &S} );
}