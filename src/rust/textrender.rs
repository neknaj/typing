use crate::parser::{Content, Line, Segment};

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
    // Clone the font_id to pass ownership.
    let galley = ui.painter().layout_no_wrap(ch.to_string(), font_id.clone(), color);
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
    (code == 0x30fc) || // 記号
    (code == 0x4e28) || // 記号
    (code >= 0x3000 && code <= 0x3040) || // 記号
    (code >= 0xFE30 && code <= 0xFE4F) || // 記号
    (code >= 0xFE10 && code <= 0xFE19) || // 記号
    (code >= 0x3040 && code <= 0x309F) || // ひらがな
    (code >= 0x30A0 && code <= 0x30FF) || // カタカナ
    (code >= 0x3400 && code <= 0x4DBF) || // 漢字（CJK統合漢字拡張A）
    (code >= 0x4E00 && code <= 0x9FFF) || // 漢字（CJK統合漢字）
    (code >= 0xF900 && code <= 0xFAFF) || // 漢字（CJK互換漢字）
    (code >= 0x20000 && code <= 0x3FFFF) || // 漢字（その他）
    false
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
        let mut s = self.text;
        if self.orientation == CharOrientation::Vertical {
            s = s
                // https://ja.wikipedia.org/wiki/CJK%E4%BA%92%E6%8F%9B%E5%BD%A2
                .replace('\u{2025}', "\u{fe30}")
                .replace('\u{2014}', "\u{fe31}")
                .replace('\u{2013}', "\u{fe32}")
                .replace('\u{205f}', "\u{fe33}")
                .replace('\u{2028}', "\u{fe35}")
                .replace('\u{2079}', "\u{fe36}")
                .replace('\u{207b}', "\u{fe37}")
                .replace('\u{307d}', "\u{fe38}")
                .replace('\u{30114}', "\u{fe39}")
                .replace('\u{3015}', "\u{fe3a}")
                .replace('\u{3010}', "\u{fe3b}")
                .replace('\u{3011}', "\u{fe3c}")
                .replace('\u{300a}', "\u{fe3d}")
                .replace('\u{300b}', "\u{fe3e}")
                .replace('\u{3008}', "\u{fe3f}")
                .replace('\u{3009}', "\u{fe40}")
                .replace('\u{300c}', "\u{fe41}")
                .replace('\u{300d}', "\u{fe42}")
                .replace('\u{300e}', "\u{fe43}")
                .replace('\u{202f}', "\u{fe44}")
                .replace('\u{005B}', "\u{fe47}")
                .replace('\u{005D}', "\u{fe48}")
                // https://ja.wikipedia.org/wiki/%E7%B8%A6%E6%9B%B8%E3%81%8D%E5%BD%A2
                .replace('\u{ff0c}', "\u{fe10}")
                .replace('\u{3001}', "\u{fe11}")
                .replace('\u{3002}', "\u{fe12}")
                .replace('\u{ff1a}', "\u{fe13}")
                .replace('\u{003b}', "\u{fe14}")
                .replace('\u{ff1b}', "\u{fe14}")
                .replace('\u{ff01}', "\u{fe15}")
                .replace('\u{ff1f}', "\u{fe16}")
                .replace('\u{3016}', "\u{fe17}")
                .replace('\u{3017}', "\u{fe18}")
                .replace('\u{2026}', "\u{fe19}")
                //
                .replace('\u{30fc}', "\u{4e28}");
        }
        for ch in s.chars() {
            let s = ch.to_string();
            let galley = ui.painter().layout_no_wrap(s, font_main.clone(), color);
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
                    let dy = size.x*0.75;
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
                    if is_japanese_kana(ch) && ch != '\u{30fc}' {
                        let mut pos = egui::pos2(x_offset+dx/2.0, y_offset+oy);
                        if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.85; } else { font_kana.size = font_main.size*0.95; }
                        if [
                            '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                            '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                            ].contains(&ch) {
                            if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.8; } else { font_kana.size = font_main.size*0.9; }
                            pos = egui::pos2(x_offset+dx/2.0-size.x/100.0, y_offset+oy+size.y/80.0);
                        }
                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_kana, color);
                    }
                    else if ch == '\u{30fc}'
                    {
                        let pos = egui::pos2(x_offset+dx/2.0, y_offset+oy);
                        let mut font = font_main.clone();
                        font.size = font_main.size*0.8;
                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font, color);
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
                    let mut pos = egui::pos2(x_offset, y_offset+dy/2.0);
                    if is_japanese_kana(ch) {
                        if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.85; } else { font_kana.size = font_main.size*0.95; }
                        if [
                            '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                            '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                            ].contains(&ch) {
                            if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.8; } else { font_kana.size = font_main.size*0.9; }
                            pos = egui::pos2(x_offset+size.x/10.0, y_offset+dy/2.0-size.y/10.0);
                        }
                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_kana, color);
                    }
                    else if ch == '\u{4e28}'
                    {
                        let mut font = font_main.clone();
                        font.size = font_main.size*0.93;
                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font, color);
                    }
                    else {
                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_main, color);
                    }
                    y_offset += dy;
                },
                (CharOrientation::Vertical,false) => {
                    let dy = size.x*0.75;
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


/// Widget to render a Line with ruby (furigana) annotations.
pub struct RenderLineWithRuby {
    line: Line,
    orientation: CharOrientation,
    font_id: Option<egui::FontId>,
    offset: f32,
    max: f32,
}

impl RenderLineWithRuby {
    /// Create a new RenderLineWithRuby widget.
    pub fn new(line: Line, orientation: CharOrientation) -> Self {
        Self {
            line,
            orientation,
            font_id: None,
            offset: 0.0,
            max: 1000.0,
        }
    }

    pub fn with_font(mut self, font_id: egui::FontId) -> Self {
        self.font_id = Some(font_id);
        self
    }
    pub fn with_offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }
    pub fn with_max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    /// Calculate the size of the rendered text.
    pub fn calc_size(&self, ui: &egui::Ui) -> (f32, f32) {
        let mut font_main = self
            .font_id.clone()
            .unwrap_or_else(|| egui::FontSelection::Default.resolve(ui.style()));
        let ruby_space = font_main.size * 0.3;
        font_main.family = egui::FontFamily::Name("main".into());

        let mut total_size = 0.0;
        let mut max_size: f32 = 0.0;

        // Calculate size for typed segments
        for (index, segment) in self.line.segments.iter().enumerate() {

            let mut s = match segment {
                Segment::Plain { text } => text.clone(),
                Segment::Annotated { base, reading: _ } => base.clone(),
            };

            if self.orientation == CharOrientation::Vertical {
                s = s.replace('\u{30fc}', "\u{4e28}");
            }

            for ch in s.chars() {
                let galley = ui.painter().layout_no_wrap(ch.to_string(), font_main.clone(), egui::Color32::WHITE);
                let size = galley.size();

                match (&self.orientation, is_japanese(ch)) {
                    (CharOrientation::Horizontal, true) => {
                        let dx = if is_japanese_kana(ch) { size.x * 0.8 } else { size.x };
                        total_size += dx;
                        max_size = max_size.max(size.y + ruby_space);
                    },
                    (CharOrientation::Horizontal, false) => {
                        total_size += size.x * 0.8;
                        max_size = max_size.max(size.y + ruby_space);
                    },
                    (CharOrientation::Vertical, true) => {
                        let dy = if is_japanese_kana(ch) { size.x * 0.85 } else { size.x };
                        total_size += dy;
                        max_size = max_size.max(size.x + ruby_space);
                    },
                    (CharOrientation::Vertical, false) => {
                        total_size += size.x * 0.75;
                        max_size = max_size.max(size.y + ruby_space);
                    },
                }
            }
        }

        (total_size, max_size)
    }
}

