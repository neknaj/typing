// msg.rs

use serde::{Serialize, Deserialize};
use ts_rs::TS;
use crate::parser::Content;

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub enum MenuMsg {
    MoveCursor(usize),
    AddContent(String),
    Start,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub enum TypingStartMsg {
    StartTyping,
    Cancel,
    ScrollMax(f64),
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub enum TypingMsg {
    KeyInput(char),
    Pause,
    ScrollTo(f64,f64),
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub enum PauseMsg {
    Resume,
    Cancel,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub enum ResultMsg {
    BackToMenu,
    Retry,
}

// Top-level Msg enum aggregates all screen-specific messages.
#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub enum Msg {
    Menu(MenuMsg),
    Typing(TypingMsg),
    TypingStart(TypingStartMsg),
    Pause(PauseMsg),
    Result(ResultMsg),
}