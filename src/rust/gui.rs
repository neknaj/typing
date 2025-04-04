use eframe::egui;

pub struct MyApp {
    name: String,
    age: u32,
    scale: f32, // Field for UI scaling factor
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_string(),
            age: 42,
            scale: 1.0, // Default scaling factor
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("私の egui アプリケーション");
            ui.horizontal(|ui| {
                ui.label("あなたの名前: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.horizontal(|ui| {
                ui.label("あなたの年齢: ");
                ui.add(egui::DragValue::new(&mut self.age).speed(1.0));
            });
            ui.label(format!("こんにちは「{}」さん、{}歳ですね！", self.name, self.age));
            if ui.button("リセット").clicked() {
                *self = Self::default();
            }
            // Interactive slider to adjust the scaling factor
            ui.horizontal(|ui| {
                ui.label("UI Scaling: ");
                ui.add(egui::Slider::new(&mut self.scale, 0.5..=5.0).text("scale"));
            });
            ui.label(format!("Current scale: {:.2}", self.scale));
        });
        // Update the UI scaling factor dynamically
        ctx.set_pixels_per_point(self.scale);
    }
}
