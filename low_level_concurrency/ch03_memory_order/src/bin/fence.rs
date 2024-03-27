use std::{sync::atomic::{fence, AtomicBool, Ordering}, thread, time::Duration};

static mut DATA: [u64; 10] = [0; 10];
const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn some_calculation(i: usize) -> u64 {
    thread::sleep(Duration::from_millis(400 + i as u64 % 3 * 100));
    123
}

fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            let data = some_calculation(i);
            unsafe { DATA[i] = data };
            READY[i].store(true, Ordering::Release);
        });
    }

    thread::sleep(Duration::from_millis(500));
    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Ordering::Relaxed));
    if ready.contains(&true) {
        // execute the fence reading the data only if there is data to be read
        // save the overhead of additional acquire operations
        fence(Ordering::Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data{} = {}", i, unsafe { DATA[i] });
            }
        }
    }
}