impl egui::Widget for RenderLineWithRuby {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // Retrieve text color from UI style.
        let color = ui.style().visuals.strong_text_color();
        let mut font_main = self
            .font_id.clone()
            .unwrap_or_else(|| egui::FontSelection::Default.resolve(ui.style()));
        let mut font_kana = self
            .font_id.clone()
            .unwrap_or_else(|| egui::FontSelection::Default.resolve(ui.style()));
        let mut font_ruby = self
            .font_id
            .unwrap_or_else(|| egui::FontSelection::Default.resolve(ui.style()));
        font_main.family = egui::FontFamily::Name("main".into());
        font_kana.family = egui::FontFamily::Name("kana".into());
        font_ruby.family = egui::FontFamily::Name("ruby".into());
        font_ruby.size = font_main.size*0.3;
        // Calculate the total width and maximum height for the entire text.
        // Allocate the required space.
        let mut rectinfo = Vec::new();
        let ruby_space = font_ruby.size;
        for segment in self.line.segments.iter() {
            let mut total_size = 0.0;
            let mut max_size: f32 = 0.0;
            let mut char_sizes = Vec::new();
            let mut s = match segment {
                Segment::Plain { text } => text.clone(),
                Segment::Annotated { base, reading } => base.clone(),
            };
            if self.orientation == CharOrientation::Vertical {
                s = s
                    // https://ja.wikipedia.org/wiki/CJK%E4%BA%92%E6%8F%9B%E5%BD%A2
                    .replace('\u{2025}', "\u{fe30}")
                    .replace('\u{2014}', "\u{fe31}")
                    .replace('\u{2013}', "\u{fe32}")
                    .replace('\u{205f}', "\u{fe33}")
                    .replace('\u{2028}', "\u{fe35}")
                    .replace('\u{2079}', "\u{fe36}")
                    .replace('\u{207b}', "\u{fe37}")
                    .replace('\u{307d}', "\u{fe38}")
                    .replace('\u{30114}', "\u{fe39}")
                    .replace('\u{3015}', "\u{fe3a}")
                    .replace('\u{3010}', "\u{fe3b}")
                    .replace('\u{3011}', "\u{fe3c}")
                    .replace('\u{300a}', "\u{fe3d}")
                    .replace('\u{300b}', "\u{fe3e}")
                    .replace('\u{3008}', "\u{fe3f}")
                    .replace('\u{3009}', "\u{fe40}")
                    .replace('\u{300c}', "\u{fe41}")
                    .replace('\u{300d}', "\u{fe42}")
                    .replace('\u{300e}', "\u{fe43}")
                    .replace('\u{202f}', "\u{fe44}")
                    .replace('\u{005B}', "\u{fe47}")
                    .replace('\u{005D}', "\u{fe48}")
                    // https://ja.wikipedia.org/wiki/%E7%B8%A6%E6%9B%B8%E3%81%8D%E5%BD%A2
                    .replace('\u{ff0c}', "\u{fe10}")
                    .replace('\u{3001}', "\u{fe11}")
                    .replace('\u{3002}', "\u{fe12}")
                    .replace('\u{ff1a}', "\u{fe13}")
                    .replace('\u{003b}', "\u{fe14}")
                    .replace('\u{ff1b}', "\u{fe14}")
                    .replace('\u{ff01}', "\u{fe15}")
                    .replace('\u{ff1f}', "\u{fe16}")
                    .replace('\u{3016}', "\u{fe17}")
                    .replace('\u{3017}', "\u{fe18}")
                    .replace('\u{2026}', "\u{fe19}")
                    //
                    .replace('\u{30fc}', "\u{4e28}");
            }
            for ch in s.chars() {
                let s = ch.to_string();
                let galley = ui.painter().layout_no_wrap(s, font_main.clone(), color);
                let size = galley.size();
                match (&self.orientation,is_japanese(ch)) {
                    (CharOrientation::Horizontal,true) => {
                        char_sizes.push((ch, size));
                        let dx = if is_japanese_kana(ch) { size.x*0.8 } else { size.x };
                        total_size += dx;
                        max_size = max_size.max(size.y+ruby_space);
                    },
                    (CharOrientation::Horizontal,false) => {
                        char_sizes.push((ch, size));
                        let dx = size.x*0.8;
                        total_size += dx;
                        max_size = max_size.max(size.y+ruby_space);
                    },
                    (CharOrientation::Vertical,true) => {
                        char_sizes.push((ch, size));
                        let dy = if is_japanese_kana(ch) { size.x*0.85 } else { size.x };
                        total_size += dy;
                        max_size = max_size.max(size.x+ruby_space);
                    },
                    (CharOrientation::Vertical,false) => {
                        char_sizes.push((ch, size));
                        let dy = size.x*0.75;
                        total_size += dy;
                        max_size = max_size.max(size.y+ruby_space);
                    },
                };
            }
            rectinfo.push((total_size, max_size, char_sizes,segment));
        }
        // rectinfoの中身を総和
        let total_size = rectinfo.iter().map(|(total_size, _, _,_)| *total_size).sum::<f32>();
        let max_size = rectinfo.iter().map(|(_, max_size, _,_)| *max_size).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0);
        let (rect, response) = ui.allocate_exact_size(if self.orientation==CharOrientation::Horizontal { egui::vec2(total_size, max_size) } else { egui::vec2(max_size, total_size ) }, egui::Sense::hover());
        // Render each character in sequence.
        let (mut x_offset,mut y_offset) = match self.orientation {
            CharOrientation::Horizontal => (rect.left(), rect.top()+font_main.size / 2.0),
            CharOrientation::Vertical => (rect.left()+font_main.size / 2.0, rect.top()),
        };
        for (total_size, max_size, char_sizes,segment) in rectinfo {
            let mut x_offset_ruby = x_offset;
            let mut y_offset_ruby = y_offset;
            let mut vert_x_offset = char_sizes[0].1.x;
            // baseの描画
            for (ch, size) in char_sizes {
                let mut f = true;
                match self.orientation {
                    CharOrientation::Horizontal if x_offset-self.offset+size.x < 0.0 || x_offset-self.offset > self.max => {
                        f = false;
                    }
                    CharOrientation::Vertical if y_offset-self.offset+size.x < 0.0 || y_offset-self.offset > self.max => {
                        f = false;
                    }
                    _ => {}
                }
                match (&self.orientation,is_japanese(ch)) {
                    (CharOrientation::Horizontal,true) => {
                        let dx = if is_japanese_kana(ch) { size.x*0.8 } else { size.x };
                        if f {
                            let oy = if is_japanese_kana(ch) { if is_japanese_hiragana(ch) { size.y*0.03 } else { size.y*0.01 } } else { 0.0 };
                            if is_japanese_kana(ch) && ch != '\u{30fc}' {
                                let mut pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+oy+ruby_space);
                                if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.85; } else { font_kana.size = font_main.size*0.95; }
                                if [
                                    '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                                    '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                                    ].contains(&ch) {
                                    if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.8; } else { font_kana.size = font_main.size*0.9; }
                                    pos = egui::pos2(x_offset+dx/2.0-self.offset-size.x/100.0, y_offset+oy+size.y/80.0+ruby_space);
                                }
                                render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_kana, color);
                            }
                            else if ch == '\u{30fc}'
                            {
                                let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+oy+ruby_space);
                                let mut font = font_main.clone();
                                font.size = font_main.size*0.8;
                                render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font, color);
                            }
                            else {
                                let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+oy+ruby_space);
                                render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_main, color);
                            }
                        }
                        x_offset += dx;
                    },
                    (CharOrientation::Horizontal,false) => {
                        let dx = size.x*0.8;
                        if f {
                            let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+size.y/20.0+ruby_space);
                            let mut font = font_main.clone();
                            font.size = font_main.size*0.85;
                            render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font, color);
                        }
                        x_offset += dx;
                    },
                    (CharOrientation::Vertical,true) => {
                        let dy = if is_japanese_kana(ch) { size.x*0.85 } else { size.x };
                        if f {
                            let mut pos = egui::pos2(x_offset, y_offset+dy/2.0-self.offset);
                            if is_japanese_kana(ch) {
                                if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.85; } else { font_kana.size = font_main.size*0.95; }
                                if [
                                    '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                                    '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                                    ].contains(&ch) {
                                    if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.8; } else { font_kana.size = font_main.size*0.9; }
                                    pos = egui::pos2(x_offset+size.x/10.0, y_offset+dy/2.0-size.y/10.0-self.offset);
                                }
                                render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_kana, color);
                            }
                            else if ch == '\u{4e28}'
                            {
                                let mut font = font_main.clone();
                                font.size = font_main.size*0.93;
                                render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font, color);
                            }
                            else {
                                render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_main, color);
                            }
                        }
                        y_offset += dy;
                    },
                    (CharOrientation::Vertical,false) => {
                        let dy = size.x*0.75;
                        if f {
                            let pos = egui::pos2(x_offset+font_main.size/100.0, y_offset+dy/2.0-self.offset);
                            let mut font = font_main.clone();
                            font.size = font_main.size*0.9;
                            render_char_at(ui, ch, pos, CharOrientation::Vertical, &font, color);
                        }
                        y_offset += dy;
                    },
                };
            }
            match segment {
                Segment::Annotated { base, reading } => {
                    // rubyの描画
                    let ruby: Vec<char> = reading.chars().collect();
                    let w = total_size/(ruby.len()) as f32;
                    x_offset_ruby += w*0.5;
                    y_offset_ruby += w*0.5;
                    match self.orientation {
                        CharOrientation::Horizontal => {
                            for ch in ruby {
                                let s = ch.to_string();
                                let galley = ui.painter().layout_no_wrap(s, font_ruby.clone(), color);
                                let size = galley.size();
                                let mut f = true;
                                if x_offset_ruby-self.offset+size.x < 0.0 || x_offset_ruby-self.offset > self.max {
                                    f = false;
                                }
                                if f {
                                    let dx = galley.rect.width()*0.0;
                                    if is_japanese_kana(ch) && ch != '\u{30fc}' {
                                        let mut pos = egui::pos2(x_offset_ruby+dx-self.offset, rect.top()+ruby_space*0.5);
                                        if [
                                            '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                                            '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                                            ].contains(&ch) {
                                            pos = egui::pos2(x_offset_ruby+dx-self.offset-size.x/100.0, rect.top()+ruby_space*0.5+size.y/80.0);
                                        }
                                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_ruby, color);
                                    }
                                    else if ch == '\u{30fc}'
                                    {
                                        let pos = egui::pos2(x_offset_ruby+dx-self.offset, rect.top()+ruby_space*0.5);
                                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_ruby, color);
                                    }
                                    else {
                                        let mut pos = egui::pos2(x_offset_ruby+dx-self.offset, rect.top()+ruby_space*0.5);
                                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_ruby, color);
                                    }
                                }
                                x_offset_ruby += w;
                            }
                        },
                        CharOrientation::Vertical => {
                            for ch in ruby {
                                let s = ch.to_string();
                                let galley = ui.painter().layout_no_wrap(s, font_ruby.clone(), color);
                                let size = galley.size();
                                let dx = size.x*0.5;
                                let dy = size.x*0.25;
                                let mut pos = egui::pos2(rect.left()+vert_x_offset+dx, y_offset+dy-self.offset);
                                if is_japanese_kana(ch) {
                                    let mut pos = egui::pos2(rect.left()+vert_x_offset+dx, y_offset_ruby+dy-self.offset);
                                    if [
                                        '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                                        '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                                        ].contains(&ch) {
                                        pos = egui::pos2(rect.left()+vert_x_offset+dx+size.x/10.0, y_offset_ruby+dy-size.y/10.0-self.offset);
                                    }
                                    render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_ruby, color);
                                }
                                else if ch == '\u{4e28}'
                                {
                                    render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_ruby, color);
                                }
                                else {
                                    render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_ruby, color);
                                }
                                y_offset_ruby += w;
                            }
                        }
                    }
                },
                _ => {}
            };
        }

        response
    }
}


