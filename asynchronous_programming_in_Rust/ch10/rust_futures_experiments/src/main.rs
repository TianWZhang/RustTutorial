mod runtime;

use tokio::runtime::Runtime;
#[allow(dead_code)]
async fn async_main1() {
    let rt = Runtime::new().unwrap();
    // explicitly enter rt to enable the reactor
    // We can hold pn to the EnterGuard as we need the reactor up and running.
    let _guard = rt.enter();
    println!("Program starting");
    let url = "http://127.0.0.1:8080/600/HelloAsyncAwait1";
    let res = reqwest::get(url).await.unwrap();
    let txt = res.text().await.unwrap();
    println!("{}", txt);
    let url = "http://127.0.0.1:8080/400/HelloAsyncAwait2";
    let res = reqwest::get(url).await.unwrap();
    let txt = res.text().await.unwrap();
    println!("{}", txt);
}

use isahc::prelude::*;
#[allow(dead_code)]
async fn async_main2() {
    println!("Program starting");
    let url = "http://127.0.0.1:8080/600/HelloAsyncAwait1";
    let mut res = isahc::get_async(url).await.unwrap();
    let txt = res.text().await.unwrap();
    println!("{txt}");
    let url = "http://127.0.0.1:8080/400/HelloAsyncAwait2";
    let mut res = isahc::get_async(url).await.unwrap();
    let txt = res.text().await.unwrap();
    println!("{txt}");
}

async fn async_main3() {
    for i in 0..100 {
        let delay = i * 10;
        let req = format!("http://127.0.0.1:8080/{delay}/HelloAsyncAwait{i}");

        runtime::spawn(async move {
            let mut res = isahc::get_async(&req).await.unwrap();
            let txt = res.text().await.unwrap();
            println!("{txt}");
        });
    }
}

fn main() {
    let mut executor = runtime::init();
    executor.block_on(async_main3());
}
