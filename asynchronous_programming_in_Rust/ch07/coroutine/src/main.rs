use std::{thread, time::Duration};

use future::{Future, PollState};

use crate::http::Http;

mod future;
mod http;

struct Coroutine {
    state: State
}

enum State {
    Start,
    // When we call Http::get, we get a HttpGetFuture returned that we store here. 
    // At this point, we return control back to the calling function so it can do other things if needed.
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Resolved
}

impl Coroutine {
    fn new() -> Self {
        Self {
            state: State::Start
        }
    }
}

impl Future for Coroutine {
    type Output = ();

    fn poll(&mut self) -> PollState<Self::Output> {
        // The loop allows us to drive the state machine forward until we reach a point
        // where we can't progress any further without getting PollState::Pending from one
        // of the child futures.
        loop {
            match self.state {
                State::Start => {
                    println!("Program starting");
                    // receive a future that we need to poll to completion before we progress any further
                    let fut = Box::new(Http::get("/600/HelloWorld1"));
                    self.state = State::Wait1(fut);
                }
                State::Wait1(ref mut fut) => match fut.poll() {
                    PollState::Ready(txt) => {
                        println!("{}", txt);
                        let fut2 = Box::new(Http::get("/400/HelloWorld2"));
                        self.state = State::Wait2(fut2);
                    }
                    PollState::Pending => break PollState::Pending
                }
                State::Wait2(ref mut fut2) => match fut2.poll() {
                    PollState::Ready(txt2) => {
                        println!("{}", txt2);
                        self.state = State::Resolved;
                        break PollState::Ready(());
                    }
                    PollState::Pending => break PollState::Pending
                }
                State::Resolved => panic!("Polled a resolved future")
            }
        }
    }
}

fn async_main() -> impl Future<Output = ()> {
    Coroutine::new()
}

fn main() {
    let mut future = async_main();
    loop {
        match future.poll() {
            PollState::Pending => {
                // The control is yielded back to us.
                println!("Schedule other tasks");
            },
            PollState::Ready(_) => break
        }
        thread::sleep(Duration::from_millis(100));
    }
}
