// typing.rs

use crate::console_log;
use crate::model::TypingModel;
use crate::jsapi::console_log_js;
use crate::parser::Segment;

pub fn key_input(model_: TypingModel,input: String) -> TypingModel {
    console_log!("key_input", input);
    let remaining = match &model_.content.lines[model_.status.line as usize].segments[model_.status.segment as usize] {
        Segment::Plain { text } => text,
        Segment::Annotated { base, reading } => reading,
    };
    console_log!("remaining", remaining);

    // let expect = model_.layout.mapping
    model_
}