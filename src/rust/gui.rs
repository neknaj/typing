use eframe::egui::{self, Color32, FontId, Pos2};
use serde::{Deserialize, Serialize};
use crate::parser::{Content, Line, Segment, parse_problem};

// アプリケーションの構造体 / Application struct
pub struct MyApp {}

// MyAppのデフォルト実装 / Default implementation of MyApp
impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

// Helper function to extract base text and ruby texts.
// It checks if the base text is a single character. If so, the entire reading is kept.
fn extract_ruby(segment: &Segment) -> (String, Vec<String>) {
    match segment {
        Segment::Plain { text } => {
            (text.clone(), text.chars().map(|_| "".to_string()).collect::<Vec<_>>())
        }
        Segment::Annotated { base, reading } => {
            let base_chars: Vec<char> = base.chars().collect();
            if base_chars.len() == 1 {
                // For single-character base, keep the entire reading.
                (base.clone(), vec![reading.clone()])
            } else {
                // For multiple-character base, split the reading by character.
                let reading_chars: Vec<char> = reading.chars().collect();
                let mut ruby_vec = Vec::new();
                for i in 0..base_chars.len() {
                    let ruby = if i < reading_chars.len() {
                        reading_chars[i].to_string()
                    } else {
                        "".to_string()
                    };
                    ruby_vec.push(ruby);
                }
                (base.clone(), ruby_vec)
            }
        }
    }
}

/// Draw horizontal ruby text.
/// This function iterates through each segment in the provided line and renders
/// both the main text and its ruby annotations.
fn draw_horizontal_ruby_text(ui: &egui::Ui, pos: Pos2, line: &Line) {
    let painter = ui.painter();
    let main_font = FontId::new(30.0, egui::FontFamily::Proportional);
    let ruby_font = FontId::new(15.0, egui::FontFamily::Proportional);

    let mut x = pos.x;
    let y_ruby = pos.y;
    let y_main = pos.y - 7.0;

    // Iterate through each segment in the line.
    for segment in &line.segments {
        let (base_text, ruby_texts) = extract_ruby(segment);

        // Draw each character with its ruby annotation.
        for (i, ch) in base_text.chars().enumerate() {
            let char_str = ch.to_string();
            // Measure the size of the main character.
            let galley = ui.fonts(|fonts| {
                fonts.layout_no_wrap(char_str.clone(), main_font.clone(), Color32::WHITE)
            });
            let char_width = galley.size().x;
            // Use the entire ruby text (if exists) for the current character.
            let ruby = if i < ruby_texts.len() { &ruby_texts[i] } else { "" };
            // Measure the size of the ruby text.
            let ruby_galley = ui.fonts(|fonts| {
                fonts.layout_no_wrap(ruby.to_string(), ruby_font.clone(), Color32::GRAY)
            });
            let ruby_width = ruby_galley.size().x;
            // Center the ruby text above the main text.
            let x_ruby = x + (char_width - ruby_width) / 2.0;
            // Draw the main character.
            painter.text(
                Pos2::new(x, y_main),
                egui::Align2::LEFT_TOP,
                char_str,
                main_font.clone(),
                Color32::WHITE,
            );
            // Draw the ruby annotation.
            painter.text(
                Pos2::new(x_ruby, y_ruby),
                egui::Align2::LEFT_BOTTOM,
                ruby,
                ruby_font.clone(),
                Color32::GRAY,
            );
            x += char_width;
        }
    }
}

