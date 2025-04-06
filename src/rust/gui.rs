use eframe::egui;
use crate::textrender::{RenderText, RenderLineWithRuby, CharOrientation};
use crate::parser::{parse_problem,Content,Line};


#[cfg(feature = "web")]
fn timestamp() -> f64 {
    web_sys::window()
    .expect("should have a window")
    .performance()
    .expect("performance should be available")
    .now()
}

#[cfg(not(feature = "web"))]
fn timestamp() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    // Calculate the duration since UNIX_EPOCH.
    let duration = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_millis() as f64
}

/// Sample application to demonstrate the usage of RenderChar, RenderText, and FPS calculation.
pub struct MyApp {
    name: String,
    age: u32,
    dark_mode: bool,
    fps: u32,
    frame_count: u32,                  // Count of frames within the 1-second interval
    last_fps_update: Option<f64>, // Timestamp (in milliseconds) when the frame count was last reset
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_string(),
            age: 42,
            dark_mode: true,
            fps: 0,
            frame_count: 0,
            last_fps_update: None,   // Initialize with None.
        }
    }
}

impl eframe::App for MyApp {
    // Setup function to initialize app settings
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let font = egui::FontId::new(150.0, egui::FontFamily::Proportional);

        // Side panel to toggle the theme.
        egui::SidePanel::left("side_panel")
        .frame(
            egui::Frame {
                fill: if self.dark_mode {
                    egui::Color32::from_rgb(6,12,22)
                } else {
                    egui::Color32::from_rgb(237, 238, 222)
                },
                inner_margin: egui::Margin {
                    left  : 20,
                    right : 20,
                    top   : 20,
                    bottom: 20,
                },
                ..Default::default()
            }
        ).show(ctx, |ui| {
            if ui.button("Toggle Theme").clicked() {
                self.dark_mode = !self.dark_mode;
                let visuals = if self.dark_mode {
                    egui::Visuals::dark()
                } else {
                    egui::Visuals::light()
                };
                ctx.set_visuals(visuals);
            }
        });

        // Get current time.
        let now = timestamp();

        // Frame count for FPS calculation:
        self.frame_count += 1;

        // Initialize last_fps_update if it's not set.
        if self.last_fps_update.is_none() {
            self.last_fps_update = Some(now);
        }

        if let Some(last_update) = self.last_fps_update {
            if now - last_update >= 1000.0 {
                // Update FPS, reset the frame count, and update the timestamp.
                self.fps = self.frame_count;
                self.frame_count = 0;
                self.last_fps_update = Some(now);
            }
        }

