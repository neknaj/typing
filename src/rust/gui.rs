// Import necessary crates and modules
use eframe::{egui};
use egui::debug_text::print;
use egui::{style, vec2, ScrollArea, Vec2};
use egui_extras::{Column, TableBuilder, Size, StripBuilder};
use js_sys::wasm_bindgen::UnwrapThrowExt;
#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(target_arch = "wasm32")]
use rfd::AsyncFileDialog;

use crate::model::{Model, MenuModel, TypingStartModel, TypingModel, PauseModel, ResultModel, TypingStatus, TextConvert, ErrorMsg, KeyboardRemapping, TypingScroll,TypingSession};
use crate::msg::{Msg, MenuMsg, TypingStartMsg, TypingMsg, PauseMsg, ResultMsg};
use crate::parser::{parse_problem, Content};
use crate::typing::calculate_line_metrics;
use crate::typing::calculate_total_metrics;
use crate::update::update;
use std::collections::HashMap;
use crate::textrender::{RenderText, RenderLineWithRuby, RenderTypingLine, CharOrientation};
#[cfg(target_arch = "wasm32")]
use crate::jsapi;


use std::{sync::Mutex, sync::Once};
// ファイル内容を一時的に保持するためのstatic変数
#[cfg(target_arch = "wasm32")]
static PENDING_CONTENTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
#[cfg(target_arch = "wasm32")]
static INIT: Once = Once::new();
static FILEDIALOG: Mutex<bool> = Mutex::new(false); // フルスクリーンの時にフルスクリーン解除してからファイルダイアログを開く

use eframe::Frame;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[derive(Clone,PartialEq)]
pub enum TextOrientation {
    Vertical,
    Horizontal,
}

pub struct TypingApp {
    dark_mode: bool,
    init: bool,
    typing: Model,
    text_orientation: TextOrientation,
    selected_index: Option<usize>,
    key_released: bool, // 別シーン間のコンボを阻止するやつ 別シーンではキーを押し直す
    fullscreen: bool,
    fullscreen_flag4filedialog: bool,
    scale: f32,
    fps: f32,
    frame_count: u32,                  // Count of frames within the 1-second interval
    last_fps_update: Option<f64>, // Timestamp (in milliseconds) when the frame count was last reset
}

impl Default for TypingApp {
    fn default() -> Self {
        let layout: Vec<(String, Vec<String>)> = serde_json::from_str::<HashMap<String, Vec<String>>>(&include_str!("../../layouts/japanese.json")).unwrap().into_iter().collect();
        Self {
            init: false,
            text_orientation: TextOrientation::Vertical,
            // text_orientation: TextOrientation::Horizontal,
            selected_index: None,
            dark_mode: true,
            key_released: true,
            fullscreen: false,
            fullscreen_flag4filedialog: false,
            scale: 1.0,
            fps: 0.0,
            frame_count: 0,
            last_fps_update: None,   // Initialize with None.
            typing: Model::Menu(
                MenuModel {
                    available_contents: vec![
                        parse_problem(&include_str!("../../examples/いろは歌.ntq")),
                        parse_problem(&include_str!("../../examples/五十音.ntq")),
                        parse_problem(&include_str!("../../examples/平仮名.ntq")),
                        parse_problem(&include_str!("../../examples/MIT.ntq")),
                    ],
                    selecting: 0,
                    error_messages: vec![],
                    layout: TextConvert { mapping: layout },
                }
            ),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl TypingApp {
    fn handle_pending_contents(&mut self) {
        if let Ok(mut contents) = PENDING_CONTENTS.try_lock() {
            for content in contents.drain(..) {
                self.typing = update(self.typing.clone(), Msg::Menu(MenuMsg::AddContent(content)));
            }
        }
    }
}

impl TypingApp {
    #[cfg(not(target_arch = "wasm32"))]
    fn toggle_fullscreen(&mut self, ui: &mut egui::Ui) {
        self.fullscreen = !self.fullscreen;
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));
    }

    #[cfg(target_arch = "wasm32")]
    fn toggle_fullscreen(&mut self, ui: &mut egui::Ui) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        if self.fullscreen {
            let _ = document.exit_fullscreen();
        } else {
            if let Some(body) = document.body() {
                let _ = body.request_fullscreen();
            }
        }
        self.fullscreen = !self.fullscreen;
    }
}

