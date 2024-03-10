use std::time::Instant;
use crate::http::Http;
use future::*;

mod future;
mod http;

fn get_path(i: usize) -> String {
    format!("/{}/HelloWorld{}", i * 1000, i)
}

coroutine fn read_request(i: usize) {
    let txt = Http::get(&get_path(i)).wait;
    println!("{}", txt);
}

// around 10s
coroutine fn async_main1() {
    println!("Program starting");
    let txt = Http::get(&get_path(0)).wait;
    println!("{}", txt);
    let txt = Http::get(&get_path(1)).wait;
    println!("{}", txt);
    let txt = Http::get(&get_path(2)).wait;
    println!("{}", txt);
    let txt = Http::get(&get_path(3)).wait;
    println!("{}", txt);
    let txt = Http::get(&get_path(4)).wait;
    println!("{}", txt);
}

// around 4s
coroutine fn async_main2() {
    println!("Program starting");
    let mut futures = vec![];
    for i in 0..5 {
        futures.push(read_request(i));
    }
    future::join_all(futures).wait;
}

fn main() {
    let start = Instant::now();
    let mut future = async_main2();
    loop {
        match future.poll() {
            PollState::Pending => (),
            PollState::Ready(_) => break
        }
    }
    println!("\nElapsed Time: {}", start.elapsed().as_secs_f32());
}
