// msg.rs

use serde::{Serialize, Deserialize};
use crate::parser::Content;

#[derive(Debug, Clone)]
pub enum MenuMsg {
    MoveCursor(usize),
    AddContent(String),
    Start
}

#[derive(Debug, Clone)]
pub enum TypingStartMsg {
    StartTyping,
    Cancel,
    ScrollMax(f64),
}

#[derive(Debug, Clone)]
pub enum TypingMsg {
    KeyInput(char),
    Pause,
    ScrollTo(f64,f64),
}

#[derive(Debug, Clone)]
pub enum PauseMsg {
    Resume,
    Cancel,
}

#[derive(Debug, Clone)]
pub enum ResultMsg {
    BackToMenu,
    Retry,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Menu(MenuMsg),
    Typing(TypingMsg),
    TypingStart(TypingStartMsg),
    Pause(PauseMsg),
    Result(ResultMsg),
}