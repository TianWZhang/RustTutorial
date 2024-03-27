use std::thread;

struct A {
    msg: String
}

impl A {
    fn foo(&self) {
        thread::scope(|s| {
            for _ in 0..10 {
                s.spawn(|| println!("{}", self.msg));
            }
        });
    }
}

fn main() {
    let numbers = vec![1, 2, 3];
    // It allows us to spawn threads that cannot outlive the scope of the closure we
    // pass to that function, making it possible to safely borrow local variables from
    // longer-living parent thread.
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{}", n);
            }
        });
    });
    // When the scope ends, all threads that haven't been joined yet are automatically joined.

    A {
        msg: "hello".to_string()
    }
    .foo();
}