use crate::model::{ TypingCorrectnessChar, TypingCorrectnessSegment,TypingCorrectnessLine, TypingStatus };

pub struct RenderTypingLine {
    line: Line,
    correctness: TypingCorrectnessLine,
    status: TypingStatus,
    orientation: CharOrientation,
    font_id: Option<egui::FontId>,
    offset: f32,
}

impl RenderTypingLine {
    /// Create a new RenderLineWithRuby widget.
    pub fn new(line: Line, correctness: TypingCorrectnessLine, status: TypingStatus, orientation: CharOrientation) -> Self {
        Self {
            line,
            orientation,
            correctness,
            status,
            font_id: None,
            offset: 0.0,
        }
    }

    pub fn with_font(mut self, font_id: egui::FontId) -> Self {
        self.font_id = Some(font_id);
        self
    }
    pub fn with_offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// Calculate the size of the rendered text.
    pub fn calc_size(&self, ui: &egui::Ui) -> (f32, f32) {
        let mut font_main = self
            .font_id.clone()
            .unwrap_or_else(|| egui::FontSelection::Default.resolve(ui.style()));
        let ruby_space = font_main.size * 0.3;
        font_main.family = egui::FontFamily::Name("main".into());

        let mut total_size = 0.0;
        let mut max_size: f32 = 0.0;

        // Calculate size for typed segments
        for (index, segment) in self.line.segments.iter().enumerate() {
            if index >= self.status.segment as usize {
                break;
            }

            let mut s = match segment {
                Segment::Plain { text } => text.clone(),
                Segment::Annotated { base, reading: _ } => base.clone(),
            };

            if self.orientation == CharOrientation::Vertical {
                s = s.replace('\u{30fc}', "\u{4e28}");
            }

            for ch in s.chars() {
                let galley = ui.painter().layout_no_wrap(ch.to_string(), font_main.clone(), egui::Color32::WHITE);
                let size = galley.size();

                match (&self.orientation, is_japanese(ch)) {
                    (CharOrientation::Horizontal, true) => {
                        let dx = if is_japanese_kana(ch) { size.x * 0.8 } else { size.x };
                        total_size += dx;
                        max_size = max_size.max(size.y + ruby_space);
                    },
                    (CharOrientation::Horizontal, false) => {
                        total_size += size.x * 0.8;
                        max_size = max_size.max(size.y + ruby_space);
                    },
                    (CharOrientation::Vertical, true) => {
                        let dy = if is_japanese_kana(ch) { size.x * 0.85 } else { size.x };
                        total_size += dy;
                        max_size = max_size.max(size.x + ruby_space);
                    },
                    (CharOrientation::Vertical, false) => {
                        total_size += size.x * 0.75;
                        max_size = max_size.max(size.y + ruby_space);
                    },
                }
            }
        }

        // Calculate size for current segment
        if self.status.segment < self.line.segments.len() as i32 {
            let current_segment = &self.line.segments[self.status.segment as usize];
            let text = match current_segment {
                Segment::Plain { text } => text.chars().take(self.status.char_ as usize).collect::<String>(),
                Segment::Annotated { base: _, reading } => reading.chars().take(self.status.char_ as usize).collect::<String>(),
            };

            for ch in text.chars() {
                let galley = ui.painter().layout_no_wrap(ch.to_string(), font_main.clone(), egui::Color32::WHITE);
                let size = galley.size();
                
                match self.orientation {
                    CharOrientation::Horizontal => {
                        total_size += size.x * 0.8;
                        max_size = max_size.max(size.y + ruby_space);
                    },
                    CharOrientation::Vertical => {
                        total_size += if is_japanese(ch) { size.x * 0.85 } else { size.x * 0.75 };
                        max_size = max_size.max(if is_japanese(ch) { size.x } else { size.y } + ruby_space);
                    },
                }
            }
        }

        (total_size, max_size)
    }
}

