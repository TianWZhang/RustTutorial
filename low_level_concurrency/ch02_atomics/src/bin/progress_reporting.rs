use std::{
    sync::atomic::{AtomicU32, AtomicUsize, Ordering},
    thread,
    time::Duration,
};

#[allow(dead_code)]
fn increment(a: &AtomicU32) {
    let mut current = a.load(Ordering::Relaxed);
    loop {
        let new = current + 1;
        // update the value of a only if its value is still the same value we loaded before
        match a.compare_exchange(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            Err(v) => current = v,
        }
    }
}

fn main() {
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each.
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..25 {
                    thread::sleep(Duration::from_millis(100));
                    // We can not use the `store` method, as that would overwrite
                    // the progress from other threads.
                    num_done.fetch_add(1, Ordering::Relaxed);
                    // any status updates are immediately reported to the user
                    main_thread.unpark();
                }
            });
        }

        // The main thread shows status updates every second.
        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
        }
    });
    println!("Done!");
}
