mod future;
mod http;
mod runtime;
use future::{Future, PollState};
use runtime::Waker;
use std::fmt::Write;

fn main() {
    let mut executor = runtime::init();
    executor.block_on(async_main());
}

// =================================
// We rewrite this:
// =================================

// coroutine fn async_main() {
//     let mut buffer = String::from("\nBUFFER:\n----\n");
//     let writer = &mut buffer;
//     println!("Program starting");
//     let txt = http::Http::get("/600/HelloAsyncAwait").wait;
//     writeln!(writer, "{txt}").unwrap();
//     let txt = http::Http::get("/400/HelloAsyncAwait").wait;
//     writeln!(writer, "{txt}").unwrap();

//     println!("{}", buffer);
// }

// =================================
// Into this:
// =================================

fn async_main() -> impl Future<Output = String> {
    Coroutine0::new()
}

enum State0 {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Resolved,
}

#[derive(Default)]
struct Stack0 {
    counter: Option<usize>,
    buffer: Option<String>,
    writer: Option<*mut String>,
}

struct Coroutine0 {
    stack: Stack0,
    state: State0,
}

impl Coroutine0 {
    fn new() -> Self {
        Self {
            state: State0::Start,
            stack: Stack0::default(),
        }
    }
}

impl Future for Coroutine0 {
    type Output = String;

    fn poll(&mut self, waker: &Waker) -> PollState<Self::Output> {
        loop {
            match self.state {
                State0::Start => {
                    // initialize stack (hoist variables)
                    self.stack.counter = Some(0);
                    self.stack.buffer = Some(String::from("\nBUFFER:\n----\n"));
                    // We cast the &mut reference to buffer to a *mut pointer.
                    self.stack.writer = Some(self.stack.buffer.as_mut().unwrap());
                    println!("Program starting");
                    let fut1 = Box::new(http::Http::get("/600/HelloAsyncAwait"));
                    self.state = State0::Wait1(fut1);
                    // save stack
                }

                State0::Wait1(ref mut f1) => {
                    match f1.poll(waker) {
                        PollState::Ready(txt) => {
                            // Restore stack
                            // We do need to store/restore the parts of the stack that will be used
                            // across wait points. Stackless coroutines still need to save some information
                            // from the stack.
                            let mut counter = self.stack.counter.take().unwrap();
                            counter += 1;
                            let writer = unsafe { &mut *self.stack.writer.take().unwrap() };
                            writeln!(writer, "{}", txt).unwrap();
                            let fut2 = Box::new(http::Http::get("/400/HelloAsyncAwait"));
                            self.state = State0::Wait2(fut2);
                            // save stack
                            self.stack.writer = Some(writer);
                            self.stack.counter = Some(counter);
                        }
                        PollState::Pending => break PollState::Pending,
                    }
                }

                State0::Wait2(ref mut f2) => {
                    match f2.poll(waker) {
                        PollState::Ready(txt) => {
                            // Restore stack
                            // We restore buffer to a &String type, not the owned version. Because transferring 
                            // ownership would invalidate the pointer in writer variable.
                            let mut counter = self.stack.counter.take().unwrap();
                            counter += 1;
                            println!("Received {} responses.", counter);
                            let buffer = self.stack.buffer.as_ref().take().unwrap();
                            let writer = unsafe { &mut *self.stack.writer.take().unwrap() };
                            writeln!(writer, "{}", txt).unwrap();
                            println!("{}", buffer);
                            self.state = State0::Resolved;
                            // Save stack / free resources
                            let _ = self.stack.buffer.take();
                            break PollState::Ready(String::new());
                        }
                        PollState::Pending => break PollState::Pending,
                    }
                }
                State0::Resolved => panic!("Polled a resolved future"),
            }
        }
    }
}
