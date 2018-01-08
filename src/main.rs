#![feature(plugin)]
#![plugin(dotenv_macros)]
use std::env;

extern crate dotenv;
extern crate rich_uncle_pennybags_bot;
use rich_uncle_pennybags_bot::RichUnclePennybagsBot;

fn main() {
    dotenv::dotenv().ok();
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    let username = env::var("TELEGRAM_USERNAME").unwrap();
    let coinfile = env::var("COINFILE").unwrap();
    let error = RichUnclePennybagsBot::new(&token, &username, &coinfile).start();
    println!("Failed to launch bot: {}", &error);
}
