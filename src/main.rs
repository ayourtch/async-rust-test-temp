use clap::Parser as ClapParser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use tide::Request;
use tide::prelude::*;

/// This program does something useful, but its author needs to edit this.
/// Else it will be just hanging around forever
#[derive(Debug, Clone, ClapParser, Serialize, Deserialize)]
#[clap(version = env!("GIT_VERSION"), author = "Andrew Yourtchenko <ayourtch@gmail.com>")]
struct Opts {
    /// Target hostname to do things on
    #[clap(short, long, default_value = "localhost")]
    target_host: String,

    /// Override options from this yaml/json file
    #[clap(short, long)]
    options_override: Option<String>,

    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}


#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

async fn litir_main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:4000").await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}


fn main() {
    let opts: Opts = Opts::parse();

    // allow to load the options, so far there is no good built-in way
    let opts = if let Some(fname) = &opts.options_override {
        if let Ok(data) = std::fs::read_to_string(&fname) {
            let res = serde_json::from_str(&data);
            if res.is_ok() {
                res.unwrap()
            } else {
                serde_yaml::from_str(&data).unwrap()
            }
        } else {
            opts
        }
    } else {
        opts
    };

    if opts.verbose > 4 {
        let data = serde_json::to_string_pretty(&opts).unwrap();
        println!("{}", data);
        println!("===========");
        let data = serde_yaml::to_string(&opts).unwrap();
        println!("{}", data);
    }

    println!("Hello, here is your options: {:#?}", &opts);

    // std::thread::sleep(std::time::Duration::from_secs(1));`
    async_std::task::block_on(litir_main());
}
