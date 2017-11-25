#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

mod exchange;
mod telegram;
mod bot;
pub mod error;
pub use bot::RichUnclePennybagsBot;