        egui::CentralPanel::default()
        .frame(
            egui::Frame {
                fill: if self.dark_mode {
                    egui::Color32::from_rgb(6,5,10)
                } else {
                    egui::Color32::from_rgb(243, 243, 253)
                },
                inner_margin: egui::Margin {
                    left  : 20,
                    right : 20,
                    top   : 20,
                    bottom: 20,
                },
                ..Default::default()
            }
        ).show(ctx, |ui| {
            ui.heading("My egui Application");
            // Display the calculated FPS.
            ui.label(format!("FPS: {:.1}", self.fps));
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.horizontal(|ui| {
                ui.label("Your age: ");
                ui.add(egui::DragValue::new(&mut self.age).speed(1.0));
            });
            ui.label(format!("Hello '{}', you are {} years old!", self.name, self.age));
            if ui.button("Reset").clicked() {
                *self = Self::default();
            }
            ui.separator();
            ui.heading("RenderText and RenderChar Samples");
            let sample_text = "#title いろは歌
(色/いろい)は(匂/にほ)へ/ど(散/ち)り/ぬる/を
(我/わ)が(世/よ)(誰/たれ)ぞ(常/つね)なら/む
(有為/うゐ)の(奥山/おくやま)(今日/けふ)(越/こ)え/て
(浅/あさ)き(夢/ゆめ)(見/み)じ(酔/ゑ)ひ/も/せ/ず";
            let sample_content: Content = parse_problem(sample_text);
            ui.add(RenderLineWithRuby::new(sample_content.lines[0].clone(),CharOrientation::Horizontal).with_font(font.clone()));
            ui.add(RenderLineWithRuby::new(sample_content.lines[1].clone(),CharOrientation::Horizontal).with_font(font.clone()));
            ui.add(RenderLineWithRuby::new(sample_content.lines[2].clone(),CharOrientation::Horizontal).with_font(font.clone()));
            ui.add(RenderLineWithRuby::new(sample_content.lines[3].clone(),CharOrientation::Horizontal).with_font(font.clone()));
            let sample_text = "#title いろは歌
(色/iro)は(匂/niho)へ/ど(散/ti)り/ぬる/を
(我/wa)が(世/yo)(誰/tare)ぞ(常/tune)なら/む
(有為/uwyi)の(奥山/okuyama)(今日/きょう)(越/ko)え/て
(浅/asa)き(夢/yume)(見/mi)じ(酔/wye)ひ/も/せ/ず";
            let sample_content: Content = parse_problem(sample_text);
            ui.add(RenderLineWithRuby::new(sample_content.lines[0].clone(),CharOrientation::Horizontal).with_font(font.clone()));
            ui.add(RenderLineWithRuby::new(sample_content.lines[1].clone(),CharOrientation::Horizontal).with_font(font.clone()));
            ui.add(RenderLineWithRuby::new(sample_content.lines[2].clone(),CharOrientation::Horizontal).with_font(font.clone()));
            ui.add(RenderLineWithRuby::new(sample_content.lines[3].clone(),CharOrientation::Horizontal).with_font(font.clone()));

            ui.separator();
            egui::Grid::new("layout_grid")
            .spacing([30.0, 30.0]) // Set spacing between grid cells.
            .show(ui, |ui| {
                // First grid cell with top_down layout.
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.add(RenderText::new("最近、OpenAI、Microsoft、そして SoftBank といった大手テック企業が、", CharOrientation::Horizontal).with_font(font.clone()));
                    ui.add(RenderText::new("革新的なAIおよびGPU技術を活用した新製品・サービスで大きく注目されています。", CharOrientation::Horizontal).with_font(font.clone()));
                    ui.add(RenderText::new("OpenAIは次世代モデル「GPT-4.5」の開発と大規模な資金調達を発表し、", CharOrientation::Horizontal).with_font(font.clone()));
                    ui.add(RenderText::new("MicrosoftはNVIDIAのH200 GPUを組み込んだCloud基盤の拡張を進め、", CharOrientation::Horizontal).with_font(font.clone()));
                    ui.add(RenderText::new("先進的なAI処理を加速させています。また、SoftBankは大阪の旧Sharp LCD工場を", CharOrientation::Horizontal).with_font(font.clone()));
                    ui.add(RenderText::new("最新鋭のAI Data Centerへと転換し、革新的なAI Agentモデルの商用展開を目指しています。", CharOrientation::Horizontal).with_font(font.clone()));
                    ui.separator();
                    ui.add(RenderText::new("ルビ付きの美しい横書き日本語を描画したい", CharOrientation::Horizontal).with_font(font.clone()));
                    ui.add(RenderText::new("横書きテキスト Horizontal Text", CharOrientation::Horizontal).with_font(font.clone()));
                    ui.add(RenderText::new("I want to write text that combines Japanese and English!", CharOrientation::Horizontal).with_font(font.clone()));
                    ui.separator();
                    ui.add(RenderText::new("色は匂へど　散りぬるを", CharOrientation::Horizontal).with_font(font.clone()));
                    ui.add(RenderText::new("我が世誰ぞ　常ならむ", CharOrientation::Horizontal).with_font(font.clone()));
                    ui.add(RenderText::new("有為の奥山　今日越えて", CharOrientation::Horizontal).with_font(font.clone()));
                    ui.add(RenderText::new("浅き夢見し　酔ひもせず", CharOrientation::Horizontal).with_font(font.clone()));
                });
                // Second grid cell with right_to_left layout.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.add(RenderText::new("最近、OpenAI、Microsoft、そして SoftBank といった大手テック企業が、", CharOrientation::Vertical).with_font(font.clone()));
                    ui.add(RenderText::new("革新的なAIおよびGPU技術を活用した新製品・サービスで大きく注目されています。", CharOrientation::Vertical).with_font(font.clone()));
                    ui.add(RenderText::new("OpenAIは次世代モデル「GPT-4.5」の開発と大規模な資金調達を発表し、", CharOrientation::Vertical).with_font(font.clone()));
                    ui.add(RenderText::new("MicrosoftはNVIDIAのH200 GPUを組み込んだCloud基盤の拡張を進め、", CharOrientation::Vertical).with_font(font.clone()));
                    ui.add(RenderText::new("先進的なAI処理を加速させています。また、SoftBankは大阪の旧Sharp LCD工場を", CharOrientation::Vertical).with_font(font.clone()));
                    ui.add(RenderText::new("最新鋭のAI Data Centerへと転換し、革新的なAI Agentモデルの商用展開を目指しています。", CharOrientation::Vertical).with_font(font.clone()));
                    ui.separator();
                    ui.add(RenderText::new("ルビ付きの美しい縦書き日本語を描画したい", CharOrientation::Vertical).with_font(font.clone()));
                    ui.add(RenderText::new("縦書きテキスト Vertical Text", CharOrientation::Vertical).with_font(font.clone()));
                    ui.add(RenderText::new("I want to write text that combines Japanese and English!", CharOrientation::Vertical).with_font(font.clone()));
                    ui.separator();
                    ui.add(RenderText::new("色は匂へど　散りぬるを", CharOrientation::Vertical).with_font(font.clone()));
                    ui.add(RenderText::new("我が世誰ぞ　常ならむ", CharOrientation::Vertical).with_font(font.clone()));
                    ui.add(RenderText::new("有為の奥山　今日越えて", CharOrientation::Vertical).with_font(font.clone()));
                    ui.add(RenderText::new("浅き夢見し　酔ひもせず", CharOrientation::Vertical).with_font(font.clone()));
                });
                ui.end_row(); // Ends the current row in the grid.
                // Additional rows/cells can be added similarly.
            });
            ctx.set_pixels_per_point(1.5);
        });
        // Request a repaint to continuously update the FPS.
        ctx.request_repaint();
    }
}
