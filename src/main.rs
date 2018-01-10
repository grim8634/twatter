#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#[macro_use] extern crate log;
extern crate env_logger;
extern crate twitter_api;
extern crate oauth_client;
extern crate config;

mod twitter;
mod counter;

fn main() {
    env_logger::init().unwrap();

    let mut config = config::Config::default();

    config
        .merge(config::File::with_name("twatter"))
        .expect("Unable to load twatter.toml")
        .merge(config::Environment::with_prefix("twatter"))
        .expect("Failed to read ENV");

    twitter::run(config);
}


