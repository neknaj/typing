// src/jsapi.rs

use wasm_bindgen::prelude::*;
use js_sys::Date;
use serde::Serialize;
use serde::Deserialize;
use crate::model::{Model, MenuModel, TypingModel, PauseModel, ResultModel};
use crate::msg::{Msg, MenuMsg, TypingMsg, PauseMsg, ResultMsg};
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum FileGetResponse {
    Success { value: String },
    Failure { error: i32 },
}

#[wasm_bindgen(module = "/src/web/api.js")]
extern "C" {
    #[wasm_bindgen(js_name = file_get)]
    async fn file_get_js(file_path: &str) -> JsValue;
}

pub async fn file_get(file_path: &str) -> Result<String, i32> {
    // Call the JS function.
    let js_value = file_get_js(file_path).await;
    // Deserialize the JsValue into the FileGetResponse enum.
    let response: FileGetResponse = serde_wasm_bindgen::from_value(js_value)
        .map_err(|_| {
            // Use a custom error code for deserialization errors, e.g., 500.
            500
        })?;
    // Convert the deserialized response into a Rust Result.
    match response {
        FileGetResponse::Success { value, .. } => Ok(value),
        FileGetResponse::Failure { error, .. } => Err(error),
    }
}