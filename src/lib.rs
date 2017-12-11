#![recursion_limit = "1024"]

pub mod display;
pub mod events;
pub mod input;
pub mod logging;
pub mod objects;
pub mod game;
#[macro_use]
extern crate slog;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
