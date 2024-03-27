use std::sync::atomic::{AtomicU32, Ordering};

static NEXT_ID: AtomicU32 = AtomicU32::new(0);
static KEY: AtomicU32 = AtomicU32::new(0);

fn allocate_new_id1() -> u32 {
    let mut id = NEXT_ID.load(Ordering::Relaxed);
    loop {
        assert!(id < 1000, "too many IDs");
        match NEXT_ID.compare_exchange(id, id + 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}

fn allocate_new_id2() -> u32 {
    NEXT_ID
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |a| a.checked_add(1))
        .expect("too many IDs")
}

#[allow(dead_code)]
fn get_key() -> u32 {
    let key = KEY.load(Ordering::Relaxed);
    if key == 0 {
        let new_key = 78273; // generate random key
        match KEY.compare_exchange(0, new_key, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => new_key,
            Err(k) => k
        }
    } else {
        key
    }
}

fn main() {
    let id = allocate_new_id1();
    println!("id: {}", id);
    let id = allocate_new_id2();
    println!("id: {}", id);
}
