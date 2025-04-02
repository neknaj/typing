// msg.rs

use serde::{Serialize, Deserialize};
use ts_rs::TS;
use crate::parser::Content;

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
#[serde(tag = "type")]
pub enum MenuMsg {
    SelectContent(Content),
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
#[serde(tag = "type")]
pub enum TypingMsg {
    StartTyping,
    UpdateInput(String),
    Pause,
    Finish,
    Cancel,
    Tick,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
#[serde(tag = "type")]
pub enum PauseMsg {
    Resume,
    Cancel,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
#[serde(tag = "type")]
pub enum ResultMsg {
    BackToMenu,
}

// Top-level Msg enum aggregates all screen-specific messages.
#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
#[serde(tag = "type")]
pub enum Msg {
    Menu(MenuMsg),
    Typing(TypingMsg),
    Pause(PauseMsg),
    Result(ResultMsg),
}