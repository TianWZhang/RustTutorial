mod future;
mod http;
mod parker;
mod runtime;

use crate::http::Http;

async fn async_main() {
    println!("Program starting");
    let txt = Http::get("/600/HelloAsyncAwait").await;
    println!("{}", txt);
    let txt = Http::get("/400/HelloAsyncAwait").await;
    println!("{}", txt);
}

fn main() {
    let mut executor = runtime::init();
    executor.block_on(async_main());
}
