// typing.rs

use crate::{console_log, debug};
use crate::model::TypingModel;
use crate::parser::Segment;

pub fn key_input(model_: TypingModel,input: String) -> TypingModel {
    debug! {
        console_log!("key_input", input);
    }
    let remaining = match &model_.content.lines[model_.status.line as usize].segments[model_.status.segment as usize] {
        Segment::Plain { text } => text,
        Segment::Annotated { base, reading } => reading,
    };
    debug! {
        console_log!("remaining", remaining);
    }

    let mut expect = Vec::new();
    for (key,value) in model_.layout.mapping.iter() {
        // console_log!("key",key);
        let mut flag = true;
        for c in key.chars() {
            // console_log!(s.to_string());
            if c!=remaining.chars().nth(model_.status.char_ as usize + model_.status.unconfirmed.len()).unwrap() {
                flag = false;
                break;
            }
        }
        if flag {
            expect.push(value);
        }
    }
    debug! {
        console_log!("expect", expect);
    }

    model_
}