impl egui::Widget for RenderTypingLine {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // Retrieve text color from UI style.
        let color = ui.style().visuals.strong_text_color();
        let wrong_color = egui::Color32::from_hex("#f55252").unwrap();
        let incorrect_color = egui::Color32::from_hex("#ff9898").unwrap();
        let correct_color = egui::Color32::from_hex("#9097ff").unwrap();
        let pending_color = egui::Color32::from_hex("#999999").unwrap();
        let cursor_color = ui.style().visuals.selection.bg_fill;
        let mut font_main = self
            .font_id.clone()
            .unwrap_or_else(|| egui::FontSelection::Default.resolve(ui.style()));
        let mut font_kana = self
            .font_id.clone()
            .unwrap_or_else(|| egui::FontSelection::Default.resolve(ui.style()));
        let mut font_ruby = self
            .font_id
            .unwrap_or_else(|| egui::FontSelection::Default.resolve(ui.style()));
        font_main.family = egui::FontFamily::Name("main".into());
        font_kana.family = egui::FontFamily::Name("kana".into());
        font_ruby.family = egui::FontFamily::Name("ruby".into());
        font_ruby.size = font_main.size*0.3;
        // Calculate the total width and maximum height for the entire text.
        // Allocate the required space.
        let mut rectinfo = Vec::new();
        let ruby_space = font_ruby.size;

