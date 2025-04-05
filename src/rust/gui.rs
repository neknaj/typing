use eframe::egui;

/// Enum to specify character orientation.
#[derive(Clone,PartialEq)]
pub enum CharOrientation {
    Vertical,
    Horizontal,
}

/// Helper function to render a single character at a given position.
/// Returns the size of the rendered character.
fn render_char_at(
    ui: &mut egui::Ui,
    ch: char,
    pos: egui::Pos2,
    orientation: CharOrientation,
    font_id: &egui::FontId,
    color: egui::Color32,
) -> egui::Vec2 {
    // Convert the character to a string for layout.
    let s = ch.to_string();
    // Clone the font_id to pass ownership.
    let galley = ui.painter().layout_no_wrap(s, font_id.clone(), color);
    // Determine rotation angle in radians.
    let angle_rad = match orientation {
        CharOrientation::Horizontal => 0.0,
        CharOrientation::Vertical => std::f32::consts::FRAC_PI_2,
    };
    let rotation = egui::emath::Rot2::from_angle(angle_rad);
    // Adjust position so that the character is centered in its allocated space.
    let pos_adjusted = pos - (rotation * (galley.size() / 2.0));
    ui.painter().add(egui::epaint::TextShape {
        pos: pos_adjusted,
        galley: galley.clone(),
        angle: angle_rad,
        fallback_color: color,
        override_text_color: None,
        underline: Default::default(),
        opacity_factor: 1.0,
    });
    galley.size()
}

fn is_japanese(c: char) -> bool {
    let code = c as u32;
    (code >= 0x3040 && code <= 0x309F) || // ひらがな
    (code >= 0x30A0 && code <= 0x30FF) || // カタカナ
    (code >= 0x4E00 && code <= 0x9FFF)    // 漢字（CJK統合漢字）
}

fn is_japanese_kana(c: char) -> bool {
    let code = c as u32;
    (code >= 0x3040 && code <= 0x309F) || // ひらがな
    (code >= 0x30A0 && code <= 0x30FF)    // カタカナ
}

/// RenderText widget: renders a string by calling RenderChar for each character.
pub struct RenderText {
    text: String,
    orientation: CharOrientation,
    font_id: Option<egui::FontId>,
}

impl RenderText {
    /// Create a new RenderText widget.
    pub fn new(text: impl ToString, orientation: CharOrientation) -> Self {
        Self {
            text: text.to_string(),
            orientation,
            font_id: None,
        }
    }

    /// Set a custom font for the text.
    pub fn with_font(mut self, font_id: egui::FontId) -> Self {
        self.font_id = Some(font_id);
        self
    }
}

impl egui::Widget for RenderText {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // Retrieve text color from UI style.
        let color = ui.style().visuals.strong_text_color();
        let font_id = self
            .font_id
            .unwrap_or_else(|| egui::FontSelection::Default.resolve(ui.style()));
        // Calculate the total width and maximum height for the entire text.
        let mut total_size = 0.0;
        let mut max_size: f32 = 0.0;
        let mut char_sizes = Vec::new();
        for ch in self.text.chars() {
            let s = ch.to_string();
            let galley = ui.painter().layout_no_wrap(s, font_id.clone(), color);
            let size = galley.size();
            match (&self.orientation,is_japanese(ch)) {
                (CharOrientation::Horizontal,true) => {
                    char_sizes.push((ch, size));
                    let dx = if is_japanese_kana(ch) { size.x*0.8 } else { size.x };
                    total_size += dx;
                    max_size = max_size.max(size.y);
                },
                (CharOrientation::Horizontal,false) => {
                    char_sizes.push((ch, size));
                    let dx = size.x*0.8;
                    total_size += dx;
                    max_size = max_size.max(size.y);
                },
                (CharOrientation::Vertical,true) => {
                    char_sizes.push((ch, size));
                    let dy = if is_japanese_kana(ch) { size.x*0.85 } else { size.x };
                    total_size += dy;
                    max_size = max_size.max(size.x);
                },
                (CharOrientation::Vertical,false) => {
                    char_sizes.push((ch, size));
                    let dy = size.x*0.8;
                    total_size += dy;
                    max_size = max_size.max(size.y);
                },
            };
        }
        // Allocate the required space.
        let (rect, response) = ui.allocate_exact_size(if self.orientation==CharOrientation::Horizontal { egui::vec2(total_size, max_size) } else { egui::vec2(max_size, total_size ) }, egui::Sense::hover());
        // Render each character in sequence.
        let (mut x_offset,mut y_offset) = match self.orientation {
            CharOrientation::Horizontal => (rect.left(), rect.top()+font_id.size / 2.0),
            CharOrientation::Vertical => (rect.left()+font_id.size / 2.0, rect.top()),
        };
        for (ch, size) in char_sizes {
            match (&self.orientation,is_japanese(ch)) {
                (CharOrientation::Horizontal,true) => {
                    let dx = if is_japanese_kana(ch) { size.x*0.8 } else { size.x };
                    let pos = egui::pos2(x_offset+dx/2.0, y_offset);
                    render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_id, color);
                    x_offset += dx;
                },
                (CharOrientation::Horizontal,false) => {
                    let dx = size.x*0.8;
                    let pos = egui::pos2(x_offset+dx/2.0, y_offset);
                    render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_id, color);
                    x_offset += dx;
                },
                (CharOrientation::Vertical,true) => {
                    let dy = if is_japanese_kana(ch) { size.x*0.85 } else { size.x };
                    let pos = egui::pos2(x_offset, y_offset+dy/2.0);
                    render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_id, color);
                    y_offset += dy;
                },
                (CharOrientation::Vertical,false) => {
                    let dy = size.x*0.8;
                    let pos = egui::pos2(x_offset+font_id.size/6.0, y_offset+dy/2.0);
                    render_char_at(ui, ch, pos, CharOrientation::Vertical, &font_id, color);
                    y_offset += dy;
                },
            };
        }
        response
    }
}

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
            ui.add(RenderText::new("横書きテキスト Horizontal Text", CharOrientation::Horizontal).with_font(font.clone()));

            ui.separator();
        });
    }
}
