use dummy_waker::dummy_waker;
use std::future::Future;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

mod http;

async fn async_main() {
    println!("Program starting");
    let txt = http::Http::get("/600/HelloAsyncAwait1").await;
    println!("{}", txt);
    let txt = http::Http::get("/400/HelloAsyncAwait2").await;
    println!("{}", txt);
}

fn main() {
    let fut = async_main();
    let waker = dummy_waker();
    let mut context = Context::from_waker(&waker);
    let mut pinned = Box::pin(fut);
    loop {
        match pinned.as_mut().poll(&mut context) {
            Poll::Pending => {
                // The control is yielded back to us.
                println!("Schedule other tasks");
            }
            Poll::Ready(_) => break,
        }
        thread::sleep(Duration::from_millis(100));
    }
}
