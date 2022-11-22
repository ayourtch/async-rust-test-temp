use std::collections::HashMap;
use std::time::{Duration, SystemTime};


// comment out the next line to avoid crash
use tide::Request;


// use tide::prelude::*;

async fn litir() -> () {
    println!("hello2");
}

fn main() {
    println!("hello");

    std::thread::sleep(std::time::Duration::from_secs(1));
    async_std::task::block_on(litir());
}
