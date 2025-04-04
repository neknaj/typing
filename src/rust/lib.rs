// lib.rs
#![cfg(feature = "web")]
#![cfg(target_arch = "wasm32")]

mod model;
mod msg;
mod update;
mod jsapi;
mod parser;
mod typing;
mod gui;

pub use update::init_model;
pub use update::event_receive_keyboard;
pub use update::typing_scroll;
pub use update::start_gui;