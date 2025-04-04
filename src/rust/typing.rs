// typing.rs

use crate::{console_log, debug};
use crate::model::{Model, TypingModel, ResultModel, TypingCorrectnessContent, TypingSession, TypingCorrectnessLine, TypingCorrectnessSegment, TypingCorrectnessChar};
use crate::parser::{Content, Line, Segment};

pub fn key_input(mut model_: TypingModel, input: char) -> Model {
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
    for (key, values) in model_.layout.mapping.iter() {
        for v in values {
            let mut flag = true;
            let start_index = model_.status.char_ as usize;
            for (i, c) in key.chars().enumerate() {
                if let Some(rs_char) = remaining_s.chars().nth(start_index + i) {
                    if c != rs_char {
                        flag = false;
                        break;
                    }
                } else {
                    flag = false;
                    break;
                }
            }
            if !flag {
                continue;
            }
            for i in 0..model_.status.unconfirmed.len() {
                if model_.status.unconfirmed[i] != v.chars().nth(i).unwrap() {
                    flag = false;
                    break;
                }
            }
            if flag {
                expect.push((key,v.chars().collect::<Vec<char>>()));
            }
        }
    }
    debug! {
        console_log!(format!("expect {:?}", &expect));
    }
    let mut is_correct = false;
    let mut is_finished = false;
    for (key,e) in expect {
        if e[model_.status.unconfirmed.len()] == input {
            is_correct = true;
            model_.status.last_wrong_keydown = None;
            // expectに一致
            if e.len() == model_.status.unconfirmed.len() + 1 {
                // 完全一致時、typing_correctnessを更新
                let char_pos = model_.status.char_ as usize;
                let segment = &mut model_.typing_correctness.lines[model_.status.line as usize].segments[model_.status.segment as usize];
                let mut flag = false;
                for i in 0..key.chars().collect::<Vec<char>>().len() {
                    if segment.chars[char_pos+i] == TypingCorrectnessChar::Incorrect {
                        flag = true;
                    }
                }
                for i in 0..key.chars().collect::<Vec<char>>().len() {
                    if !flag {
                        segment.chars[char_pos+i] = TypingCorrectnessChar::Correct;
                    }
                    else {
                        segment.chars[char_pos+i] = TypingCorrectnessChar::Incorrect;
                    }
                }

                // 1文字進める
                if remaining.len() == model_.status.char_ as usize + key.chars().collect::<Vec<char>>().len() {
                    if model_.content.lines[model_.status.line as usize].segments.len() == model_.status.segment as usize + 1 {
                        if model_.content.lines.len() == model_.status.line as usize + 1 {
                            // typing終了
                            model_.status.char_ = 0;
                            model_.status.segment = 0;
                            model_.status.line += 1;
                            model_.status.unconfirmed.clear();
                            is_finished = true;
                        } else {
                            // lineを進める
                            model_.status.char_ = 0;
                            model_.status.segment = 0;
                            model_.status.line += 1;
                            model_.status.unconfirmed.clear();
                            model_.scroll.scroll = model_.scroll.max;
                        }
                    } else {
                        // segmentを進める
                        model_.status.char_ = 0;
                        model_.status.segment += 1;
                        model_.status.unconfirmed.clear();
                    }
                } else {
                    // charを進める
                    model_.status.char_ += key.chars().collect::<Vec<char>>().len() as i32;
                    model_.status.unconfirmed.clear();
                }
            } else {
                // 一致したが、まだ続く
                model_.status.unconfirmed.push(e[model_.status.unconfirmed.len()]);
            }
            break;
        }
    }
    if !is_correct {
        model_.status.last_wrong_keydown = Some(input);
        // 不正解時、typing_correctnessを更新
        let char_pos = model_.status.char_ as usize;
        let segment = &mut model_.typing_correctness.lines[model_.status.line as usize].segments[model_.status.segment as usize];
        segment.chars[char_pos] = TypingCorrectnessChar::Incorrect;
    }
    debug! {
        console_log!("remaining", remaining);
        console_log!("unconfirmed", &model_.status.unconfirmed);
        console_log!(&model_.status.segment);
        console_log!(&model_.status.char_);
    }

    if is_finished {
        Model::Result(ResultModel {
            typing_model: model_,
        })
    } else {
        Model::Typing(model_)
    }
}

// typing正誤の記録をpending状態で新規作成する関数
pub fn create_typing_correctness_model(content: Content) -> TypingCorrectnessContent {
    let mut lines = Vec::new();
    // ContentのLine構造に合わせてTypingCorrectnessLineを作成
    for line in content.lines {
        let mut segments = Vec::new();
        // 各セグメントを処理
        for segment in line.segments {
            let target_text = match segment {
                Segment::Plain { text } => text,
                Segment::Annotated { base: _, reading } => reading,
            };
            // 文字ごとにPending状態で初期化
            let chars = target_text.chars()
                .map(|_| TypingCorrectnessChar::Pending)
                .collect();
            segments.push(TypingCorrectnessSegment { chars });
        }
        lines.push(TypingCorrectnessLine { segments });
    }
    TypingCorrectnessContent { lines }
}