use eframe::egui;

use crate::textrender::{RenderText, CharOrientation};

// Sample application to demonstrate the usage of RenderChar and RenderText.
pub struct MyApp {
    name: String,
    age: u32,
    dark_mode: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_string(),
            age: 42,
            dark_mode: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let font = egui::FontId::new(50.0, egui::FontFamily::Proportional);
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            if ui.button("Toggle Theme").clicked() {
                // Toggle the theme state.
                self.dark_mode = !self.dark_mode;
                // Set the theme based on the updated state.
                let mut visuals = if self.dark_mode {
                    egui::Visuals::dark()
                } else {
                    egui::Visuals::light()
                };
                // Apply the customized visuals.
                ctx.set_visuals(visuals);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
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

            // RenderText example using vertical orientation.
            ui.label("RenderText (Vertical):");
            ui.add(RenderText::new("縦書きテキスト Vertical Text", CharOrientation::Vertical).with_font(font.clone()));

            ui.separator();

            // RenderText example using horizontal orientation.
            ui.label("RenderText (Horizontal):");
            ui.add(RenderText::new("ルビ付きの美しい横書き日本語を描画したい", CharOrientation::Horizontal).with_font(font.clone()));
            ui.add(RenderText::new("横書きテキスト Horizontal Text", CharOrientation::Horizontal).with_font(font.clone()));
            ui.add(RenderText::new("色は匂へど　散りぬるを", CharOrientation::Horizontal).with_font(font.clone()));
            ui.add(RenderText::new("我が世誰ぞ　常ならむ", CharOrientation::Horizontal).with_font(font.clone()));
            ui.add(RenderText::new("有為の奥山　今日越えて", CharOrientation::Horizontal).with_font(font.clone()));
            ui.add(RenderText::new("浅き夢見し　酔ひもせず", CharOrientation::Horizontal).with_font(font.clone()));

            ui.separator();
        });
    }
}
