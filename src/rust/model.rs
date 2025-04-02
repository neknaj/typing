// model.rs

use serde::{Serialize, Deserialize};
use crate::parser::Content;

#[derive(Serialize, Deserialize, Clone)]
pub struct MenuModel {
    // Example: Available typing contents
    pub available_contents: Vec<Content>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TypingModel {
    pub content: Content,
    pub user_input: String,
    pub start_time: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PauseModel {
    pub typing_model: TypingModel,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResultModel {
    pub typing_model: TypingModel,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub pause_time: Option<f64>,
}

// ------------------------------------
// Top-level Model enum
// ------------------------------------

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Model {
    Menu(MenuModel),
    Typing(TypingModel),
    Pause(PauseModel),
    Result(ResultModel),
}




#[derive(Serialize, Deserialize, Clone)]
pub struct TypingStatus {
    pub line: i32,
    pub segment: i32,
    pub char: i32,
}