        //
        // typed segment
        //
        for (index,segment) in self.line.segments.iter().enumerate() {
            if index>=self.status.segment as usize {
                break;
            }
            let mut total_size = 0.0;
            let mut max_size: f32 = 0.0;
            let mut char_sizes = Vec::new();
            let mut s = match segment {
                Segment::Plain { text } => text.clone(),
                Segment::Annotated { base, reading } => base.clone(),
            };
            if self.orientation == CharOrientation::Vertical {
                s = s
                    // https://ja.wikipedia.org/wiki/CJK%E4%BA%92%E6%8F%9B%E5%BD%A2
                    .replace('\u{2025}', "\u{fe30}")
                    .replace('\u{2014}', "\u{fe31}")
                    .replace('\u{2013}', "\u{fe32}")
                    .replace('\u{205f}', "\u{fe33}")
                    .replace('\u{2028}', "\u{fe35}")
                    .replace('\u{2079}', "\u{fe36}")
                    .replace('\u{207b}', "\u{fe37}")
                    .replace('\u{307d}', "\u{fe38}")
                    .replace('\u{30114}', "\u{fe39}")
                    .replace('\u{3015}', "\u{fe3a}")
                    .replace('\u{3010}', "\u{fe3b}")
                    .replace('\u{3011}', "\u{fe3c}")
                    .replace('\u{300a}', "\u{fe3d}")
                    .replace('\u{300b}', "\u{fe3e}")
                    .replace('\u{3008}', "\u{fe3f}")
                    .replace('\u{3009}', "\u{fe40}")
                    .replace('\u{300c}', "\u{fe41}")
                    .replace('\u{300d}', "\u{fe42}")
                    .replace('\u{300e}', "\u{fe43}")
                    .replace('\u{202f}', "\u{fe44}")
                    .replace('\u{005B}', "\u{fe47}")
                    .replace('\u{005D}', "\u{fe48}")
                    // https://ja.wikipedia.org/wiki/%E7%B8%A6%E6%9B%B8%E3%81%8D%E5%BD%A2
                    .replace('\u{ff0c}', "\u{fe10}")
                    .replace('\u{3001}', "\u{fe11}")
                    .replace('\u{3002}', "\u{fe12}")
                    .replace('\u{ff1a}', "\u{fe13}")
                    .replace('\u{003b}', "\u{fe14}")
                    .replace('\u{ff1b}', "\u{fe14}")
                    .replace('\u{ff01}', "\u{fe15}")
                    .replace('\u{ff1f}', "\u{fe16}")
                    .replace('\u{3016}', "\u{fe17}")
                    .replace('\u{3017}', "\u{fe18}")
                    .replace('\u{2026}', "\u{fe19}")
                    //
                    .replace('\u{30fc}', "\u{4e28}");
            }
            for ch in s.chars() {
                let s = ch.to_string();
                let galley = ui.painter().layout_no_wrap(s, font_main.clone(), color);
                let size = galley.size();
                match (&self.orientation,is_japanese(ch)) {
                    (CharOrientation::Horizontal,true) => {
                        char_sizes.push((ch, size));
                        let dx = if is_japanese_kana(ch) { size.x*0.8 } else { size.x };
                        total_size += dx;
                        max_size = max_size.max(size.y+ruby_space);
                    },
                    (CharOrientation::Horizontal,false) => {
                        char_sizes.push((ch, size));
                        let dx = size.x*0.8;
                        total_size += dx;
                        max_size = max_size.max(size.y+ruby_space);
                    },
                    (CharOrientation::Vertical,true) => {
                        char_sizes.push((ch, size));
                        let dy = if is_japanese_kana(ch) { size.x*0.85 } else { size.x };
                        total_size += dy;
                        max_size = max_size.max(size.x+ruby_space);
                    },
                    (CharOrientation::Vertical,false) => {
                        char_sizes.push((ch, size));
                        let dy = size.x*0.75;
                        total_size += dy;
                        max_size = max_size.max(size.y+ruby_space);
                    },
                };
            }
            rectinfo.push((total_size, max_size, char_sizes,segment));
        }
        // rectinfoの中身を総和
        let total_size = rectinfo.iter().map(|(total_size, _, _,_)| *total_size).sum::<f32>();
        let max_size = rectinfo.iter().map(|(_, max_size, _,_)| *max_size).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0);
        let (rect, response) = ui.allocate_exact_size(if self.orientation==CharOrientation::Horizontal { egui::vec2(total_size, max_size) } else { egui::vec2(max_size, total_size ) }, egui::Sense::hover());
        // Render each character in sequence.
        let (mut x_offset,mut y_offset) = match self.orientation {
            CharOrientation::Horizontal => (rect.left(), rect.top()+font_main.size / 2.0),
            CharOrientation::Vertical => (rect.left()+font_main.size / 2.0, rect.top()),
        };
        // typed segmentsの表示
        for (index, (total_size, max_size, char_sizes, segment)) in rectinfo.iter().enumerate() {
            let mut x_offset_ruby = x_offset;
            let mut y_offset_ruby = y_offset;
            let mut vert_x_offset = char_sizes[0].1.x;
            // 各文字の色を決定
            let char_color = self.correctness.segments[index].chars.iter().map(|c| match c {
                    TypingCorrectnessChar::Correct => correct_color,
                    _ => incorrect_color,
                }).collect::<Vec<_>>();
            match segment {
                Segment::Annotated { base, reading } => {
                    let col = &if self.correctness.segments[index].chars.iter().any(|c| match c {
                            TypingCorrectnessChar::Correct => false,  // The character is correct
                            _ => true,                                // The character is incorrect
                        }) {
                            incorrect_color  // If at least one character is incorrect
                        } else {
                            correct_color    // If all characters are correct
                        };
                    // baseの描画
                    for (ch, size) in char_sizes.iter() {
                        let mut f = true;
                        if x_offset-self.offset+size.x < 0.0 {
                            f = false;
                        }
                        match (&self.orientation,is_japanese(*ch)) {
                            (CharOrientation::Horizontal,true) => {
                                let dx = if is_japanese_kana(*ch) { size.x*0.8 } else { size.x };
                                if f {
                                    let oy = if is_japanese_kana(*ch) { if is_japanese_hiragana(*ch) { size.y*0.03 } else { size.y*0.01 } } else { 0.0 };
                                    if is_japanese_kana(*ch) && *ch != '\u{30fc}' {
                                        let mut pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+oy+ruby_space);
                                        if is_japanese_hiragana(*ch) { font_kana.size = font_main.size*0.85; } else { font_kana.size = font_main.size*0.95; }
                                        if [
                                            '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                                            '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                                            ].contains(&*ch) {
                                            if is_japanese_hiragana(*ch) { font_kana.size = font_main.size*0.8; } else { font_kana.size = font_main.size*0.9; }
                                            pos = egui::pos2(x_offset+dx/2.0-self.offset-size.x/100.0, y_offset+oy+size.y/80.0+ruby_space);
                                        }
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_kana, *col);
                                    }
                                    else if *ch == '\u{30fc}'
                                    {
                                        let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+oy+ruby_space);
                                        let mut font = font_main.clone();
                                        font.size = font_main.size*0.8;
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font, *col);
                                    }
                                    else {
                                        let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+oy+ruby_space);
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_main, *col);
                                    }
                                }
                                x_offset += dx;
                            },
                            (CharOrientation::Horizontal,false) => {
                                let dx = size.x*0.8;
                                if f{
                                    let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+size.y/20.0+ruby_space);
                                    let mut font = font_main.clone();
                                    font.size = font_main.size*0.85;
                                    render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font, *col);
                                }
                                x_offset += dx;
                            },
                            (CharOrientation::Vertical,true) => {
                                let dy = if is_japanese_kana(*ch) { size.x*0.85 } else { size.x };
                                if f {
                                    let mut pos = egui::pos2(x_offset, y_offset+dy/2.0-self.offset);
                                    if is_japanese_kana(*ch) {
                                        if is_japanese_hiragana(*ch) { font_kana.size = font_main.size*0.85; } else { font_kana.size = font_main.size*0.95; }
                                        if [
                                            '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                                            '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                                            ].contains(&*ch) {
                                            if is_japanese_hiragana(*ch) { font_kana.size = font_main.size*0.8; } else { font_kana.size = font_main.size*0.9; }
                                            pos = egui::pos2(x_offset+size.x/10.0, y_offset+dy/2.0-size.y/10.0-self.offset);
                                        }
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_kana, *col);
                                    }
                                    else if *ch == '\u{4e28}'
                                    {
                                        let mut font = font_main.clone();
                                        font.size = font_main.size*0.93;
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font, *col);
                                    }
                                    else {
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_main, *col);
                                    }
                                }
                                y_offset += dy;
                            },
                            (CharOrientation::Vertical,false) => {
                                let dy = size.x*0.75;
                                if f {
                                    let pos = egui::pos2(x_offset+font_main.size/100.0, y_offset+dy/2.0-self.offset);
                                    let mut font = font_main.clone();
                                    font.size = font_main.size*0.9;
                                    render_char_at(ui, *ch, pos, CharOrientation::Vertical, &font, *col);
                                }
                                y_offset += dy;
                            },
                        };
                    }
                    // rubyの描画
                    let ruby: Vec<char> = reading.chars().collect();
                    let w = total_size/(ruby.len()) as f32;
                    x_offset_ruby += w*0.5;
                    y_offset_ruby += w*0.5;
                    match self.orientation {
                        CharOrientation::Horizontal => {
                            for (ch,col) in ruby.iter().zip(char_color.iter()) {
                                let s = ch.to_string();
                                let galley = ui.painter().layout_no_wrap(s, font_ruby.clone(), *col);
                                let size = galley.size();
                                let mut f = true;
                                if x_offset_ruby-self.offset+size.x < 0.0 {
                                    f = false;
                                }
                                if f {
                                    let dx = galley.rect.width()*0.0;
                                    if is_japanese_kana(*ch) && *ch != '\u{30fc}' {
                                        let mut pos = egui::pos2(x_offset_ruby+dx-self.offset, rect.top()+ruby_space*0.5);
                                        if [
                                            '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                                            '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                                            ].contains(&ch) {
                                            pos = egui::pos2(x_offset_ruby+dx-self.offset-size.x/100.0, rect.top()+ruby_space*0.5+size.y/80.0);
                                        }
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_ruby, *col);
                                    }
                                    else if *ch == '\u{30fc}'
                                    {
                                        let pos = egui::pos2(x_offset_ruby+dx-self.offset, rect.top()+ruby_space*0.5);
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_ruby, *col);
                                    }
                                    else {
                                        let mut pos = egui::pos2(x_offset_ruby+dx-self.offset, rect.top()+ruby_space*0.5);
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_ruby, *col);
                                    }
                                }
                                x_offset_ruby += w;
                            }
                        },
                        CharOrientation::Vertical => {
                            for (ch,col) in ruby.iter().zip(char_color.iter()) {
                                let s = ch.to_string();
                                let galley = ui.painter().layout_no_wrap(s, font_ruby.clone(), *col);
                                let size = galley.size();
                                let mut f = true;
                                if x_offset_ruby-self.offset+size.x < 0.0 {
                                    f = false;
                                }
                                if f {
                                    let dx = size.x*0.5;
                                    let dy = size.x*0.25;
                                    let mut pos = egui::pos2(rect.left()+vert_x_offset+dx, y_offset+dy-self.offset);
                                    if is_japanese_kana(*ch) {
                                        let mut pos = egui::pos2(rect.left()+vert_x_offset+dx, y_offset_ruby+dy-self.offset);
                                        if [
                                            '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                                            '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                                            ].contains(&ch) {
                                            pos = egui::pos2(rect.left()+vert_x_offset+dx+size.x/10.0, y_offset_ruby+dy-size.y/10.0-self.offset);
                                        }
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_ruby, *col);
                                    }
                                    else if *ch == '\u{4e28}'
                                    {
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_ruby, *col);
                                    }
                                    else {
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_ruby, *col);
                                    }
                                    y_offset_ruby += w;
                                }
                            }
                        }
                    }
                },
                Segment::Plain { text } => {
                    // baseの描画 (rubyは無い)
                    for ((ch, size), col) in char_sizes.iter().zip(char_color.iter()) {
                        let mut f = true;
                        if x_offset-self.offset+size.x < 0.0 {
                            f = false;
                        }
                        match (&self.orientation,is_japanese(*ch)) {
                            (CharOrientation::Horizontal,true) => {
                                let dx = if is_japanese_kana(*ch) { size.x*0.8 } else { size.x };
                                if f {
                                    let oy = if is_japanese_kana(*ch) { if is_japanese_hiragana(*ch) { size.y*0.03 } else { size.y*0.01 } } else { 0.0 };
                                    if is_japanese_kana(*ch) && *ch != '\u{30fc}' {
                                        let mut pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+oy+ruby_space);
                                        if is_japanese_hiragana(*ch) { font_kana.size = font_main.size*0.85; } else { font_kana.size = font_main.size*0.95; }
                                        if [
                                            '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                                            '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                                            ].contains(&*ch) {
                                            if is_japanese_hiragana(*ch) { font_kana.size = font_main.size*0.8; } else { font_kana.size = font_main.size*0.9; }
                                            pos = egui::pos2(x_offset+dx/2.0-self.offset-size.x/100.0, y_offset+oy+size.y/80.0+ruby_space);
                                        }
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_kana, *col);
                                    }
                                    else if *ch == '\u{30fc}'
                                    {
                                        let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+oy+ruby_space);
                                        let mut font = font_main.clone();
                                        font.size = font_main.size*0.8;
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font, *col);
                                    }
                                    else {
                                        let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+oy+ruby_space);
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_main, *col);
                                    }
                                }
                                x_offset += dx;
                            },
                            (CharOrientation::Horizontal,false) => {
                                let dx = size.x*0.8;
                                if f {
                                    let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+size.y/20.0+ruby_space);
                                    let mut font = font_main.clone();
                                    font.size = font_main.size*0.85;
                                    render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font, *col);
                                }
                                x_offset += dx;
                            },
                            (CharOrientation::Vertical,true) => {
                                let dy = if is_japanese_kana(*ch) { size.x*0.85 } else { size.x };
                                if f {
                                    let mut pos = egui::pos2(x_offset, y_offset+dy/2.0-self.offset);
                                    if is_japanese_kana(*ch) {
                                        if is_japanese_hiragana(*ch) { font_kana.size = font_main.size*0.85; } else { font_kana.size = font_main.size*0.95; }
                                        if [
                                            '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                                            '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                                            ].contains(&*ch) {
                                            if is_japanese_hiragana(*ch) { font_kana.size = font_main.size*0.8; } else { font_kana.size = font_main.size*0.9; }
                                            pos = egui::pos2(x_offset+size.x/10.0, y_offset+dy/2.0-size.y/10.0-self.offset);
                                        }
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_kana, *col);
                                    }
                                    else if *ch == '\u{4e28}'
                                    {
                                        let mut font = font_main.clone();
                                        font.size = font_main.size*0.93;
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font, *col);
                                    }
                                    else {
                                        render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font_main, *col);
                                    }
                                }
                                y_offset += dy;
                            },
                            (CharOrientation::Vertical,false) => {
                                let dy = size.x*0.75;
                                if f {
                                    let pos = egui::pos2(x_offset+font_main.size/100.0, y_offset+dy/2.0-self.offset);
                                    let mut font = font_main.clone();
                                    font.size = font_main.size*0.9;
                                    render_char_at(ui, *ch, pos, CharOrientation::Vertical, &font, *col);
                                }
                                y_offset += dy;
                            },
                        };
                    }
                }
            };
        }
        
        // 現在入力中のセグメント (PendingSegment) の表示
        if self.status.segment < self.line.segments.len() as i32 {
            let current_segment = &self.line.segments[self.status.segment as usize];
            let text = match current_segment {
                Segment::Plain { text } => text.chars().take(self.status.char_ as usize).collect::<String>(),
                Segment::Annotated { base: _, reading } => reading.chars().take(self.status.char_ as usize).collect::<String>(),
            };
            
            let mut s = text;
            if self.orientation == CharOrientation::Vertical {
                s = s
                    // https://ja.wikipedia.org/wiki/CJK%E4%BA%92%E6%8F%9B%E5%BD%A2
                    .replace('\u{2025}', "\u{fe30}")
                    .replace('\u{2014}', "\u{fe31}")
                    .replace('\u{2013}', "\u{fe32}")
                    .replace('\u{205f}', "\u{fe33}")
                    .replace('\u{2028}', "\u{fe35}")
                    .replace('\u{2079}', "\u{fe36}")
                    .replace('\u{207b}', "\u{fe37}")
                    .replace('\u{307d}', "\u{fe38}")
                    .replace('\u{30114}', "\u{fe39}")
                    .replace('\u{3015}', "\u{fe3a}")
                    .replace('\u{3010}', "\u{fe3b}")
                    .replace('\u{3011}', "\u{fe3c}")
                    .replace('\u{300a}', "\u{fe3d}")
                    .replace('\u{300b}', "\u{fe3e}")
                    .replace('\u{3008}', "\u{fe3f}")
                    .replace('\u{3009}', "\u{fe40}")
                    .replace('\u{300c}', "\u{fe41}")
                    .replace('\u{300d}', "\u{fe42}")
                    .replace('\u{300e}', "\u{fe43}")
                    .replace('\u{202f}', "\u{fe44}")
                    .replace('\u{005B}', "\u{fe47}")
                    .replace('\u{005D}', "\u{fe48}")
                    // https://ja.wikipedia.org/wiki/%E7%B8%A6%E6%9B%B8%E3%81%8D%E5%BD%A2
                    .replace('\u{ff0c}', "\u{fe10}")
                    .replace('\u{3001}', "\u{fe11}")
                    .replace('\u{3002}', "\u{fe12}")
                    .replace('\u{ff1a}', "\u{fe13}")
                    .replace('\u{003b}', "\u{fe14}")
                    .replace('\u{ff1b}', "\u{fe14}")
                    .replace('\u{ff01}', "\u{fe15}")
                    .replace('\u{ff1f}', "\u{fe16}")
                    .replace('\u{3016}', "\u{fe17}")
                    .replace('\u{3017}', "\u{fe18}")
                    .replace('\u{2026}', "\u{fe19}")
                    //
                    .replace('\u{30fc}', "\u{4e28}");
            }

            // 入力済み文字の表示（色付き）
            for (i, ch) in s.chars().enumerate() {
                let color = match &self.correctness.segments[self.status.segment as usize].chars[i] {
                    TypingCorrectnessChar::Correct => correct_color,
                    TypingCorrectnessChar::Incorrect => incorrect_color,
                    TypingCorrectnessChar::Pending => pending_color,
                };

                let galley = ui.painter().layout_no_wrap(ch.to_string(), font_main.clone(), color);
                let size = galley.size();

                match (&self.orientation, is_japanese(ch)) {
                    (CharOrientation::Horizontal, true) => {
                        let dx = if is_japanese_kana(ch) { size.x*0.8 } else { size.x };
                        let oy = if is_japanese_kana(ch) { if is_japanese_hiragana(ch) { size.y*0.03 } else { size.y*0.01 } } else { 0.0 };
                        if is_japanese_kana(ch) && ch != '\u{30fc}' {
                            let mut pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+oy+ruby_space);
                            if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.85; } else { font_kana.size = font_main.size*0.95; }
                            if [
                                '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                                '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                                ].contains(&ch) {
                                if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.8; } else { font_kana.size = font_main.size*0.9; }
                                pos = egui::pos2(x_offset+dx/2.0-self.offset-size.x/100.0, y_offset+oy+size.y/80.0+ruby_space);
                            }
                            render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_kana, color);
                        }
                        else if ch == '\u{30fc}'
                        {
                            let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+oy+ruby_space);
                            let mut font = font_main.clone();
                            font.size = font_main.size*0.8;
                            render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font, color);
                        }
                        else {
                            let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+oy+ruby_space);
                            render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_main, color);
                        }
                        x_offset += dx;
                    },
                    (CharOrientation::Horizontal,false) => {
                        let dx = size.x*0.8;
                        let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+size.y/20.0+ruby_space);
                        let mut font = font_main.clone();
                        font.size = font_main.size*0.85;
                        render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font, color);
                        x_offset += dx;
                    },
                    (CharOrientation::Vertical,true) => {
                        let dy = if is_japanese_kana(ch) { size.x*0.85 } else { size.x };
                        let mut pos = egui::pos2(x_offset, y_offset+dy/2.0-self.offset);
                        if is_japanese_kana(ch) {
                            if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.85; } else { font_kana.size = font_main.size*0.95; }
                            if [
                                '\u{3041}','\u{3043}','\u{3045}','\u{3047}','\u{3049}','\u{3063}','\u{3041}','\u{3083}','\u{3085}','\u{3087}','\u{308e}','\u{3095}','\u{3096}','\u{3041}',
                                '\u{30a1}','\u{30a3}','\u{30a5}','\u{30a7}','\u{30a9}','\u{30c3}','\u{30e3}','\u{30e5}','\u{30e7}','\u{30ee}','\u{30f5}','\u{30f6}'
                                ].contains(&ch) {
                                if is_japanese_hiragana(ch) { font_kana.size = font_main.size*0.8; } else { font_kana.size = font_main.size*0.9; }
                                pos = egui::pos2(x_offset+size.x/10.0, y_offset+dy/2.0-size.y/10.0-self.offset);
                            }
                            render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_kana, color);
                        }
                        else if ch == '\u{4e28}'
                        {
                            let mut font = font_main.clone();
                            font.size = font_main.size*0.93;
                            render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font, color);
                        }
                        else {
                            render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font_main, color);
                        }
                        y_offset += dy;
                    },
                    (CharOrientation::Vertical,false) => {
                        let dy = size.x*0.75;
                        let pos = egui::pos2(x_offset+font_main.size/100.0, y_offset+dy/2.0-self.offset);
                        let mut font = font_main.clone();
                        font.size = font_main.size*0.9;
                        render_char_at(ui, ch, pos, CharOrientation::Vertical, &font, color);
                        y_offset += dy;
                    },
                }
            }
        }

        // 未確定文字列の表示
        for ch in &self.status.unconfirmed {
            let galley = ui.painter().layout_no_wrap(ch.to_string(), font_main.clone(), pending_color);
            let size = galley.size();

            match &self.orientation {
                CharOrientation::Horizontal => {
                    let dx = size.x*0.8;
                    let dy = size.x*0.1;
                    let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+size.y/20.0+ruby_space+dy);
                    let mut font = font_main.clone();
                    font.size = font_main.size*0.7;
                    render_char_at(ui, *ch, pos, CharOrientation::Horizontal, &font, pending_color);
                    x_offset += dx;
                },
                CharOrientation::Vertical => {
                    let dy = size.x*0.75;
                    let pos = egui::pos2(x_offset+font_main.size/100.0, y_offset+dy/2.0-self.offset);
                    let mut font = font_main.clone();
                    font.size = font_main.size*0.9;
                    render_char_at(ui, *ch, pos, CharOrientation::Vertical, &font, pending_color);
                    y_offset += dy;
                },
            }
        }

        // カーソルの表示
        match &self.orientation {
            CharOrientation::Horizontal => {
                let cursor_width = 2.0;
                let cursor_height = font_main.size;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(x_offset-self.offset, y_offset),
                        egui::vec2(cursor_width, cursor_height),
                    ),
                    0.0,
                    cursor_color,
                );
            },
            CharOrientation::Vertical => {
                let cursor_width = 2.0;
                let cursor_height = font_main.size;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(x_offset-font_main.size*0.5, y_offset-self.offset),
                        egui::vec2( cursor_height, cursor_width),
                    ),
                    0.0,
                    cursor_color,
                );
            },
        }
        
        // 誤入力文字の表示
        if let Some(ch) = self.status.last_wrong_keydown {
            let galley = ui.painter().layout_no_wrap(ch.to_string(), font_main.clone(), pending_color);
            let size = galley.size();
            match &self.orientation {
                CharOrientation::Horizontal => {
                    let dx = size.x*0.8;
                    let dy = size.x*0.1;
                    let pos = egui::pos2(x_offset+dx/2.0-self.offset, y_offset+size.y/20.0+ruby_space+dy);
                    let mut font = font_main.clone();
                    font.size = font_main.size*0.7;
                    render_char_at(ui, ch, pos, CharOrientation::Horizontal, &font, wrong_color);
                    x_offset += dx;
                },
                CharOrientation::Vertical => {
                    let dy = size.x*0.75;
                    let pos = egui::pos2(x_offset+font_main.size/100.0, y_offset+dy/2.0-self.offset);
                    let mut font = font_main.clone();
                    font.size = font_main.size*0.9;
                    render_char_at(ui, ch, pos, CharOrientation::Vertical, &font, wrong_color);
                    y_offset += dy;
                },
            }
        }
        
        response
    }
}