/// Draw vertical ruby text.
/// This function iterates over each segment in the line and draws the main text vertically
/// with its ruby annotation rendered to the right.
fn draw_vertical_ruby_text(ui: &egui::Ui, pos: Pos2, line: &Line) {
    let painter = ui.painter();
    let main_font = FontId::new(30.0, egui::FontFamily::Proportional);
    let ruby_font = FontId::new(15.0, egui::FontFamily::Proportional);

    // Initialize vertical starting position.
    let mut y = pos.y;

    // Process each segment in the provided line.
    for segment in &line.segments {
        let (base_text, ruby_texts) = extract_ruby(segment);

        // Draw each character with its ruby annotation vertically.
        for (i, ch) in base_text.chars().enumerate() {
            let char_str = ch.to_string();
            // Measure the size of the main character.
            let galley = ui.fonts(|fonts| {
                fonts.layout_no_wrap(char_str.clone(), main_font.clone(), Color32::WHITE)
            });
            let char_height = galley.size().y-10.0;
            // Obtain the ruby text for the current character.
            let ruby = if i < ruby_texts.len() { &ruby_texts[i] } else { "" };
            // Calculate the total height of the ruby annotation.
            let total_ruby_height: f32 = ruby.chars().map(|r_ch| {
                let r_str = r_ch.to_string();
                let r_galley = ui.fonts(|fonts| {
                    fonts.layout_no_wrap(r_str, ruby_font.clone(), Color32::GRAY)
                });
                r_galley.size().y
            }).sum();
            // Compute starting y position for ruby so it is centered relative to the main text.
            let start_ruby_y = y + (char_height - total_ruby_height) / 2.0;
            // Draw the main character.
            painter.text(
                Pos2::new(pos.x, y),
                egui::Align2::LEFT_TOP,
                char_str,
                main_font.clone(),
                Color32::WHITE,
            );
            // If the ruby text has multiple characters, draw them vertically.
            if ruby.chars().count() > 1 {
                let mut current_ruby_y = start_ruby_y;
                for r_ch in ruby.chars() {
                    let r_str = r_ch.to_string();
                    let r_galley = ui.fonts(|fonts| {
                        fonts.layout_no_wrap(r_str.clone(), ruby_font.clone(), Color32::GRAY)
                    });
                    painter.text(
                        Pos2::new(pos.x + galley.size().x + 2.0, current_ruby_y + 5.0),
                        egui::Align2::LEFT_TOP,
                        r_str,
                        ruby_font.clone(),
                        Color32::GRAY,
                    );
                    current_ruby_y += r_galley.size().y;
                }
            } else {
                // Otherwise, draw the ruby text as a whole.
                painter.text(
                    Pos2::new(pos.x + galley.size().x + 2.0, start_ruby_y + 5.0),
                    egui::Align2::LEFT_TOP,
                    ruby,
                    ruby_font.clone(),
                    Color32::GRAY,
                );
            }
            // Increment y position for the next character.
            y += char_height;
        }
    }
}

// eframeアプリケーションの更新処理 / Update function for the eframe application
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ヘッダーと説明ラベルを表示 / Display header and label
            ui.heading("ルビ付き日本語テキストのサンプル");
            ui.label("下記は横書きと縦書きのルビ付きテキストです。");

            let sample_text = "#title いろは歌
(色/いろ)は(匂/にほ)へ/ど(散/ち)り/ぬる/を
(我/わ)が(世/よ)(誰/たれ)ぞ(常/つね)なら/む
(有為/うゐ)の(奥山/おくやま)(今日/けふ)(越/こ)え/て
(浅/あさ)き(夢/ゆめ)(見/み)じ(酔/ゑ)ひ/も/せ/ず";
            let sample_content: Content = parse_problem(sample_text);

            // 横書きテキスト
            let available_rect = ui.available_rect_before_wrap();
            // Set initial offset for horizontal text.
            let mut pos = available_rect.min + egui::vec2(10.0, 70.0);
            // Iterate over all lines and draw them horizontally.
            for line in &sample_content.lines {
                draw_horizontal_ruby_text(ui, pos, line);
                // Update vertical offset for the next line.
                pos.y += 60.0; // Adjust the spacing as needed.
            }
            // 縦書きテキスト
            let available_rect = ui.available_rect_before_wrap();
            // Set initial offset for vertical text.
            let mut pos = available_rect.min + egui::vec2(240.0, 300.0);
            // Iterate over all lines and draw them vertically.
            for line in &sample_content.lines {
                draw_vertical_ruby_text(ui, pos, line);
                // Update horizontal offset for the next vertical line.
                pos.x -= 60.0; // Adjust the spacing as needed.
            }
        });
    }
}
