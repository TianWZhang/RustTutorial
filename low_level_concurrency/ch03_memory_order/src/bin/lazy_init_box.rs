use std::sync::atomic::{AtomicPtr, Ordering};

struct Data([u8; 100]);

fn generate_data() -> Data {
    Data([123; 100])
}

fn get_data() -> &'static Data {
    // use a null pointer as the placeholder for the initial state
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    // make sure that allocating and initializing the data doesn't race with
    // reading it
    let mut p = PTR.load(Ordering::Acquire);
    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        // We also load a potentially non-null pointer through the compare_exchange operation
        // when it fails. So we need to use Acquire for the compare_exchange failure memory 
        // ordering to be able to synchronize with the operation that stores the pointer.
        if let Err(e) = PTR.compare_exchange(
            std::ptr::null_mut(),
            p,
            Ordering::Release,
            Ordering::Acquire,
        ) {
            // Safety: p coms from Box::into_raw right above, and wasn't shared with
            // any other thread.
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    unsafe { &*p }
}

fn main() {
    println!("{:p}", get_data());
    println!("{:p}", get_data()); // Same address as before
}
