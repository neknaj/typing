use eframe::egui::{self, Color32, FontId, Pos2};

// アプリケーションの構造体
pub struct MyApp {}

// MyAppのデフォルト実装
impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

/// 横書きルビ付きテキストを描画する関数
fn draw_horizontal_ruby_text(ui: &egui::Ui, pos: Pos2) {
    let painter = ui.painter();
    let main_text = "横書き"; // メインテキスト
    let ruby_texts = ["よこ", "が", ""]; // ルビの注釈

    // フォント設定
    let main_font = FontId::new(30.0, egui::FontFamily::Proportional);
    let ruby_font = FontId::new(15.0, egui::FontFamily::Proportional);

    let mut x = pos.x;
    // ルビテキストとメインテキストのy座標の調整
    let y_ruby = pos.y;
    let y_main = pos.y - 7.0;

    // メインテキストの各文字に対してルビを描画
    for (i, ch) in main_text.chars().enumerate() {
        let char_str = ch.to_string();
        // メイン文字のサイズを取得
        let galley = ui.fonts(|fonts| {
            fonts.layout_no_wrap(char_str.clone(), main_font.clone(), Color32::WHITE)
        });
        let char_width = galley.size().x;
        // ルビテキストのサイズを取得
        let ruby_galley = ui.fonts(|fonts| {
            fonts.layout_no_wrap(ruby_texts[i].to_string(), ruby_font.clone(), Color32::GRAY)
        });
        let ruby_width = ruby_galley.size().x;
        // ルビテキストを中央に配置
        let x_ruby = x + (char_width - ruby_width) / 2.0;
        // メイン文字を描画
        painter.text(
            Pos2::new(x, y_main),
            egui::Align2::LEFT_TOP,
            char_str,
            main_font.clone(),
            Color32::WHITE,
        );
        // ルビテキストを描画
        painter.text(
            Pos2::new(x_ruby, y_ruby),
            egui::Align2::LEFT_BOTTOM,
            ruby_texts[i],
            ruby_font.clone(),
            Color32::GRAY,
        );
        x += char_width;
    }
}

/// 縦書きルビ付きテキストを描画する関数
fn draw_vertical_ruby_text(ui: &egui::Ui, pos: Pos2) {
    let painter = ui.painter();
    let main_text = "縦書き"; // 縦書き用メインテキスト
    let ruby_texts = ["たて", "が", ""]; // ルビの注釈

    // フォント設定
    let main_font = FontId::new(30.0, egui::FontFamily::Proportional);
    let ruby_font = FontId::new(15.0, egui::FontFamily::Proportional);

    let mut y = pos.y;
    // メインテキストの各文字に対してルビを描画
    for (i, ch) in main_text.chars().enumerate() {
        let char_str = ch.to_string();
        // メイン文字の高さを取得
        let galley = ui.fonts(|fonts| {
            fonts.layout_no_wrap(char_str.clone(), main_font.clone(), Color32::WHITE)
        });
        let char_height = galley.size().y;
        // メイン文字を描画
        painter.text(
            Pos2::new(pos.x, y),
            egui::Align2::LEFT_TOP,
            char_str,
            main_font.clone(),
            Color32::WHITE,
        );
        // ルビテキスト全体の高さを計算して中央配置を調整
        let total_ruby_height: f32 = ruby_texts[i]
            .chars()
            .map(|r_ch| {
                let r_str = r_ch.to_string();
                let r_galley = ui.fonts(|fonts| {
                    fonts.layout_no_wrap(r_str, ruby_font.clone(), Color32::GRAY)
                });
                r_galley.size().y
            })
            .sum();
        let start_ruby_y = y + (char_height - total_ruby_height) / 2.0;
        // ルビの各文字を縦に描画
        let mut current_ruby_y = start_ruby_y;
        for r_ch in ruby_texts[i].chars() {
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
        y += char_height;
    }
}

// eframeアプリケーションの更新処理
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ヘッダーと説明ラベルを表示
            ui.heading("ルビ付き日本語テキストのサンプル");
            ui.label("下記は横書きと縦書きのルビ付きテキストです。");

            // 横書きテキストのグループ
            ui.group(|ui| {
                ui.label("横書き:");
                let available_rect = ui.available_rect_before_wrap();
                // オフセットの設定
                let pos = available_rect.min + egui::vec2(10.0, 70.0);
                draw_horizontal_ruby_text(ui, pos);
            });

            // 縦書きテキストのグループ
            ui.group(|ui| {
                ui.label("縦書き:");
                let available_rect = ui.available_rect_before_wrap();
                let pos = available_rect.min + egui::vec2(70.0, 80.0);
                draw_vertical_ruby_text(ui, pos);
            });
        });
    }
}
