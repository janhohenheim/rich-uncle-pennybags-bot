#![feature(plugin, custom_attribute)]
#![plugin(rocket_codegen)]
extern crate heck;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate rand;

mod exchange;
mod telegram;
mod bot;
mod model;
mod routes;

pub mod error;
pub use bot::RichUnclePennybagsBot;
