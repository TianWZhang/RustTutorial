use std::{
    sync::atomic::{AtomicU64, AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

fn process_item(_i: i32) {
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each.
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    // Subtly, because the Relaxed memory ordering gives no guarantees about
                    // the relative order of operations as seen from another thread, the main
                    // thread might briefly see a new updated value of total_time, while still
                    // seeing an old value of num_done, resulting in an overestimate of the average.
                    num_done.fetch_add(1, Ordering::Relaxed);
                    total_time.fetch_add(time_taken, Ordering::Relaxed);
                    max_time.fetch_max(time_taken, Ordering::Relaxed);
                }
            });
        }

        // The main thread shows status updates every second.
        loop {
            let total_time = Duration::from_micros(total_time.load(Ordering::Relaxed));
            let max_time = Duration::from_micros(max_time.load(Ordering::Relaxed));
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            if n == 0 {
                println!("Working.. nothing done yet.");
            } else {
                println!(
                    "Working.. {}/100 done, {:?} average, {:?} peak",
                    n,
                    total_time / n as u32,
                    max_time
                );
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done!");
}
