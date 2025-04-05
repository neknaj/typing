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

fn is_japanese_hiragana(c: char) -> bool {
    let code = c as u32;
    (code >= 0x3040 && code <= 0x309F) // ひらがな
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
        let mut font_main = self
            .font_id.clone()
            .unwrap_or_else(|| egui::FontSelection::Default.resolve(ui.style()));
        let mut font_kana = self
            .font_id
            .unwrap_or_else(|| egui::FontSelection::Default.resolve(ui.style()));
        font_main.family = egui::FontFamily::Name("main".into());
        font_kana.family = egui::FontFamily::Name("kana".into());
        // Calculate the total width and maximum height for the entire text.
        let mut total_size = 0.0;
        let mut max_size: f32 = 0.0;
        let mut char_sizes = Vec::new();
        for ch in self.text.chars() {
            let s = ch.to_string();
            let galley = ui.painter().layout_no_wrap(s, font_main.clone(), color);
            let size = galley.size();
            match (&self.orientation,is_japanese(ch)) {
                (CharOrientation::Horizontal,true) => {
                    char_sizes.push((ch, size));
                    // let dx = if is_japanese_kana(ch) { size.x*0.8 } else { size.x };
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
            CharOrientation::Horizontal => (rect.left(), rect.top()+font_main.size / 2.0),
            CharOrientation::Vertical => (rect.left()+font_main.size / 2.0, rect.top()),
        };
        for (ch, size) in char_sizes {
            match (&self.orientation,is_japanese(ch)) {
                (CharOrientation::Horizontal,true) => {
                    let dx = if is_japanese_kana(ch) { size.x*0.8 } else { size.x };
                    let oy = if is_japanese_kana(ch) { if is_japanese_hiragana(ch) { size.y*0.03 } else { size.y*0.01 } } else { 0.0 };
                    if is_japanese_kana(ch) {
                        let pos = egui::pos2(x_offset+dx/2.0, y_offset+oy);
                        if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.85; } else { font_kana.size = font_main.size*0.95; }
                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_kana, color);
                    }
                    else {
                        let pos = egui::pos2(x_offset+dx/2.0, y_offset+oy);
                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_main, color);
                    }
                    x_offset += dx;
                },
                (CharOrientation::Horizontal,false) => {
                    let dx = size.x*0.8;
                    let pos = egui::pos2(x_offset+dx/2.0, y_offset+size.y/20.0);
                    let mut font = font_main.clone();
                    font.size = font_main.size*0.85;
                    render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font, color);
                    x_offset += dx;
                },
                (CharOrientation::Vertical,true) => {
                    let dy = if is_japanese_kana(ch) { size.x*0.85 } else { size.x };
                    let pos = egui::pos2(x_offset, y_offset+dy/2.0);
                    if is_japanese_kana(ch) {
                        if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.85; } else { font_kana.size = font_main.size*0.95; }
                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_kana, color);
                    }
                    else {
                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_main, color);
                    }
                    y_offset += dy;
                },
                (CharOrientation::Vertical,false) => {
                    let dy = size.x*0.8;
                    let pos = egui::pos2(x_offset+font_main.size/100.0, y_offset+dy/2.0);
                    let mut font = font_main.clone();
                    font.size = font_main.size*0.9;
                    render_char_at(ui, ch, pos, CharOrientation::Vertical, &font, color);
                    y_offset += dy;
                },
            };
        }
        response
    }
}