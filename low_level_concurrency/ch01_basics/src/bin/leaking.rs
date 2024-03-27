use std::thread;

fn main() {
    // One can release ownership of a Box, promising to never drop it. From
    // that point on, the Box will live forever, without an owner.
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    let t1 = thread::spawn(move || {
        // Only a reference is moved into the threads.
        // Note that references are `Copy`, meaning that when you move them, the
        // original still exists.
        dbg!(x);
        let id = thread::current().id();
        println!("thread id: {:?}", id);
    });
    let t2 = thread::spawn(move || {
        dbg!(x);
        let id = thread::current().id();
        println!("thread id: {:?}", id);
    });
    t1.join().unwrap();
    t2.join().unwrap();
}
