// lib.rs
#![cfg(feature = "web")]

mod model;
mod msg;
mod update;
mod jsapi;
mod parser;
mod typing;

pub use update::init_model;
pub use update::event_receive_keyboard;
pub use update::typing_scroll;