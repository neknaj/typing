// model.rs

use serde::{Serialize, Deserialize};
use ts_rs::TS;
use crate::{parser::Content, typing};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct MenuModel {
    pub available_contents: Vec<Content>,
    pub selecting: usize,
    pub text_orientation: TextOrientation,
    pub layout: TextConvert,
    pub error_messages: Vec<ErrorMsg>,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingStartModel {
    pub content: Content,
    pub available_contents: Vec<Content>,
    pub text_orientation: TextOrientation,
    pub layout: TextConvert,
    pub scroll_max: f64,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingModel {
    pub content: Content,
    pub typing_correctness: TypingCorrectnessContent,
    pub user_input: Vec<TypingSession>,
    pub status: TypingStatus,
    pub available_contents: Vec<Content>,
    pub text_orientation: TextOrientation,
    pub layout: TextConvert,
    pub keyboard_remapping: KeyboardRemapping,
    pub scroll: TypingScroll,
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
    Empty,
    Menu(MenuModel),
    TypingStart(TypingStartModel),
    Typing(TypingModel),
    Pause(PauseModel),
    Result(ResultModel),
}


// ------------------------------------
// Typing
// ------------------------------------

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
    pub line: i32,
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


#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingCorrectnessContent {
    pub lines: Vec<TypingCorrectnessLine>,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingCorrectnessLine {
    pub segments: Vec<TypingCorrectnessSegment>,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingCorrectnessSegment {
    pub chars: Vec<TypingCorrectnessChar>,
}


#[derive(Serialize, Deserialize, Clone, TS, PartialEq)]
#[ts(export, export_to = "../src/web/model.ts")]
pub enum TypingCorrectnessChar {
    Pending,
    Correct,
    Incorrect,
}

#[derive(Serialize, Deserialize, Clone, TS, PartialEq)]
#[ts(export, export_to = "../src/web/model.ts")]
pub enum TextOrientation {
    Vertical,
    Horizontal,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingScroll {
    pub last_update: f64,
    pub scroll: f64,
    pub max: f64,
}