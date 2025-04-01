// src/jsapi.rs

use wasm_bindgen::prelude::*;
use serde::Deserialize;

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

/**
 * Asynchronously fetches the content of a file from the given path using JavaScript's fetch API.
 *
 * # Arguments
 * - `file_path`: A string slice that represents the public URL path of the file.
 *
 * # Returns
 * - `Ok(String)` containing the file content if successful.
 * - `Err(i32)` containing an error code (e.g., HTTP status code or 500 for deserialization failure) on error.
 * This function calls the JavaScript `file_get_js` function.  
 * If deserialization fails, it returns an error code of 500.
 *
 * # Example
 * ```rust
 * let result = file_get("./file.txt").await;
 * match result {
 *     Ok(content) => println!("File content: {}", content),
 *     Err(code) => eprintln!("Error occurred, code: {}", code),
 * }
 * ```
 */
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