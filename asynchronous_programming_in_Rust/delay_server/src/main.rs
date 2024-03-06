use axum::{routing::get, Router, extract::Path};
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::time::{sleep, Duration};

const EXPLANATION: &str =
"USAGE:
Delay server works by issuing a http GET request in the format:
http://localhost:8080/[delay in ms]/[UrlEncoded meesage]

On reception, it immidiately reports the following to the console:
{Message #} - {delay in ms}: {message}

The server then delays the response for the requested time and echoes the message back to the caller.

REQUESTS:
--------
";
static COUNTER: AtomicUsize = AtomicUsize::new(1);

pub async fn delay(Path((delay_ms, message)): Path<(u64, String)>) -> String {
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("#{count} - {delay_ms}ms: {message}");
    sleep(Duration::from_millis(delay_ms)).await;
    message
} 

#[tokio::main]
async fn main() {
    println!("{}", EXPLANATION);
    let app = Router::new()
        .route("/:delay/:message", get(delay));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
