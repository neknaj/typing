// lib.rs
#![cfg(feature = "web")]

mod model;
mod msg;
mod update;
mod jsapi;
mod parser;
mod typing;

pub use update::update;
pub use update::new_model;