use std::time::Instant;
use crate::http::Http;
use future::*;

mod future;
mod http;

fn get_path(i: usize) -> String {
    format!("/{}/HelloWorld{}", i * 1000, i)
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


// =================================
// We rewrite this:
// =================================
    
// coroutine fn read_request(i: usize) {
//     let txt = Http::get(&get_path(i)).wait;
//     println!("{}", txt);

// }

// =================================
// Into this:
// =================================

fn read_request(i: usize) -> impl Future<Output=String> {
    Coroutine0::new(i)
}
        
enum State0 {
    Start(usize),
    Wait1(Box<dyn Future<Output = String>>),
    Resolved,
}

struct Coroutine0 {
    state: State0,
}

impl Coroutine0 {
    fn new(i: usize) -> Self {
        Self { state: State0::Start(i) }
    }
}


impl Future for Coroutine0 {
    type Output = String;

    fn poll(&mut self) -> PollState<Self::Output> {
        loop {
        match self.state {
                State0::Start(i) => {
                    // ---- Code you actually wrote ----
                    // ---------------------------------
                    let fut1 = Box::new( Http::get(&get_path(i)));
                    self.state = State0::Wait1(fut1);
                }

                State0::Wait1(ref mut f1) => {
                    match f1.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                            println!("{}", txt);
                            // ---------------------------------
                            self.state = State0::Resolved;
                            break PollState::Ready(String::new());
                        }
                        PollState::Pending=> break PollState::Pending,
                    }
                }

                State0::Resolved => panic!("Polled a resolved future")
            }
        }
    }
}


// =================================
// We rewrite this:
// =================================
    
// coroutine fn async_main1() {
//     println!("Program starting");
//     let txt = Http::get(&get_path(0)).wait;
//     println!("{}", txt);
//     let txt = Http::get(&get_path(1)).wait;
//     println!("{}", txt);
//     let txt = Http::get(&get_path(2)).wait;
//     println!("{}", txt);
//     let txt = Http::get(&get_path(3)).wait;
//     println!("{}", txt);
//     let txt = Http::get(&get_path(4)).wait;
//     println!("{}", txt);

// }

// =================================
// Into this:
// =================================

fn async_main1() -> impl Future<Output=String> {
    Coroutine1::new()
}
        
enum State1 {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Wait3(Box<dyn Future<Output = String>>),
    Wait4(Box<dyn Future<Output = String>>),
    Wait5(Box<dyn Future<Output = String>>),
    Resolved,
}

struct Coroutine1 {
    state: State1,
}

impl Coroutine1 {
    fn new() -> Self {
        Self { state: State1::Start }
    }
}


impl Future for Coroutine1 {
    type Output = String;

    fn poll(&mut self) -> PollState<Self::Output> {
        loop {
        match self.state {
                State1::Start => {
                    // ---- Code you actually wrote ----
                    println!("Program starting");

                    // ---------------------------------
                    let fut1 = Box::new( Http::get(&get_path(0)));
                    self.state = State1::Wait1(fut1);
                }

                State1::Wait1(ref mut f1) => {
                    match f1.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                            println!("{}", txt);
                            // ---------------------------------
                            let fut2 = Box::new( Http::get(&get_path(1)));
                            self.state = State1::Wait2(fut2);
                        }
                        PollState::Pending => break PollState::Pending,
                    }
                }

                State1::Wait2(ref mut f2) => {
                    match f2.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                            println!("{}", txt);
                            // ---------------------------------
                            let fut3 = Box::new( Http::get(&get_path(2)));
                            self.state = State1::Wait3(fut3);
                        }
                        PollState::Pending => break PollState::Pending,
                    }
                }

                State1::Wait3(ref mut f3) => {
                    match f3.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                            println!("{}", txt);
                            // ---------------------------------
                            let fut4 = Box::new( Http::get(&get_path(3)));
                            self.state = State1::Wait4(fut4);
                        }
                        PollState::Pending => break PollState::Pending,
                    }
                }

                State1::Wait4(ref mut f4) => {
                    match f4.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                            println!("{}", txt);
                            // ---------------------------------
                            let fut5 = Box::new( Http::get(&get_path(4)));
                            self.state = State1::Wait5(fut5);
                        }
                        PollState::Pending => break PollState::Pending,
                    }
                }

                State1::Wait5(ref mut f5) => {
                    match f5.poll() {
                        PollState::Ready(txt) => {
                            // ---- Code you actually wrote ----
                            println!("{}", txt);
                            // ---------------------------------
                            self.state = State1::Resolved;
                            break PollState::Ready(String::new());
                        }
                        PollState::Pending=> break PollState::Pending,
                    }
                }

                State1::Resolved => panic!("Polled a resolved future")
            }
        }
    }
}


// =================================
// We rewrite this:
// =================================
    
// coroutine fn async_main2() {
//     println!("Program starting");
//     let mut futures = vec![];
//     for i in 0..5 {
//         futures.push(read_request(i));
//     }
//     future::join_all(futures).wait;

// }

// =================================
// Into this:
// =================================

fn async_main2() -> impl Future<Output=String> {
    Coroutine2::new()
}
        
enum State2 {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Resolved,
}

struct Coroutine2 {
    state: State2,
}

impl Coroutine2 {
    fn new() -> Self {
        Self { state: State2::Start }
    }
}


impl Future for Coroutine2 {
    type Output = String;

    fn poll(&mut self) -> PollState<Self::Output> {
        loop {
        match self.state {
                State2::Start => {
                    // ---- Code you actually wrote ----
                    println!("Program starting");
    let mut futures = vec![];
    for i in 0..5 {
        futures.push(read_request(i));
    }
                    // ---------------------------------
                    let fut1 = Box::new(future::join_all(futures));
                    self.state = State2::Wait1(fut1);
                }

                State2::Wait1(ref mut f1) => {
                    match f1.poll() {
                        PollState::Ready(_) => {
                            // ---- Code you actually wrote ----
                            // ---------------------------------
                            self.state = State2::Resolved;
                            break PollState::Ready(String::new());
                        }
                        PollState::Pending=> break PollState::Pending,
                    }
                }

                State2::Resolved => panic!("Polled a resolved future")
            }
        }
    }
}
