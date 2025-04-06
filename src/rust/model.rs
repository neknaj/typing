// model.rs

use serde::{Serialize, Deserialize};
use crate::{parser::Content, typing};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MenuModel {
    pub available_contents: Vec<Content>,
    pub selecting: usize,
    pub layout: TextConvert,
    pub error_messages: Vec<ErrorMsg>,
}

#[derive(Debug, Clone)]
pub struct TypingStartModel {
    pub content: Content,
    pub available_contents: Vec<Content>,
    pub layout: TextConvert,
    pub scroll_max: f64,
}

#[derive(Debug, Clone)]
pub struct TypingModel {
    pub content: Content,
    pub typing_correctness: TypingCorrectnessContent,
    pub user_input: Vec<TypingSession>,
    pub status: TypingStatus,
    pub available_contents: Vec<Content>,
    pub layout: TextConvert,
    pub keyboard_remapping: KeyboardRemapping,
    pub scroll: TypingScroll,
}

#[derive(Debug, Clone)]
pub struct PauseModel {
    pub typing_model: TypingModel,
}

#[derive(Debug, Clone)]
pub struct ResultModel {
    pub typing_model: TypingModel,
    // pub start_time: Option<f64>,
    // pub end_time: Option<f64>,
    // pub pause_time: Option<f64>,
}

// ------------------------------------
// Top-level Model enum
// ------------------------------------

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct TypingStatus {
    pub line: i32,
    pub segment: i32,
    pub char_: i32,
    pub unconfirmed: Vec<char>,
    pub last_wrong_keydown: Option<char>,
}

#[derive(Debug, Clone)]
pub struct TypingSession {
    pub line: i32,
    pub inputs: Vec<TypingInput>,
}

#[derive(Debug, Clone)]
pub struct TypingInput {
    pub key: char,
    pub timestamp: f64,
    pub is_correct: bool,
}

#[derive(Debug, Clone)]
pub struct ErrorMsg {
    pub message: String,
    pub timestamp: f64,
}

#[derive(Debug, Clone)]
pub struct TextConvert {
    pub mapping: Vec<(String, Vec<String>)>,
}

#[derive(Debug, Clone)]
pub struct KeyboardRemapping {
    pub mapping: HashMap<char, char>,
}


#[derive(Debug, Clone)]
pub struct TypingCorrectnessContent {
    pub lines: Vec<TypingCorrectnessLine>,
}

#[derive(Debug, Clone)]
pub struct TypingCorrectnessLine {
    pub segments: Vec<TypingCorrectnessSegment>,
}

#[derive(Debug, Clone)]
pub struct TypingCorrectnessSegment {
    pub chars: Vec<TypingCorrectnessChar>,
}


#[derive(Debug, Clone,PartialEq)]
pub enum TypingCorrectnessChar {
    Pending,
    Correct,
    Incorrect,
}

#[derive(Debug, Clone)]
pub struct TypingScroll {
    pub last_update: f64,
    pub scroll: f64,
    pub max: f64,
}

#[derive(Debug, Clone)]
pub struct TypingMetrics {
    pub miss_count: i32,      // タイプミス数
    pub type_count: i32,      // タイプ数（正解のみ）
    pub total_time: f64,      // 合計時間（ミリ秒）
    pub accuracy: f64,        // 正確さ（0.0 - 1.0）
    pub speed: f64,           // 速さ（タイプ/秒）
}