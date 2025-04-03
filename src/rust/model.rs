// model.rs

use serde::{Serialize, Deserialize};
use ts_rs::TS;
use crate::parser::Content;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct MenuModel {
    pub available_contents: Vec<Content>,
    pub layout: TextConvert,
    pub error_messages: Vec<ErrorMsg>,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingStartModel {
    pub content: Content,
    pub available_contents: Vec<Content>,
    pub layout: TextConvert,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingModel {
    pub content: Content,
    pub user_input: Vec<TypingSession>,
    pub status: TypingStatus,
    pub available_contents: Vec<Content>,
    pub layout: TextConvert,
    pub keyboard_remapping: KeyboardRemapping,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct PauseModel {
    pub typing_model: TypingModel,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct ResultModel {
    pub typing_model: TypingModel,
    // pub start_time: Option<f64>,
    // pub end_time: Option<f64>,
    // pub pause_time: Option<f64>,
}

// ------------------------------------
// Top-level Model enum
// ------------------------------------

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
#[serde(tag = "type")]
pub enum Model {
    Menu(MenuModel),
    TypingStart(TypingStartModel),
    Typing(TypingModel),
    Pause(PauseModel),
    Result(ResultModel),
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingStatus {
    pub line: i32,
    pub segment: i32,
    pub char_: i32,
    pub unconfirmed: Vec<char>,
    pub last_wrong_keydown: Option<char>,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingSession {
    pub inputs: Vec<TypingInput>,
}
#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingInput {
    pub key: char,
    pub timestamp: f64,
    pub is_correct: bool,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct ErrorMsg {
    pub message: String,
    pub timestamp: f64,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TextConvert {
    pub mapping: Vec<(String, Vec<String>)>,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct KeyboardRemapping {
    pub mapping: HashMap<char, char>,
}