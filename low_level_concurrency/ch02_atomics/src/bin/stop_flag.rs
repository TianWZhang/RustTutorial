use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = thread::spawn(|| {
        // check the stop flag before each new iteration
        while !STOP.load(Ordering::Relaxed) {
            // do something else
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Use the main thread to listen for user input.
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }

    STOP.store(true, Ordering::Relaxed);
    background_thread.join().unwrap();
}
