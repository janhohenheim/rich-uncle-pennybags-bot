#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod exchange;
mod telegram;
mod bot;
pub mod error;
pub use bot::RichUnclePennybagsBot;
