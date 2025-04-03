// typing.rs

use crate::{console_log, debug};
use crate::model::TypingModel;
use crate::parser::Segment;

pub fn key_input(mut model_: TypingModel,input: char) -> TypingModel {
    debug! {
        console_log!("key_input", input);
    }
    let remaining_s = match &model_.content.lines[model_.status.line as usize].segments[model_.status.segment as usize] {
        Segment::Plain { text } => text,
        Segment::Annotated { base, reading } => reading,
    };
    let remaining = remaining_s.chars().collect::<Vec<char>>();
    debug! {
        console_log!("remaining", &remaining);
        console_log!("unconfirmed", &model_.status.unconfirmed);
    }

    let mut expect = Vec::new();
    for (key,value) in model_.layout.mapping.iter() {
        for v in value {
            // console_log!("key",key);
            let mut flag = true;
            for c in key.chars() {
                // console_log!(s.to_string());
                if c!=remaining_s.chars().nth(model_.status.char_ as usize).unwrap() {
                    flag = false;
                    break;
                }
            }
            if flag==false {
                continue;
            }
            for i in 0..model_.status.unconfirmed.len() {
                if model_.status.unconfirmed[i]!=v.chars().nth(i).unwrap() {
                    flag = false;
                    break;
                }
            }
            if flag {
                expect.push(v.chars().collect::<Vec<char>>());
            }
        }
    }
    debug! {
        console_log!(format!("expect {:?}",&expect));
    }
    let mut is_correct = false;
    let mut is_finished = false;
    for e in expect {
        if e[model_.status.unconfirmed.len()] == input {
            is_correct = true;
            model_.status.last_wrong_keydown = None;
            // expectに一致
            if e.len() == model_.status.unconfirmed.len() + 1 {
                // 完全一致
                // 1文字進める
                if remaining.len() == model_.status.char_ as usize + 1 {
                    if model_.content.lines[model_.status.line as usize].segments.len()==model_.status.segment as usize + 1 {
                        if model_.content.lines.len() == model_.status.line as usize + 1 {
                            // typing終了
                            model_.status.char_ = 0;
                            model_.status.segment = 0;
                            model_.status.line = 0;
                            model_.status.unconfirmed.clear();
                            is_finished = true;
                        } else {
                            // lineを進める
                            model_.status.char_ = 0;
                            model_.status.segment = 0;
                            model_.status.line += 1;
                            model_.status.unconfirmed.clear();
                        }
                    } else {
                        // segmentを進める
                        model_.status.char_ = 0;
                        model_.status.segment += 1;
                        model_.status.unconfirmed.clear();
                    }
                } else {
                    // charを進める
                    model_.status.char_ += 1;
                    model_.status.unconfirmed.clear();
                }
            } else {
                // 一致したが、まだ続く
                model_.status.unconfirmed.push(e[model_.status.unconfirmed.len()]);
            }
            break;
        }
        else {
            model_.status.last_wrong_keydown = Some(input);
        }
    }
    debug! {
        console_log!("remaining", remaining);
        console_log!("unconfirmed", &model_.status.unconfirmed);
        console_log!(&model_.status.segment);
        console_log!(&model_.status.char_);
    }

    model_
}