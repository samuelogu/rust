// use std::time;
// use std::thread::sleep;
// use std::{thread, time};
use tokio::time::{sleep, Duration};

async fn hello_world() {
    sleep(Duration::from_millis(10)).await;
    println!("Done sleeping");
}

pub async fn run() {
    println!("starting");
    hello_world().await;
    println!("done")
}
