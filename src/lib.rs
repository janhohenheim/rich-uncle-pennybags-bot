#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod exchange;
mod telegram;
mod bot;
mod model;
mod routes;

pub mod error;
pub use bot::RichUnclePennybagsBot;
