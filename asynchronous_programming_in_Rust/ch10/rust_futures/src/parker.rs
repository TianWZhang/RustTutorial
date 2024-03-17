use std::sync::{Condvar, Mutex};

#[derive(Default)]
pub struct Parker(Mutex<bool>, Condvar);

impl Parker {
    pub fn park(&self) {
        // We acquire a lock to the Mutex which protects our flag indicating
        // if we should resume execution or not.
        let mut resumable = self.0.lock().unwrap();

        // We put this in a loop since there is a chance we'll get woken, but
        // our flag hasn't changed. If that happens, we simply go back to sleep.
        while !*resumable {
            // sleep until someone notifies us
            resumable = self.1.wait(resumable).unwrap();
        }
        // We immediately set the condition to false, so that next time we call `park`
        // we'll go right to sleep.
        *resumable = false;
    }

    pub fn unpark(&self) {
        *self.0.lock().unwrap() = true;
        self.1.notify_one();
    }
}

#[test]
fn parker_works() {
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread,
        time::Duration,
    };

    let flag = Arc::new(AtomicBool::new(false));
    let parker = Arc::new(Parker::default());

    thread::spawn({
        let flag = flag.clone();
        let parker = parker.clone();
        move || {
            thread::sleep(Duration::from_millis(200));
            flag.store(true, Ordering::SeqCst);
            parker.unpark();
        }
    });
    assert!(!flag.load(Ordering::SeqCst));
    parker.park();
    assert!(flag.load(Ordering::SeqCst));
}
