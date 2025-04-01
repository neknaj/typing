// src/model.rs

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MenuModel {
    // Example: Available typing contents
    pub available_contents: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TypingModel {
    pub content: String,
    pub user_input: String,
    pub start_time: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct PauseModel {
    pub content: String,
    pub user_input: String,
    pub start_time: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct ResultModel {
    pub content: String,
    pub user_input: String,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
}

// ------------------------------------
// Top-level Model enum
// ------------------------------------

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Model {
    Menu(MenuModel),
    Typing(TypingModel),
    Pause(PauseModel),
    Result(ResultModel),
}