// model.rs

use serde::{Serialize, Deserialize};
use ts_rs::TS;
use crate::parser::Content;

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct MenuModel {
    pub available_contents: Vec<Content>,
    pub error_messages: Vec<ErrorMsg>,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingModel {
    pub content: Content,
    pub user_input: String,
    pub start_time: Option<f64>,
    pub available_contents: Vec<Content>,
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
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub pause_time: Option<f64>,
}

// ------------------------------------
// Top-level Model enum
// ------------------------------------

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
#[serde(tag = "type")]
pub enum Model {
    Menu(MenuModel),
    Typing(TypingModel),
    Pause(PauseModel),
    Result(ResultModel),
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct TypingStatus {
    pub line: i32,
    pub segment: i32,
    pub char: i32,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct ErrorMsg {
    pub content: String,
    pub timestamp: f64,
}
