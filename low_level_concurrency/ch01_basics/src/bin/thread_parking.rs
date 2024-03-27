use std::{collections::VecDeque, sync::Mutex, thread, time::Duration};

fn main() {
    let queue = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        // consuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            // While there is most likely only a very brief moment between releasing the
            // queue and parking, a pushing in the producing thread could potentially happen
            // in that moment before the thread parks itself.
            // Thanks to unpark requests getting saved for a future call to park(), we don't have
            // to worry about this.
            if let Some(item) = item {
                dbg!(item);
            } else {
                // If it gets unparked, the park() will return.
                thread::park();
                // If unpark() is called right after park() returns, but before the queue gets locked
                // emptied out, the unpark() call was unnecessary but still causes the next park() call
                // to instantly return.
            }
        });

        // producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
