#![feature(plugin)]
#![plugin(dotenv_macros)]

extern crate dotenv;
extern crate rich_uncle_pennybags_bot;
use rich_uncle_pennybags_bot::RichUnclePennybagsBot;

fn main() {
    dotenv::dotenv().ok();
    let token = dotenv!("TELEGRAM_TOKEN");
    let username = dotenv!("TELEGRAM_USERNAME");
    let error = RichUnclePennybagsBot::new(token, username).start();
    println!("Failed to launch bot: {}", &error);
}
