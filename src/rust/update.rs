// update.rs

use std::fmt::format;
// resource manager
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use js_sys::Date;
use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;
use crate::console_log;
use crate::jsapi;
use crate::model::{Model, MenuModel, TypingStartModel, TypingModel, PauseModel, ResultModel, TypingStatus, TextConvert, ErrorMsg, KeyboardRemapping, TextOrientation};
use crate::msg::{Msg, MenuMsg, TypingStartMsg, TypingMsg, PauseMsg, ResultMsg};
use crate::jsapi::{*};
use crate::parser::{parse_problem, Content};
use crate::typing;
use crate::typing::key_input;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use ts_rs::TS;

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
                }
                MenuMsg::Start => {
                    Model::TypingStart(TypingStartModel {
                        content: _menu_model.available_contents[_menu_model.selecting].clone(),
                        layout: _menu_model.layout,
                        text_orientation: _menu_model.text_orientation,
                        available_contents: _menu_model.available_contents,
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
                        user_input: vec![],
                        status: TypingStatus { line: 0, segment: 0, char_: 0, unconfirmed: Vec::new(), last_wrong_keydown: None },
                        available_contents: _typing_start_model.available_contents,
                        layout: _typing_start_model.layout,
                        text_orientation: _typing_start_model.text_orientation,
                        keyboard_remapping: KeyboardRemapping {
                            mapping: HashMap::new(),
                        },
                    })
                },
                TypingStartMsg::Cancel => {
                    Model::Menu(MenuModel {
                        available_contents: _typing_start_model.available_contents,
                        selecting: 0,
                        layout: _typing_start_model.layout,
                        text_orientation: _typing_start_model.text_orientation,
                        error_messages: vec![],
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
                        text_orientation: _result_model.typing_model.text_orientation,
                        error_messages: vec![],
                    })
                },
                ResultMsg::Retry => {
                    Model::TypingStart(TypingStartModel {
                        content: _result_model.typing_model.content,
                        layout: _result_model.typing_model.layout,
                        text_orientation: _result_model.typing_model.text_orientation,
                        available_contents: _result_model.typing_model.available_contents,
                    })
                },
            }
        },
        (m, _) => m,
    };

    updated_model
}

#[wasm_bindgen]
pub async fn init_model(to: JsValue) {
    let text_orientation: TextOrientation = to.into_serde().unwrap();
    let json_str = file_get("./layouts/japanese.json").await.unwrap();
    // JSONをHashMapとして一旦パース
    let map: HashMap<String, Vec<String>> = serde_json::from_str(&json_str).unwrap();

    // HashMapをVec<(String, String)>に変換
    let layout: Vec<(String, Vec<String>)> = map.into_iter().collect();

    let mut menu_model = MenuModel {
        available_contents: vec![],
        text_orientation,
        selecting: 0,
        error_messages: vec![],
        layout: TextConvert { mapping: layout },
    };

    // サンプルファイルを追加
    menu_model.available_contents.push(parse_problem(&file_get("./examples/iroha.ntq").await.unwrap()));
    menu_model.available_contents.push(parse_problem(&file_get("./examples/五十音.ntq").await.unwrap()));

    let model = Model::Menu(menu_model.clone());
    *Module_resource.lock().unwrap() = model;
}

#[wasm_bindgen]
pub async fn add_contents(data: JsValue) {
    let content: String = data.into_serde().unwrap();
    let mut model = Module_resource.lock().unwrap();
    match *model {
        Model::Menu(ref menu_model) => {
            *model = update(model.clone(), Msg::Menu(MenuMsg::AddContent(content)));
        },
        _ => {}
    }
}

#[wasm_bindgen]
pub fn event_receive_keyboard(event: JsValue) {
    let key: String = event.into_serde().unwrap();
    console_log!(format!("key event {:?}",key));
    let mut model = Module_resource.lock().unwrap();
    match *model {
        Model::Menu(ref menu_model) => {
            match (key.as_str(),menu_model.text_orientation.clone()) {
                ("ArrowLeft",TextOrientation::Vertical) | ("ArrowDown",TextOrientation::Horizontal) => {
                    if menu_model.selecting<menu_model.available_contents.len()-1 {
                        *model = update(model.clone(), Msg::Menu(MenuMsg::MoveCursor(menu_model.selecting+1)));
                    }
                },
                ("ArrowRight",TextOrientation::Vertical) | ("ArrowUp",TextOrientation::Horizontal) => {
                    if menu_model.selecting>0 {
                        *model = update(model.clone(), Msg::Menu(MenuMsg::MoveCursor(menu_model.selecting-1)));
                    }
                },
                (" ",_) => {
                    *model = update(model.clone(), Msg::Menu(MenuMsg::Start));
                }
                _ => {
                }
            }
        },
        Model::TypingStart(ref typing_start_model) => {
            match key.as_str() {
                " " => {
                    *model = update(model.clone(), Msg::TypingStart(TypingStartMsg::StartTyping));
                },
                "Escape" => {
                    *model = update(model.clone(), Msg::TypingStart(TypingStartMsg::Cancel));
                },
                _ => {
                }
            }
        },
        Model::Typing(ref typing_model) => {
            match key.as_str() {
                "Escape" => {
                    *model = update(model.clone(), Msg::Typing(TypingMsg::Pause));
                },
                key if key.chars().collect::<Vec<char>>().len()==1 => {
                    *model = update(model.clone(), Msg::Typing(TypingMsg::KeyInput(key.chars().nth(0).unwrap())));
                }
                _ => {
                }
            }
        },
        Model::Pause(ref pause_model) => {
        },
        Model::Result(ref result_model) => {
        },
        Model::Empty => {
        }
    }
}

#[wasm_bindgen]
pub fn fetch_render_data() -> String {
    let mut model = Module_resource.lock().unwrap();
    match *model {
        Model::Menu(ref scene_model) => {
            let menu: Vec<String> = scene_model.available_contents
                .iter()
                .map(|content| content.title.clone())
                .collect();
            jsvalue!("Menu",scene_model.selecting,menu)
        },
        Model::TypingStart(ref scene_model) => {
            jsvalue!("TypingStart",&scene_model.content.title)
        },
        Model::Typing(ref scene_model) => {
            jsvalue!("Typing",&scene_model.content.title,&scene_model.content.lines[scene_model.status.line as usize].segments,&scene_model.typing_correctness.lines[scene_model.status.line as usize].segments,&scene_model.status,&scene_model.text_orientation)
        },
        _ => jsvalue!("Other")
    }
}