// typing.rs

use crate::model::{Model, TypingModel, ResultModel, TypingCorrectnessContent, TypingSession, TypingInput, TypingCorrectnessLine, TypingCorrectnessSegment, TypingCorrectnessChar, TypingMetrics};
use crate::parser::{Content, Line, Segment};
use crate::timestamp::now;

pub fn key_input(mut model_: TypingModel, input: char) -> Model {
    #[cfg(target_arch = "wasm32")]
    let current_time = js_sys::Date::now();
    #[cfg(not(target_arch = "wasm32"))]
    let current_time = now();
    let current_line = model_.status.line;
    
    // 新しいセッションを開始するかどうかを判断
    let should_start_new_session = if model_.user_input.is_empty() {
        true
    } else {
        let last_session = model_.user_input.last().unwrap();
        if let Some(last_input) = last_session.inputs.last() {
            // 前回の入力から1秒以上経過している場合は新しいセッション
            (current_time - last_input.timestamp) > 1000.0
        } else {
            true
        }
    };

    if should_start_new_session {
        model_.user_input.push(TypingSession {
            line: current_line,
            inputs: Vec::new(),
        });
    }

    let current_session = model_.user_input.last_mut().unwrap();
    let current_line = model_.status.line;

    let remaining_s = match &model_.content.lines[model_.status.line as usize].segments[model_.status.segment as usize] {
        Segment::Plain { text } => text,
        Segment::Annotated { base, reading } => reading,
    };
    let remaining = remaining_s.chars().collect::<Vec<char>>();

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

    // 入力履歴を記録
    current_session.inputs.push(TypingInput {
        key: input,
        timestamp: current_time,
        is_correct,
    });

    // line変更時に新しいセッションを開始
    if current_line != model_.status.line {
        model_.user_input.push(TypingSession {
            line: model_.status.line,
            inputs: Vec::new(),
        });
    }

    if !is_correct {
        model_.status.last_wrong_keydown = Some(input);
        // 不正解時、typing_correctnessを更新
        let char_pos = model_.status.char_ as usize;
        let segment = &mut model_.typing_correctness.lines[model_.status.line as usize].segments[model_.status.segment as usize];
        segment.chars[char_pos] = TypingCorrectnessChar::Incorrect;
    }

    if is_finished {
        Model::Result(ResultModel {
            typing_model: model_,
        })
    } else {
        Model::Typing(model_)
    }
}

// 一時停止から再開時の新しいセッション開始用の関数を追加
pub fn start_new_session(mut typing_model: TypingModel) -> TypingModel {
    typing_model.user_input.push(TypingSession {
        line: typing_model.status.line,
        inputs: Vec::new(),
    });
    typing_model
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


impl TypingMetrics {
    fn new() -> Self {
        TypingMetrics {
            miss_count: 0,
            type_count: 0,
            total_time: 0.0,
            accuracy: 0.0,
            speed: 0.0,
        }
    }

    fn calculate(&mut self) {
        // 正確さと速さの計算
        if self.type_count > 0 {
            self.accuracy = 1.0 - (self.miss_count as f64 / self.type_count as f64);
        }
        if self.total_time > 0.0 {
            self.speed = (self.type_count as f64) / (self.total_time / 1000.0); // 秒あたりのタイプ数
        }
    }
}

/// 特定の行のタイピング統計を計算
pub fn calculate_line_metrics(model: &TypingModel, line: i32) -> TypingMetrics {
    let mut metrics = TypingMetrics::new();
    
    // 指定された行のセッションを取得
    let line_sessions: Vec<&TypingSession> = model.user_input.iter()
        .filter(|session| session.line == line)
        .collect();

    for session in line_sessions {
        let mut consecutive_errors = 0;
        
        if let (Some(first), Some(last)) = (session.inputs.first(), session.inputs.last()) {
            metrics.total_time += last.timestamp - first.timestamp;
        }

        for input in &session.inputs {
            if input.is_correct {
                metrics.type_count += 1;
                consecutive_errors = 0;
            } else {
                consecutive_errors += 1;
                if consecutive_errors == 1 {  // 連続エラーの最初のみカウント
                    metrics.miss_count += 1;
                }
            }
        }
    }

    metrics.calculate();
    metrics
}

/// 全体のタイピング統計を計算
pub fn calculate_total_metrics(model: &TypingModel) -> TypingMetrics {
    let mut metrics = TypingMetrics::new();

    for session in &model.user_input {
        let mut consecutive_errors = 0;
        
        if let (Some(first), Some(last)) = (session.inputs.first(), session.inputs.last()) {
            metrics.total_time += last.timestamp - first.timestamp;
        }

        for input in &session.inputs {
            if input.is_correct {
                metrics.type_count += 1;
                consecutive_errors = 0;
            } else {
                consecutive_errors += 1;
                if consecutive_errors == 1 {  // 連続エラーの最初のみカウント
                    metrics.miss_count += 1;
                }
            }
        }
    }

    metrics.calculate();
    metrics
}