// update.rs

use wasm_bindgen::prelude::*;
use js_sys::Date;
use serde::Serialize;
use serde::Deserialize;
use crate::console_log;
use crate::model::{Model, MenuModel, TypingStartModel, TypingModel, PauseModel, ResultModel, TypingStatus};
use crate::msg::{Msg, MenuMsg, TypingStartMsg, TypingMsg, PauseMsg, ResultMsg};
use crate::jsapi::{file_get};
use crate::parser::{parse_problem, Content};
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use ts_rs::TS;

#[wasm_bindgen]
pub fn update(model_js: JsValue, msg_js: JsValue) -> Result<JsValue, JsValue> {
    let model: Model = model_js.into_serde().map_err(|e| e.to_string())?;
    let msg: Msg = msg_js.into_serde().map_err(|e| e.to_string())?;

    let updated_model = match (model, msg) {
        (Model::Menu(_menu_model), Msg::Menu(menu_msg)) => {
            match menu_msg {
                MenuMsg::SelectContent(content) => {
                    Model::TypingStart(TypingStartModel {
                        content,
                        available_contents: _menu_model.available_contents,
                    })
                },
            }
        },
        (Model::TypingStart(_typing_start_model), Msg::TypingStart(typing_start_msg)) => {
            match typing_start_msg {
                TypingStartMsg::StartTyping => {
                    Model::Typing(TypingModel {
                        content: _typing_start_model.content,
                        user_input: vec![],
                        status: TypingStatus { line: 0, segment: 0, char_: 0 },
                        available_contents: _typing_start_model.available_contents,
                    })
                },
                TypingStartMsg::Cancel => {
                    Model::Menu(MenuModel {
                        available_contents: _typing_start_model.available_contents,
                        error_messages: vec![],
                    })
                },
            }
        },
        (Model::Typing(mut typing_model), Msg::Typing(typing_msg)) => {
            match typing_msg {
                TypingMsg::UpdateInput(new_input) => {
                    // typing_model.user_input = new_input;
                    Model::Typing(typing_model)
                },
                TypingMsg::Pause => {
                    Model::Pause(PauseModel {
                        typing_model,
                    })
                },
                TypingMsg::Finish => {
                    Model::Result(ResultModel {
                        typing_model: typing_model.clone(),
                        // start_time: typing_model.start_time,
                        // end_time: Some(Date::now()),
                        // pause_time: None,
                    })
                },
                TypingMsg::Cancel => {
                    Model::Menu(MenuModel {
                        available_contents: typing_model.available_contents,
                        error_messages: vec![],
                    })
                },
                TypingMsg::Tick => {
                    Model::Typing(typing_model)
                },
            }
        },
        (Model::Pause(pause_model), Msg::Pause(pause_msg)) => {
            match pause_msg {
                PauseMsg::Resume => {
                    Model::Typing(pause_model.typing_model)
                },
                PauseMsg::Cancel => {
                    Model::Menu(MenuModel {
                        available_contents: pause_model.typing_model.available_contents,
                        error_messages: vec![],
                    })
                },
            }
        },
        (Model::Result(_result_model), Msg::Result(result_msg)) => {
            match result_msg {
                ResultMsg::BackToMenu => {
                    Model::Menu(MenuModel {
                        available_contents: _result_model.typing_model.available_contents,
                        error_messages: vec![],
                    })
                },
            }
        },
        (m, _) => m,
    };

    JsValue::from_serde(&updated_model).map_err(|e| e.to_string().into())
}

#[wasm_bindgen]
pub async fn new_model() -> Result<JsValue, JsValue> {
    match file_get("./examples/iroha.ntq").await {
        Ok(file_content) => {
            let content = parse_problem(&file_content);
            let menu_model = MenuModel {
                available_contents: vec![content],
                error_messages: vec![],
            };
            let model = Model::Menu(menu_model);
            JsValue::from_serde(&model).map_err(|e| e.to_string().into())
        },
        Err(error_code) => {
            let menu_model = MenuModel {
                available_contents: vec![],
                error_messages: vec![],
            };
            let model = Model::Menu(menu_model);
            JsValue::from_serde(&model).map_err(|e| e.to_string().into())
        }
    }
}