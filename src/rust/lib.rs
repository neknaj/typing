// src/lib.rs
#![cfg(feature = "web")]


mod model;
mod msg;
mod update;
mod jsapi;

pub use update::update;
pub use update::new_model;