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
// If the reading's length does not match the base text's length,
// treat the entire reading as a single annotation.
fn extract_ruby(segment: &Segment) -> (String, Vec<String>) {
    match segment {
        Segment::Plain { text } => {
            (text.clone(), text.chars().map(|_| "".to_string()).collect())
        }
        Segment::Annotated { base, reading } => {
            let base_chars: Vec<char> = base.chars().collect();
            let reading_chars: Vec<char> = reading.chars().collect();
            if base_chars.len() != reading_chars.len() {
                // Use the entire reading as one annotation.
                (base.clone(), vec![reading.clone()])
            } else {
                // Map each character of the base with the corresponding reading.
                let ruby_vec = base_chars
                    .iter()
                    .enumerate()
                    .map(|(i, _)| reading_chars[i].to_string())
                    .collect();
                (base.clone(), ruby_vec)
            }
        }
    }
}

/// Draw horizontal ruby text.
/// If a segment’s ruby annotation is provided as a single unit for a multi-character base,
/// draw the annotation centered above the entire base text.
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

        // If the ruby annotation is a single unit for multi-character base text.
        if ruby_texts.len() == 1 && base_text.chars().count() > 1 {
            // Calculate total width of the base text.
            let segment_start_x = x;
            let mut total_width = 0.0;
            for ch in base_text.chars() {
                let char_str = ch.to_string();
                let galley = ui.fonts(|fonts| {
                    fonts.layout_no_wrap(char_str, main_font.clone(), Color32::WHITE)
                });
                total_width += galley.size().x;
            }
            // Draw the base text character by character.
            for ch in base_text.chars() {
                let char_str = ch.to_string();
                let galley = ui.fonts(|fonts| {
                    fonts.layout_no_wrap(char_str.clone(), main_font.clone(), Color32::WHITE)
                });
                painter.text(
                    Pos2::new(x, y_main),
                    egui::Align2::LEFT_TOP,
                    char_str,
                    main_font.clone(),
                    Color32::WHITE,
                );
                x += galley.size().x;
            }
            // Draw the entire ruby annotation centered above the base text.
            let segment_center = segment_start_x + total_width / 2.0;
            let ruby = &ruby_texts[0];
            let ruby_galley = ui.fonts(|fonts| {
                fonts.layout_no_wrap(ruby.to_string(), ruby_font.clone(), Color32::GRAY)
            });
            let ruby_width = ruby_galley.size().x;
            let x_ruby = segment_center - ruby_width / 2.0;
            painter.text(
                Pos2::new(x_ruby, y_ruby),
                egui::Align2::LEFT_BOTTOM,
                ruby,
                ruby_font.clone(),
                Color32::GRAY,
            );
        } else {
            // Otherwise, perform per-character drawing.
            for (i, ch) in base_text.chars().enumerate() {
                let char_str = ch.to_string();
                // Measure the size of the main character.
                let galley = ui.fonts(|fonts| {
                    fonts.layout_no_wrap(char_str.clone(), main_font.clone(), Color32::WHITE)
                });
                let char_width = galley.size().x;
                // Use per-character ruby if available.
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
}

/// Draw vertical ruby text.
/// If a segment’s ruby annotation is provided as a single unit for a multi-character base,
/// draw the annotation once, centered to the right of the vertically arranged base text.
fn draw_vertical_ruby_text(ui: &egui::Ui, pos: Pos2, line: &Line) {
    let painter = ui.painter();
    let main_font = FontId::new(30.0, egui::FontFamily::Proportional);
    let ruby_font = FontId::new(15.0, egui::FontFamily::Proportional);

    let mut y = pos.y;

    // Iterate through each segment in the line.
    for segment in &line.segments {
        let (base_text, ruby_texts) = extract_ruby(segment);

        // Check if we have a single ruby annotation for a multi-character base text.
        if ruby_texts.len() == 1 && base_text.chars().count() > 1 {
            // Calculate total height for the base text.
            let segment_start_y = y;
            let mut total_height = 0.0;
            for ch in base_text.chars() {
                let char_str = ch.to_string();
                let galley = ui.fonts(|fonts| {
                    fonts.layout_no_wrap(char_str, main_font.clone(), Color32::WHITE)
                });
                total_height += galley.size().y;
            }
            // Draw the base text vertically.
            for ch in base_text.chars() {
                let char_str = ch.to_string();
                let galley = ui.fonts(|fonts| {
                    fonts.layout_no_wrap(char_str.clone(), main_font.clone(), Color32::WHITE)
                });
                painter.text(
                    Pos2::new(pos.x, y),
                    egui::Align2::LEFT_TOP,
                    char_str,
                    main_font.clone(),
                    Color32::WHITE,
                );
                y += galley.size().y;
            }
            // Split the ruby annotation into individual characters.
            let ruby = &ruby_texts[0];
            let ruby_chars: Vec<char> = ruby.chars().collect();
            // Calculate total height of the ruby characters.
            let mut total_ruby_height = 0.0;
            for r_ch in &ruby_chars {
                let r_str = r_ch.to_string();
                let r_galley = ui.fonts(|fonts| {
                    fonts.layout_no_wrap(r_str, ruby_font.clone(), Color32::GRAY)
                });
                total_ruby_height += r_galley.size().y;
            }
            // Compute starting y position for ruby so it is centered relative to the base text.
            let segment_center_y = segment_start_y + total_height / 2.0;
            let start_ruby_y = segment_center_y - total_ruby_height / 2.0;
            // Draw each ruby character vertically.
            let mut current_ruby_y = start_ruby_y;
            for r_ch in ruby_chars {
                let r_str = r_ch.to_string();
                let r_galley = ui.fonts(|fonts| {
                    fonts.layout_no_wrap(r_str.clone(), ruby_font.clone(), Color32::GRAY)
                });
                painter.text(
                    Pos2::new(pos.x + 30.0, current_ruby_y),
                    egui::Align2::LEFT_TOP,
                    r_str,
                    ruby_font.clone(),
                    Color32::GRAY,
                );
                current_ruby_y += r_galley.size().y;
            }
        } else {
            // Otherwise, perform per-character drawing.
            for (i, ch) in base_text.chars().enumerate() {
                let char_str = ch.to_string();
                let galley = ui.fonts(|fonts| {
                    fonts.layout_no_wrap(char_str.clone(), main_font.clone(), Color32::WHITE)
                });
                let char_height = galley.size().y - 10.0;
                let ruby = if i < ruby_texts.len() { &ruby_texts[i] } else { "" };
                let total_ruby_height: f32 = ruby.chars().map(|r_ch| {
                    let r_str = r_ch.to_string();
                    let r_galley = ui.fonts(|fonts| {
                        fonts.layout_no_wrap(r_str, ruby_font.clone(), Color32::GRAY)
                    });
                    r_galley.size().y
                }).sum();
                let start_ruby_y = y + (char_height - total_ruby_height) / 2.0;
                painter.text(
                    Pos2::new(pos.x, y),
                    egui::Align2::LEFT_TOP,
                    char_str,
                    main_font.clone(),
                    Color32::WHITE,
                );
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
                    painter.text(
                        Pos2::new(pos.x + galley.size().x + 2.0, start_ruby_y + 5.0),
                        egui::Align2::LEFT_TOP,
                        ruby,
                        ruby_font.clone(),
                        Color32::GRAY,
                    );
                }
                y += char_height;
            }
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

            // 横書きテキスト / Horizontal text
            let available_rect = ui.available_rect_before_wrap();
            let mut pos = available_rect.min + egui::vec2(10.0, 70.0);
            for line in &sample_content.lines {
                draw_horizontal_ruby_text(ui, pos, line);
                pos.y += 60.0; // Adjust spacing as needed.
            }
            // 縦書きテキスト / Vertical text
            let available_rect = ui.available_rect_before_wrap();
            let mut pos = available_rect.min + egui::vec2(240.0, 300.0);
            for line in &sample_content.lines {
                draw_vertical_ruby_text(ui, pos, line);
                pos.x -= 60.0; // Adjust spacing as needed.
            }
        });
    }
}