impl eframe::App for TypingApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Apply font scaling once
        if (!self.init) {
            let mut style = (*ctx.style()).clone();
            for (_key, font_id) in style.text_styles.iter_mut() {
                font_id.size *= 2.0;
            }
            ctx.set_style(style);
            self.init = true;
            ctx.set_visuals(egui::Visuals::dark());
            #[cfg(target_arch = "wasm32")]
            {
                crate::jsapi::notify_start();
            }
        }
        let window_height = ctx.input(|input| input.screen_rect().height());
        let window_width = ctx.input(|input| input.screen_rect().width());
        // ctx.set_debug_on_hover(true);
        // Get current time.
        let now = crate::timestamp::now();
        // Frame count for FPS calculation:
        self.frame_count += 1;
        // Initialize last_fps_update if it's not set.
        if self.last_fps_update.is_none() {
            self.last_fps_update = Some(now);
        }
        if let Some(last_update) = self.last_fps_update {
            let div = 2.0;
            if now - last_update >= 1000.0/div {
                // Update FPS, reset the frame count, and update the timestamp.
                self.fps = self.frame_count as f32 * div as f32;
                self.frame_count = 0;
                self.last_fps_update = Some(now);
            }
        }
        {
            // リサイズ
            let info = ctx.input(|i| i.screen_rect());
            let scale = (info.width()*info.height()*self.scale*self.scale).sqrt() /2000.0 * 1.5;
            if ((self.scale / scale).abs() - 1.0).abs() >  0.000001 {
                self.scale = scale;
                ctx.request_repaint();
                ctx.set_pixels_per_point(scale);
                return;
            }
        }

        let cursor_target: f32 = 0.3;

        let typing_font_size = match self.text_orientation {
            TextOrientation::Horizontal => (window_height/8.0).min(window_width/8.0),
            TextOrientation::Vertical => (window_width/8.0).min(window_height/8.0),
        };

        match self.typing.clone() {
            Model::Menu(scene) => {

                egui::SidePanel::right("settings_panel")
                    .resizable(false)
                    .min_width(270.0)
                    .max_width(270.0)
                    .frame(
                        egui::Frame {
                            fill: if self.dark_mode {
                                egui::Color32::from_rgb(6,12,22)
                            } else {
                                egui::Color32::from_rgb(237, 238, 222)
                            },
                            inner_margin: egui::Margin {
                                left: 20,
                                right: 20,
                                top: 20,
                                bottom: 20,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
                        ui.heading("Settings");
                        // Additional settings controls can be added here.
                        if ui.button("Toggle Fullscreen").clicked() {
                            self.toggle_fullscreen(ui);
                        }
                        ui.label("Color Theme");
                        if ui.button(if self.dark_mode {"Dark"} else {"Light"}).clicked() {
                            self.dark_mode = !self.dark_mode;
                            let visuals = if self.dark_mode {
                                egui::Visuals::dark()
                            } else {
                                egui::Visuals::light()
                            };
                            ctx.set_visuals(visuals);
                        }
                        ui.label("Text Orientation");
                        if ui.button(if self.text_orientation==TextOrientation::Vertical {"Vertical"} else {"Horizontal"}).clicked() {
                            if self.text_orientation == TextOrientation::Vertical {
                                self.text_orientation = TextOrientation::Horizontal;
                            }
                            else {
                                self.text_orientation = TextOrientation::Vertical;
                            }
                        }
                    });

                    egui::TopBottomPanel::bottom("bottom_panel")
                    .min_height(window_height*0.3)
                    .max_height(window_height*0.3)
                    .frame(
                        egui::Frame {
                            fill: if self.dark_mode {
                                egui::Color32::from_rgb(6, 9, 15)
                            } else {
                                egui::Color32::from_rgb(237, 238, 222)
                            },
                            inner_margin: egui::Margin {
                                left: 20,
                                right: 20,
                                top: 20,
                                bottom: 20,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
                        if let Some(idx) = self.selected_index {
                            if let Some(content) = scene.available_contents.get(idx) {
                                let mut font = egui::FontSelection::Default.resolve(ui.style());
                                font.size *= 1.5;
                                ui.add(RenderLineWithRuby::new(content.title.clone(), CharOrientation::Horizontal).with_font(font).with_max(window_width));
                                let button_height = 40.0;
                                let button_width = ui.available_width();
                                // Allocate full available space
                                ui.allocate_ui(ui.available_size()-vec2(0.0, button_height), |ui| {
                                    // Ensure the inner content uses the full width
                                    ui.set_min_width(ui.available_size().x);
                                    ScrollArea::both().show(ui, |ui| {
                                        ui.vertical(|ui| {
                                            // Set the width for each section to full width
                                            ui.set_width(ui.available_size().x);
                                            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                                                for line in content.lines.iter() {
                                                    ui.add(RenderLineWithRuby::new(line.clone(), CharOrientation::Horizontal).with_max(window_width));
                                                }
                                            });
                                        });
                                    });
                                });
                                if ui.add_sized(Vec2::new(button_width, button_height), egui::Button::new("Start")).on_hover_text_at_pointer("[Space]").clicked() {
                                    self.typing = update(self.typing.clone(),Msg::Menu(MenuMsg::MoveCursor(idx)));
                                    self.typing = update(self.typing.clone(),Msg::Menu(MenuMsg::Start));
                                }
                            }
                            else {
                                self.selected_index = None;
                            }
                        }
                    });

                // Central Panel for Main Content
                egui::CentralPanel::default()
                    .frame(
                        egui::Frame {
                            fill: if self.dark_mode {
                                egui::Color32::from_rgb(6,5,10)
                            } else {
                                egui::Color32::from_rgb(243, 243, 253)
                            },
                            inner_margin: egui::Margin {
                                left: 20,
                                right: 20,
                                top: 20,
                                bottom: 20,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            let font_title_main = egui::FontId::new(90.0, egui::FontFamily::Name("app_title".into()));
                            ui.label(
                                egui::RichText::new("Neknaj Typing Game")
                                    .font(font_title_main)
                                    .color(ui.style().visuals.strong_text_color()),
                            );
                            let font_title_version = egui::FontId::new(40.0, egui::FontFamily::Name("app_title".into()));
                            let version_text = format!("ver. {}", env!("CARGO_PKG_VERSION"));
                            ui.add_space(-10.0);
                            ui.label(
                            egui::RichText::new(version_text)
                                    .font(font_title_version)
                                    .color(ui.style().visuals.text_color()),
                            );
                        });

                        ui.heading("Menu");
                        // コンテンツ読み込み遅延処理
                        {
                            if let Ok(mut flag) = FILEDIALOG.try_lock() {
                                if *flag {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    {
                                        if let Some(paths) = FileDialog::new().add_filter("Text File", &["txt", "ntq"]).pick_files() {
                                            for path in paths {
                                                match fs::read_to_string(&path) {
                                                    Ok(contents) => {
                                                        // Store file name and contents
                                                        if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                                                            self.typing = update(self.typing.clone(),Msg::Menu(MenuMsg::AddContent(contents)));
                                                        }
                                                    }
                                                    Err(e) => {
                                                        eprintln!("File read error: {}", e);
                                                    }
                                                }
                                            }
                                        }
                                        if self.fullscreen_flag4filedialog {
                                            self.toggle_fullscreen(ui);
                                            self.fullscreen_flag4filedialog = false;
                                        }
                                    }
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        // 保留中のコンテンツを処理
                                        self.handle_pending_contents();
                                        if self.fullscreen_flag4filedialog {
                                            self.toggle_fullscreen(ui);
                                            self.fullscreen_flag4filedialog = false;
                                        }
                                        *flag = false;
                                    }
                                }
                                *flag = false;
                            }
                        }
                        // Button to trigger file open dialog
                        if ui.button("Add Contents").clicked() {
                            // Native environment: use synchronous file dialog for multiple files
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                self.fullscreen_flag4filedialog = self.fullscreen;
                                if self.fullscreen {
                                    self.toggle_fullscreen(ui);
                                }
                                if let Ok(mut flag) = FILEDIALOG.try_lock() {
                                    *flag = true;
                                }
                            }
                            // WASM environment: use asynchronous file dialog for multiple files
                            #[cfg(target_arch = "wasm32")]
                            {
                                self.fullscreen_flag4filedialog = self.fullscreen;
                                if self.fullscreen {
                                    self.toggle_fullscreen(ui);
                                }
                                wasm_bindgen_futures::spawn_local(async move {
                                    if let Some(files) = AsyncFileDialog::new()
                                        .add_filter("Text File", &["txt", "ntq"])
                                        .pick_files()
                                        .await
                                    {
                                        for file in files {
                                            let bytes = file.read().await;
                                            if let Ok(text) = String::from_utf8(bytes) {
                                                if let Ok(mut contents) = PENDING_CONTENTS.try_lock() {
                                                    contents.push(text);
                                                    if let Ok(mut flag) = FILEDIALOG.try_lock() {
                                                        *flag = true;
                                                    }
                                                }
                                            } else {
                                                web_sys::console::log_1(&"Invalid UTF-8 data.".into());
                                            }
                                        }
                                    }
                                });
                            }
                        }

                        // Calculate common button size
                        let button_height = 40.0;
                        let button2_width = 130.0;
                        let button1_width = ui.available_width() - button2_width;

                        let spacing = ui.spacing().item_spacing.y;

                        // Display menu items in a scrollable area with delete buttons
                        ui.add_space(spacing);
                        ScrollArea::vertical().show(ui, |ui| {
                            for (index, item) in scene.available_contents.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    // Menu item button: selecting an item sets selected_index
                                    if ui.add_sized(Vec2::new(button1_width, button_height), egui::Button::new(item.title.to_string().clone())).clicked() {
                                        self.selected_index = Some(index);
                                    }
                                    // Delete button with a fixed small width
                                    if ui.add_sized(Vec2::new(button2_width, button_height), egui::Button::new("Delete")).clicked() {
                                        // If the deleted item is the selected one, clear selected_index
                                        if self.selected_index == Some(index) {
                                            self.selected_index = None;
                                        }
                                        // Remove the item from the list
                                        // Note: scene.available_contents must be mutable for this to work
                                        // Example: scene.available_contents.remove(index);
                                    }
                                });
                                ui.add_space(spacing);
                            }
                        });
                    });
                ctx.input(|i| {
                    for event in &i.events {
                        match event {
                            egui::Event::Key { key, pressed, .. } => {
                                if *pressed && self.key_released {
                                    // キーが押されたときの処理
                                    match key {
                                        egui::Key::Space => {
                                            if let Some(idx) = self.selected_index {
                                                self.typing = update(self.typing.clone(),Msg::Menu(MenuMsg::MoveCursor(idx)));
                                                self.typing = update(self.typing.clone(),Msg::Menu(MenuMsg::Start));
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            egui::Event::Text(text) => {
                            }
                            _ => {}
                        }
                    }
                });
            },
            Model::TypingStart(scene) => {
                let content: &Content = &scene.content;
                egui::CentralPanel::default()
                    .frame(
                        egui::Frame {
                            fill: if self.dark_mode {
                                egui::Color32::from_rgb(6,5,10)
                            } else {
                                egui::Color32::from_rgb(243, 243, 253)
                            },
                            inner_margin: egui::Margin {
                                left: 20,
                                right: 20,
                                top: 20,
                                bottom: 20,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
                    });
                let mut font = egui::FontId::new(typing_font_size, egui::FontFamily::Proportional);
                if self.text_orientation == TextOrientation::Vertical {
                    egui::Area::new("centent_title".into())
                        .fixed_pos(egui::Pos2::new(window_width-typing_font_size*0.5, 0.0))
                        .show(ctx, |ui| {
                            let line = RenderLineWithRuby::new(content.title.clone(), CharOrientation::Vertical).with_font(egui::FontId::new(typing_font_size*0.7, egui::FontFamily::Proportional)).with_max(window_height);
                            let scroll_to = line.calc_size(ui).0;
                            ui.add(line.with_offset(-window_height*0.5+scroll_to*0.5));
                        });
                    egui::Area::new("centered_text2".into())
                        .fixed_pos(egui::Pos2::new(window_width/2.0+typing_font_size*0.5, 0.0))
                        .show(ctx, |ui| {
                            ui.add(RenderLineWithRuby::new(content.lines[0].clone(), CharOrientation::Vertical).with_font(font.clone()).with_offset(-window_height*cursor_target).with_max(window_height));
                        });
                } else {
                    egui::Area::new("content_title".into())
                        .fixed_pos(egui::Pos2::new(0.0, typing_font_size*0.1))
                        .show(ctx, |ui| {
                            let line = RenderLineWithRuby::new(content.title.clone(), CharOrientation::Horizontal).with_font(egui::FontId::new(typing_font_size*0.7, egui::FontFamily::Proportional)).with_max(window_width);
                            let scroll_to = line.calc_size(ui).0;
                            ui.add(line.with_offset(-window_width*0.5+scroll_to*0.5));
                        });
                    egui::Area::new("centered_text2".into())
                        .fixed_pos(egui::Pos2::new(0.0, window_height/2.0-typing_font_size*2.0))
                        .show(ctx, |ui| {
                            ui.add(RenderLineWithRuby::new(content.lines[0].clone(), CharOrientation::Horizontal).with_font(font.clone()).with_offset(-window_width*cursor_target).with_max(window_width));
                        });
                }
                egui::Area::new("full_screen_overlay".into()) // オーバーレイ
                    .fixed_pos(egui::Pos2::new(0.0, 0.0))
                    .interactable(true)
                    .order(egui::Order::Foreground)
                    .show(ctx, |ui| {
                        // Get the full screen rectangle.
                        let screen_rect = ctx.input(|i| i.screen_rect());
                        // Allocate the full screen size.
                        let (rect, _) = ui.allocate_exact_size(screen_rect.size(), egui::Sense::hover());
                        // Draw a semi-transparent background with slight rounding.
                        ui.painter().rect_filled(
                            rect,
                            egui::Rounding::same(0),
                            egui::Color32::from_rgba_premultiplied(0, 0, 0, 230),
                        );
                        // Display overlay text in the center.
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Press [Space] to Start",
                            egui::FontId::proportional(80.0),
                            egui::Color32::WHITE,
                        );
                    });
                    ctx.input(|i| {
                        for event in &i.events {
                            match event {
                                egui::Event::Key { key, pressed, .. } => {
                                    if *pressed && self.key_released {
                                        match key {
                                            egui::Key::Space => {
                                                let scrollmax = match self.text_orientation {
                                                    TextOrientation::Horizontal => window_width,
                                                    TextOrientation::Vertical => window_height,
                                                };
                                                self.typing = update(self.typing.clone(),Msg::TypingStart(TypingStartMsg::StartTyping));
                                                self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::ScrollTo((-scrollmax*cursor_target) as f64, -scrollmax as f64)));
                                            }
                                            egui::Key::Escape => {
                                                self.typing = update(self.typing.clone(),Msg::TypingStart(TypingStartMsg::Cancel));
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                egui::Event::Text(text) => {
                                }
                                _ => {}
                            }
                        }
                    });
            },
            Model::Typing(scene) => {
                let content: Content = scene.content.clone();
                egui::CentralPanel::default()
                    .frame(
                        egui::Frame {
                            fill: if self.dark_mode {
                                egui::Color32::from_rgb(6,5,10)
                            } else {
                                egui::Color32::from_rgb(243, 243, 253)
                            },
                            inner_margin: egui::Margin {
                                left: 20,
                                right: 20,
                                top: 20,
                                bottom: 20,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
                    });
                let mut font = egui::FontId::new(typing_font_size, egui::FontFamily::Proportional);
                let scrollmax = match self.text_orientation {
                    TextOrientation::Horizontal => window_width,
                    TextOrientation::Vertical => window_height,
                };

                // リアルタイムステータス表示を左下に配置
                let stat = calculate_total_metrics(&scene);
                egui::Area::new("status_table".into())
                .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(30.0, -30.0))
                    .show(ctx, |ui| {
                        let table_width = 300.0;
                        TableBuilder::new(ui)
                            .striped(true)
                            .resizable(false)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto().at_least(100.0))
                            .column(Column::remainder().at_least(100.0))
                            .min_scrolled_height(0.0)
                            // .background_color(egui::Color32::from_rgba_premultiplied(0, 0, 0, 180))
                            .body(|mut body| {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| { ui.label("Speed"); });
                                    row.col(|ui| { ui.label(format!("{:.3} KPS", stat.speed)); });
                                });
                                body.row(30.0, |mut row| {
                                    row.col(|ui| { ui.label("Accuracy"); });
                                    row.col(|ui| { ui.label(format!("{:.3}%", stat.accuracy * 100.0)); });
                                });
                                body.row(30.0, |mut row| {
                                    row.col(|ui| { ui.label("Keystrokes"); });
                                    row.col(|ui| { ui.label(format!("{}", stat.type_count + stat.miss_count)); });
                                });
                                body.row(30.0, |mut row| {
                                    row.col(|ui| { ui.label("Mistyped"); });
                                    row.col(|ui| { ui.label(format!("{} ({:.3}%)", stat.miss_count, (stat.miss_count as f64 / (stat.type_count + stat.miss_count) as f64) * 100.0)); });
                                });
                                let total_seconds = stat.total_time / 1000.0;
                                let hours = (total_seconds / 3600.0).floor();
                                let minutes = ((total_seconds % 3600.0) / 60.0).floor();
                                let seconds = total_seconds % 60.0;
                                body.row(30.0, |mut row| {
                                    row.col(|ui| { ui.label("Time"); });
                                    row.col(|ui| { ui.label(format!("{:02.0}:{:02.0}:{:05.2}", hours, minutes, seconds)); });
                                });
                            });
                    });

                if self.text_orientation == TextOrientation::Vertical {
                    egui::Area::new("centent_title".into())
                        .fixed_pos(egui::Pos2::new(window_width-typing_font_size*0.5, 0.0))
                        .show(ctx, |ui| {
                            let line = RenderLineWithRuby::new(content.title.clone(), CharOrientation::Vertical).with_font(egui::FontId::new(typing_font_size*0.7, egui::FontFamily::Proportional)).with_max(window_height);
                            let scroll_to = line.calc_size(ui).0;
                            ui.add(line.with_offset(-window_height*0.5+scroll_to*0.5));
                        });
                    egui::Area::new("centered_text2".into())
                        .fixed_pos(egui::Pos2::new(window_width/2.0+typing_font_size*0.5, 0.0))
                        .show(ctx, |ui| {
                            ui.add(RenderLineWithRuby::new(content.lines[scene.status.line as usize].clone(), CharOrientation::Vertical).with_font(font.clone()).with_offset(scene.scroll.scroll as f32).with_max(window_height));
                        });
                    egui::Area::new("centered_text1".into())
                        .fixed_pos(egui::Pos2::new(window_width/2.0-typing_font_size*1.0, 0.0))
                        .show(ctx, |ui| {
                            let line = RenderTypingLine::new(content.lines[scene.status.line as usize].clone(), scene.typing_correctness.lines[scene.status.line as usize].clone(), scene.status.clone(), CharOrientation::Vertical).with_font(font.clone()).with_offset(scene.scroll.scroll as f32);
                            let scrollto = line.calc_size(ui).0-window_height*cursor_target;
                            let now = scene.scroll.scroll as f32;
                            let d = scrollto-now;
                            let new = now+d* (d*d/(5000000.0+d*d));
                            self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::ScrollTo(new as f64, -scrollmax as f64)));
                            ui.add(line);
                        });
                } else {
                    egui::Area::new("centered_text1".into())
                    .fixed_pos(egui::Pos2::new(0.0, window_height/2.-typing_font_size*0.5))
                        .show(ctx, |ui| {
                            let line = RenderTypingLine::new(content.lines[scene.status.line as usize].clone(), scene.typing_correctness.lines[scene.status.line as usize].clone(), scene.status.clone(), CharOrientation::Horizontal).with_font(font.clone()).with_offset(scene.scroll.scroll as f32);
                            let scrollto = line.calc_size(ui).0-window_width*cursor_target;
                            let now = scene.scroll.scroll as f32;
                            let d = scrollto-now;
                            let new = now+d* (d*d/(5000000.0+d*d));
                            self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::ScrollTo(new as f64, -scrollmax as f64)));
                            ui.add(line);
                        });
                    egui::Area::new("content_title".into())
                        .fixed_pos(egui::Pos2::new(0.0, typing_font_size*0.1))
                        .show(ctx, |ui| {
                            let line = RenderLineWithRuby::new(content.title.clone(), CharOrientation::Horizontal).with_font(egui::FontId::new(typing_font_size*0.7, egui::FontFamily::Proportional)).with_max(window_width);
                            let scroll_to = line.calc_size(ui).0;
                            ui.add(line.with_offset(-window_width*0.5+scroll_to*0.5));
                        });
                    egui::Area::new("centered_text2".into())
                    .fixed_pos(egui::Pos2::new(0.0, window_height/2.0-typing_font_size*2.0))
                        .show(ctx, |ui| {
                            ui.add(RenderLineWithRuby::new(content.lines[scene.status.line as usize].clone(), CharOrientation::Horizontal).with_font(font.clone()).with_offset(scene.scroll.scroll as f32).with_max(window_width));
                        });
                }
                ctx.input(|i| {
                    for event in &i.events {
                        match event {
                            egui::Event::Key { key, pressed, .. } => {
                                if *pressed && self.key_released {
                                    // キーが押されたときの処理
                                    match key {
                                        egui::Key::Escape => {
                                            self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::Pause));
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            egui::Event::Text(text) => {
                                self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::KeyInput(text.chars().collect::<Vec<char>>().get(0).unwrap().clone())));
                            }
                            _ => {}
                        }
                    }
                });
                // フォーカスが外れたらPause画面
                if !ctx.input(|i| i.viewport().focused).unwrap_or(true) {
                    self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::Pause));
                }
            },
            Model::Pause(scene) => {
                let content: Content = scene.typing_model.content.clone();
                egui::CentralPanel::default()
                    .frame(
                        egui::Frame {
                            fill: if self.dark_mode {
                                egui::Color32::from_rgb(6,5,10)
                            } else {
                                egui::Color32::from_rgb(243, 243, 253)
                            },
                            inner_margin: egui::Margin {
                                left: 20,
                                right: 20,
                                top: 20,
                                bottom: 20,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
                    });
                let mut font = egui::FontId::new(typing_font_size, egui::FontFamily::Proportional);
                let scrollmax = match self.text_orientation {
                    TextOrientation::Horizontal => window_width,
                    TextOrientation::Vertical => window_height,
                };

                // リアルタイムステータス表示を左下に配置
                let stat = calculate_total_metrics(&scene.typing_model);
                egui::Area::new("status_table".into())
                    .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(30.0, -30.0))
                    .show(ctx, |ui| {
                        let table_width = 300.0;
                        TableBuilder::new(ui)
                            .striped(true)
                            .resizable(false)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto().at_least(100.0))
                            .column(Column::remainder().at_least(100.0))
                            .min_scrolled_height(0.0)
                            // .background_color(egui::Color32::from_rgba_premultiplied(0, 0, 0, 180))
                            .body(|mut body| {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| { ui.label("Speed"); });
                                    row.col(|ui| { ui.label(format!("{:.3} KPS", stat.speed)); });
                                });
                                body.row(30.0, |mut row| {
                                    row.col(|ui| { ui.label("Accuracy"); });
                                    row.col(|ui| { ui.label(format!("{:.3}%", stat.accuracy * 100.0)); });
                                });
                                body.row(30.0, |mut row| {
                                    row.col(|ui| { ui.label("Keystrokes"); });
                                    row.col(|ui| { ui.label(format!("{}", stat.type_count + stat.miss_count)); });
                                });
                                body.row(30.0, |mut row| {
                                    row.col(|ui| { ui.label("Mistyped"); });
                                    row.col(|ui| { ui.label(format!("{} ({:.3}%)", stat.miss_count, (stat.miss_count as f64 / (stat.type_count + stat.miss_count) as f64) * 100.0)); });
                                });
                                let total_seconds = stat.total_time / 1000.0;
                                let hours = (total_seconds / 3600.0).floor();
                                let minutes = ((total_seconds % 3600.0) / 60.0).floor();
                                let seconds = total_seconds % 60.0;
                                body.row(30.0, |mut row| {
                                    row.col(|ui| { ui.label("Time"); });
                                    row.col(|ui| { ui.label(format!("{:02.0}:{:02.0}:{:05.2}", hours, minutes, seconds)); });
                                });
                            });
                    });

                if self.text_orientation == TextOrientation::Vertical {
                    egui::Area::new("centent_title".into())
                        .fixed_pos(egui::Pos2::new(window_width-typing_font_size*0.5, 0.0))
                        .show(ctx, |ui| {
                            let line = RenderLineWithRuby::new(content.title.clone(), CharOrientation::Vertical).with_font(egui::FontId::new(typing_font_size*0.7, egui::FontFamily::Proportional)).with_max(window_height);
                            let scroll_to = line.calc_size(ui).0;
                            ui.add(line.with_offset(-window_height*0.5+scroll_to*0.5));
                        });
                    egui::Area::new("centered_text2".into())
                        .fixed_pos(egui::Pos2::new(window_width/2.0+typing_font_size*0.5, 0.0))
                        .show(ctx, |ui| {
                            ui.add(RenderLineWithRuby::new(content.lines[scene.typing_model.status.line as usize].clone(), CharOrientation::Vertical).with_font(font.clone()).with_offset(scene.typing_model.scroll.scroll as f32).with_max(window_height));
                        });
                    egui::Area::new("centered_text1".into())
                        .fixed_pos(egui::Pos2::new(window_width/2.0-typing_font_size*1.0, 0.0))
                        .show(ctx, |ui| {
                            let line = RenderTypingLine::new(content.lines[scene.typing_model.status.line as usize].clone(), scene.typing_model.typing_correctness.lines[scene.typing_model.status.line as usize].clone(), scene.typing_model.status.clone(), CharOrientation::Vertical).with_font(font.clone()).with_offset(scene.typing_model.scroll.scroll as f32);
                            let scrollto = line.calc_size(ui).0+window_height*cursor_target;
                            self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::ScrollTo(scrollto as f64, -scrollmax as f64)));
                            ui.add(line);
                        });
                } else {
                    egui::Area::new("centered_text1".into())
                    .fixed_pos(egui::Pos2::new(0.0, window_height/2.-typing_font_size*0.5))
                        .show(ctx, |ui| {
                            let line = RenderTypingLine::new(content.lines[scene.typing_model.status.line as usize].clone(), scene.typing_model.typing_correctness.lines[scene.typing_model.status.line as usize].clone(), scene.typing_model.status.clone(), CharOrientation::Horizontal).with_font(font.clone()).with_offset(scene.typing_model.scroll.scroll as f32);
                            let scrollto = line.calc_size(ui).0-window_width*cursor_target;
                            self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::ScrollTo(scrollto as f64, -scrollmax as f64)));
                            ui.add(line);
                        });
                    egui::Area::new("content_title".into())
                        .fixed_pos(egui::Pos2::new(0.0, typing_font_size*0.1))
                        .show(ctx, |ui| {
                            let line = RenderLineWithRuby::new(content.title.clone(), CharOrientation::Horizontal).with_font(egui::FontId::new(typing_font_size*0.7, egui::FontFamily::Proportional)).with_max(window_width);
                            let scroll_to = line.calc_size(ui).0;
                            ui.add(line.with_offset(-window_width*0.5+scroll_to*0.5));
                        });
                    egui::Area::new("centered_text2".into())
                    .fixed_pos(egui::Pos2::new(0.0, window_height/2.0-typing_font_size*2.0))
                        .show(ctx, |ui| {
                            ui.add(RenderLineWithRuby::new(content.lines[scene.typing_model.status.line as usize].clone(), CharOrientation::Horizontal).with_font(font.clone()).with_offset(scene.typing_model.scroll.scroll as f32).with_max(window_width));
                        });
                }
                egui::Area::new("full_screen_overlay".into()) // オーバーレイ
                    .fixed_pos(egui::Pos2::new(0.0, 0.0))
                    .interactable(true)
                    .order(egui::Order::Foreground)
                    .show(ctx, |ui| {
                        // Get the full screen rectangle.
                        let screen_rect = ctx.input(|i| i.screen_rect());
                        // Allocate the full screen size.
                        let (rect, _) = ui.allocate_exact_size(screen_rect.size(), egui::Sense::hover());
                        // Draw a semi-transparent background with slight rounding.
                        ui.painter().rect_filled(
                            rect,
                            egui::Rounding::same(0),
                            egui::Color32::from_rgba_premultiplied(0, 0, 0, 230),
                        );
                        // Display overlay text in the center.
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Pause\n\nPress [Space] to Resume\nPress [Escape] to Finish",
                            egui::FontId::proportional(80.0),
                            egui::Color32::WHITE,
                        );
                    });
                    ctx.input(|i| {
                        for event in &i.events {
                            match event {
                                egui::Event::Key { key, pressed, .. } => {
                                    if *pressed && self.key_released {
                                        match key {
                                            egui::Key::Space => {
                                                self.typing = update(self.typing.clone(),Msg::Pause(PauseMsg::Resume));
                                            }
                                            egui::Key::Escape => {
                                                self.typing = update(self.typing.clone(),Msg::Pause(PauseMsg::Cancel));
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                egui::Event::Text(text) => {
                                }
                                _ => {}
                            }
                        }
                    });
            },
            Model::Result(scene) => {
                let content: Content = scene.typing_model.content.clone();
                let stat = calculate_total_metrics(&scene.typing_model);
                egui::CentralPanel::default()
                    .frame(
                        egui::Frame {
                            fill: if self.dark_mode {
                                egui::Color32::from_rgb(6,5,10)
                            } else {
                                egui::Color32::from_rgb(243, 243, 253)
                            },
                            inner_margin: egui::Margin {
                                left: 20,
                                right: 20,
                                top: 20,
                                bottom: 20,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
                        // タイトル
                        ui.add_space(50.0);
                        ui.vertical_centered(|ui| {
                            let mut font = egui::FontSelection::Default.resolve(ui.style());
                            font.size *= 3.0;
                            ui.add(RenderLineWithRuby::new(content.title.clone(), CharOrientation::Horizontal).with_font(font).with_max(window_width));
                        });
                        ui.add_space(100.0);

                        // テーブルサイズの制御と中央寄せ
                        let table_width = ui.available_width().min(600.0);
                        let indent = ((ui.available_width() - table_width) / 2.0) as i32;
                        ui.indent(indent, |ui| {
                            TableBuilder::new(ui)
                                .striped(true)
                                .resizable(false)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                .column(Column::auto().at_least(100.0))
                                .column(Column::remainder().at_least(100.0))
                                .min_scrolled_height(0.0)
                                // .background_color(egui::Color32::from_rgba_premultiplied(0, 0, 0, 180))
                                .body(|mut body| {
                                    body.row(30.0, |mut row| {
                                        row.col(|ui| { ui.label("Speed"); });
                                        row.col(|ui| { ui.label(format!("{:.3} KPS", stat.speed)); });
                                    });
                                    body.row(30.0, |mut row| {
                                        row.col(|ui| { ui.label("Accuracy"); });
                                        row.col(|ui| { ui.label(format!("{:.3}%", stat.accuracy * 100.0)); });
                                    });
                                    body.row(30.0, |mut row| {
                                        row.col(|ui| { ui.label("Keystrokes"); });
                                        row.col(|ui| { ui.label(format!("{}", stat.type_count + stat.miss_count)); });
                                    });
                                    body.row(30.0, |mut row| {
                                        row.col(|ui| { ui.label("Mistyped"); });
                                        row.col(|ui| { ui.label(format!("{} ({:.3}%)", stat.miss_count, (stat.miss_count as f64 / (stat.type_count + stat.miss_count) as f64) * 100.0)); });
                                    });
                                    let total_seconds = stat.total_time / 1000.0;
                                    let hours = (total_seconds / 3600.0).floor();
                                    let minutes = ((total_seconds % 3600.0) / 60.0).floor();
                                    let seconds = total_seconds % 60.0;
                                    body.row(30.0, |mut row| {
                                        row.col(|ui| { ui.label("Time"); });
                                        row.col(|ui| { ui.label(format!("{:02.0}:{:02.0}:{:05.2}", hours, minutes, seconds)); });
                                    });
                                });
                        });

                        ui.add_space(100.0);

                        // ボタン
                        ui.vertical_centered(|ui| {
                            let button_width = 300.0;
                            let button_height = 50.0;
                            if ui.add_sized([button_width, button_height], egui::Button::new("Return to Menu")).on_hover_text_at_pointer("[Escape]").clicked() {
                                self.typing = update(self.typing.clone(), Msg::Result(ResultMsg::BackToMenu));
                            }
                            ui.add_space(20.0);
                            if ui.add_sized([button_width, button_height], egui::Button::new("Retry")).on_hover_text_at_pointer("[Space]").clicked() {
                                self.typing = update(self.typing.clone(), Msg::Result(ResultMsg::Retry));
                            }
                        });
                    });
                    ctx.input(|i| {
                        for event in &i.events {
                            match event {
                                egui::Event::Key { key, pressed, .. } => {
                                    if *pressed && self.key_released {
                                        // キーが押されたときの処理
                                        match key {
                                            egui::Key::Space => {
                                                if let Some(idx) = self.selected_index {
                                                    self.typing = update(self.typing.clone(), Msg::Result(ResultMsg::Retry));
                                                }
                                            }
                                            egui::Key::Escape => {
                                                if let Some(idx) = self.selected_index {
                                                    self.typing = update(self.typing.clone(), Msg::Result(ResultMsg::BackToMenu));
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                egui::Event::Text(text) => {
                                }
                                _ => {}
                            }
                        }
                    });
            }
        }
        egui::Area::new("debug_overlay".into())
        .fixed_pos(egui::Pos2::new(0.0, 0.0))
        .interactable(false)
        .show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(80.0, 30.0), egui::Sense::hover());
            ui.painter().rect_filled(
                rect,
                egui::Rounding::same(10),
                egui::Color32::from_rgba_premultiplied(0, 0, 0, 230),
            );
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                &format!("FPS: {}",self.fps),
                egui::FontId::proportional(20.0),
                egui::Color32::WHITE,
            );
        });
        egui::Area::new("key_event".into())
            .interactable(false)
            .show(ctx, |ui| {
                ctx.input(|i| {
                    for event in &i.events {
                        match event {
                            egui::Event::Key { key, pressed, .. } => {
                                // キーが押されたときの処理
                                if *pressed {
                                    // println!("{:?}",key);
                                    match key {
                                        egui::Key::F11 => {
                                            #[cfg(target_arch = "wasm32")]
                                            self.toggle_fullscreen(ui);
                                        }
                                        _ => {}
                                    }
                                    self.key_released = false;
                                }
                                else {
                                    self.key_released = true;
                                }
                            }
                            egui::Event::Text(text) => {
                            }
                            _ => {}
                        }
                    }
                });
            });
        ctx.request_repaint();
    }
}
