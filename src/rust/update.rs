// update.rs

// resource manager
use std::sync::Mutex;
use std::collections::HashMap;
use crate::model::{Model, MenuModel, TypingStartModel, TypingModel, PauseModel, ResultModel, TypingStatus, TextConvert, ErrorMsg, KeyboardRemapping, TypingScroll,TypingSession};
use crate::msg::{Msg, MenuMsg, TypingStartMsg, TypingMsg, PauseMsg, ResultMsg};
use crate::parser::{parse_problem, Content};
use crate::typing;
use crate::typing::key_input;
use crate::timestamp::now;

lazy_static::lazy_static! {
    static ref Module_resource: Mutex<Model> = Mutex::new( Model::Empty );
}

#[macro_export]
macro_rules! jsvalue {
    ( $( $val:expr ),+ $(,)? ) => {{
         // Convert each expression to a serde_json::Value and collect them into a vector.
        let values: Vec<serde_json::Value> = vec![
            $( serde_json::to_value($val).unwrap() ),+
        ];
        // Serialize the vector into a JSON array string.
        serde_json::to_string(&values).unwrap()
    }};
}

pub fn update(model: Model, msg: Msg) -> Model {
    let updated_model = match (model, msg) {
        (Model::Menu(_menu_model), Msg::Menu(menu_msg)) => {
            match menu_msg {
                MenuMsg::MoveCursor(index) => {
                    Model::Menu({MenuModel {
                        selecting: index,
                        .._menu_model
                    }})
                },
                MenuMsg::Start => {
                    Model::TypingStart(TypingStartModel {
                        content: _menu_model.available_contents[_menu_model.selecting].clone(),
                        layout: _menu_model.layout,
                        available_contents: _menu_model.available_contents,
                        scroll_max: 0.0,
                    })
                },
                MenuMsg::AddContent(file_content) => {
                    let content = parse_problem(&file_content);
                    let mut new_contents = _menu_model.available_contents;
                    new_contents.push(content);
                    Model::Menu(MenuModel {
                        available_contents: new_contents,
                        .._menu_model
                    })
                },
            }
        },
        (Model::TypingStart(_typing_start_model), Msg::TypingStart(typing_start_msg)) => {
            match typing_start_msg {
                TypingStartMsg::StartTyping => {
                    Model::Typing(TypingModel {
                        content: _typing_start_model.clone().content,
                        typing_correctness: typing::create_typing_correctness_model(_typing_start_model.content),
                        user_input: vec![TypingSession {
                            line: 0,
                            inputs: Vec::new(),
                        }],
                        status: TypingStatus { line: 0, segment: 0, char_: 0, unconfirmed: Vec::new(), last_wrong_keydown: None },
                        available_contents: _typing_start_model.available_contents,
                        layout: _typing_start_model.layout,
                        keyboard_remapping: KeyboardRemapping {
                            mapping: HashMap::new(),
                        },
                        scroll: TypingScroll {
                            last_update: now(),
                            scroll: _typing_start_model.scroll_max,
                            max: _typing_start_model.scroll_max,
                        },
                    })
                },
                TypingStartMsg::Cancel => {
                    Model::Menu(MenuModel {
                        available_contents: _typing_start_model.available_contents,
                        selecting: 0,
                        layout: _typing_start_model.layout,
                        error_messages: vec![],
                    })
                },
                TypingStartMsg::ScrollMax(max) => {
                    Model::TypingStart(TypingStartModel {
                        scroll_max: max,
                        .._typing_start_model
                    })
                },
            }
        },
        (Model::Typing(typing_model), Msg::Typing(typing_msg)) => {
            match typing_msg {
                TypingMsg::KeyInput(input) => {
                    key_input(typing_model,input)
                },
                TypingMsg::Pause => {
                    Model::Pause(PauseModel {
                        typing_model,
                    })
                },
                TypingMsg::ScrollTo(input,max) => {
                    Model::Typing(TypingModel {
                        scroll: TypingScroll {
                            last_update: now(),
                            scroll: input,
                            max,
                        },
                        ..typing_model
                    })
                },
            }
        },
        (Model::Pause(pause_model), Msg::Pause(pause_msg)) => {
            match pause_msg {
                PauseMsg::Resume => {
                    // 一時停止から再開時に新しいセッションを開始
                    Model::Typing(typing::start_new_session(pause_model.typing_model))
                },
                PauseMsg::Cancel => {
                    Model::Result(ResultModel {
                        typing_model: pause_model.typing_model,
                    })
                },
            }
        },
        (Model::Result(_result_model), Msg::Result(result_msg)) => {
            match result_msg {
                ResultMsg::BackToMenu => {
                    Model::Menu(MenuModel {
                        available_contents: _result_model.typing_model.available_contents,
                        selecting: 0,
                        layout: _result_model.typing_model.layout,
                        error_messages: vec![],
                    })
                },
                ResultMsg::Retry => {
                    Model::TypingStart(TypingStartModel {
                        content: _result_model.typing_model.content,
                        layout: _result_model.typing_model.layout,
                        available_contents: _result_model.typing_model.available_contents,
                        scroll_max: 0.0,
                    })
                },
            }
        },
        (m, _) => m,
    };

    updated_model
}