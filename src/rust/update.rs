// src/update.rs

use wasm_bindgen::prelude::*;
use js_sys::Date;
use serde::Serialize;
use crate::model::{Model, MenuModel, TypingModel, PauseModel, ResultModel};
use crate::msg::{Msg, MenuMsg, TypingMsg, PauseMsg, ResultMsg};


#[wasm_bindgen]
pub fn update(model_js: JsValue, msg_js: JsValue) -> Result<JsValue, JsValue> {
    // Deserialize the incoming model and message
    let model: Model = model_js.into_serde().map_err(|e| e.to_string())?;
    let msg: Msg = msg_js.into_serde().map_err(|e| e.to_string())?;
    
    // Determine the new state based on current model and message
    let updated_model = match (model, msg) {
        // When in Menu state and a Menu message is received:
        (Model::Menu(_menu_model), Msg::Menu(menu_msg)) => {
            match menu_msg {
                MenuMsg::SelectContent(content) => {
                    // Transition to Typing state with the selected content
                    Model::Typing(TypingModel {
                        content,
                        user_input: String::new(),
                        start_time: None,
                    })
                },
            }
        },
        // When in Typing state and a Typing message is received:
        (Model::Typing(mut typing_model), Msg::Typing(typing_msg)) => {
            match typing_msg {
                TypingMsg::StartTyping => {
                    typing_model.start_time = Some(Date::now());
                    typing_model.user_input.clear();
                    Model::Typing(typing_model)
                },
                TypingMsg::UpdateInput(new_input) => {
                    typing_model.user_input = new_input;
                    Model::Typing(typing_model)
                },
                TypingMsg::Pause => {
                    // Transition to Pause state
                    Model::Pause(PauseModel {
                        content: typing_model.content,
                        user_input: typing_model.user_input,
                        start_time: typing_model.start_time,
                    })
                },
                TypingMsg::Finish => {
                    // Transition to Result state, capturing the end time
                    Model::Result(ResultModel {
                        content: typing_model.content,
                        user_input: typing_model.user_input,
                        start_time: typing_model.start_time,
                        end_time: Some(Date::now()),
                    })
                },
                TypingMsg::Cancel => {
                    // Return to Menu state
                    Model::Menu(MenuModel {
                        available_contents: vec!["Sample text".to_string()],
                    })
                },
            }
        },
        // When in Pause state and a Pause message is received:
        (Model::Pause(pause_model), Msg::Pause(pause_msg)) => {
            match pause_msg {
                PauseMsg::Resume => {
                    // Resume Typing by transitioning back from Pause state
                    Model::Typing(TypingModel {
                        content: pause_model.content,
                        user_input: pause_model.user_input,
                        start_time: pause_model.start_time,
                    })
                },
                PauseMsg::Cancel => {
                    // Return to Menu state
                    Model::Menu(MenuModel {
                        available_contents: vec!["Sample text".to_string()],
                    })
                },
            }
        },
        // When in Result state and a Result message is received:
        (Model::Result(_result_model), Msg::Result(result_msg)) => {
            match result_msg {
                ResultMsg::BackToMenu => {
                    // Return to Menu state
                    Model::Menu(MenuModel {
                        available_contents: vec!["Sample text".to_string()],
                    })
                },
            }
        },
        // If the message does not match the current state, keep the model unchanged.
        (m, _) => m,
    };
    
    // Serialize and return the updated model.
    JsValue::from_serde(&updated_model).map_err(|e| e.to_string().into())
}

// ------------------------------------
// new_model function: initialization logic
// ------------------------------------

#[wasm_bindgen]
pub fn new_model() -> Result<JsValue, JsValue> {
    // Initialize with a Menu state and a default list of available contents.
    let menu_model = MenuModel {
        available_contents: vec!["Sample text".to_string()],
    };
    let model = Model::Menu(menu_model);
    JsValue::from_serde(&model).map_err(|e| e.to_string().into())
}
