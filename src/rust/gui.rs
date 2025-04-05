use eframe::egui;
use std::time::{Duration, Instant};
use crate::textrender::{RenderText, CharOrientation};

/// Sample application to demonstrate the usage of RenderChar, RenderText, and FPS calculation.
pub struct MyApp {
    name: String,
    age: u32,
    dark_mode: bool,
    fps: u32,
    frame_count: u32,                  // Count of frames within the 1-second interval
    last_fps_update: Option<Instant>,  // Timestamp when the frame count was last reset
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let font = egui::FontId::new(50.0, egui::FontFamily::Proportional);

        // Side panel to toggle the theme.
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
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
        let now = Instant::now();

        // Frame count for FPS calculation:
        self.frame_count += 1;

        // Initialize last_fps_update if it's not set.
        if self.last_fps_update.is_none() {
            self.last_fps_update = Some(now);
        }

        // Check if one second has elapsed since the last FPS update.
        if let Some(last_update) = self.last_fps_update {
            if now.duration_since(last_update) >= Duration::from_secs(1) {
                // Reset the frame count and update the timestamp.
                self.fps = self.frame_count;
                self.frame_count = 0;
                self.last_fps_update = Some(now);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
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
        });
        // Request a repaint to continuously update the FPS.
        ctx.request_repaint();
    }
}
