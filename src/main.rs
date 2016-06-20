#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#[macro_use] extern crate log;
#[macro_use] extern crate quick_error;

extern crate env_logger;
extern crate twitter_api;
extern crate oauth_client;
extern crate toml;
extern crate serde;

mod twitter;
mod counter;
mod config;

fn main() {
    env_logger::init().unwrap();

    let config = match config::TwatterConfig::parse( "twatter.toml" ) {
        Ok(v) => v, //Arc because threads
        Err(e) => panic!("{}", e ),
    };

    twitter::run(&config);
}


