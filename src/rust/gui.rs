// Import necessary crates and modules
use eframe::egui;
use egui::debug_text::print;
use egui::{style, vec2, ScrollArea, Vec2};
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
    escape_released: bool, // typing -> pause -> result の escapeコンボを阻止するやつ 2回escapeを押さないとfinishしない
}

impl Default for TypingApp {
    fn default() -> Self {
        let layout: Vec<(String, Vec<String>)> = serde_json::from_str::<HashMap<String, Vec<String>>>(&include_str!("../../layouts/japanese.json")).unwrap().into_iter().collect();
        Self {
            init: false,
            // text_orientation: TextOrientation::Vertical,
            text_orientation: TextOrientation::Horizontal,
            selected_index: None,
            dark_mode: true,
            escape_released: true,
            typing: Model::Menu(
                MenuModel {
                    available_contents: vec![],
                    selecting: 0,
                    error_messages: vec![],
                    layout: TextConvert { mapping: layout },
                }
            ),
        }
    }
}

impl eframe::App for TypingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply font scaling once
        if !self.init {
            let mut style = (*ctx.style()).clone();
            for (_key, font_id) in style.text_styles.iter_mut() {
                font_id.size *= 2.0;
            }
            ctx.set_style(style);
            self.init = true;
        }
        let window_height = ctx.input(|input| input.screen_rect().height());
        let window_width = ctx.input(|input| input.screen_rect().width());

        let typing_font_size = 150.0;

        match self.typing.clone() {
            Model::Menu(scene) => {

                egui::SidePanel::right("settings_panel")
                    .resizable(false)
                    .min_width(400.0)
                    .max_width(400.0)
                    .frame(
                        egui::Frame {
                            fill: if self.dark_mode {
                                egui::Color32::from_rgb(6,12,22)
                            } else {
                                egui::Color32::from_rgb(237, 238, 222)
                            },
                            inner_margin: egui::Margin {
                                left: 30,
                                right: 30,
                                top: 30,
                                bottom: 30,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
                        ui.heading("Settings");
                        // Additional settings controls can be added here.
                        if ui.button("Toggle Theme").clicked() {
                            self.dark_mode = !self.dark_mode;
                            let visuals = if self.dark_mode {
                                egui::Visuals::dark()
                            } else {
                                egui::Visuals::light()
                            };
                            ctx.set_visuals(visuals);
                        }
                        if ui.button("Option 2").clicked() {
                            // Handle Option 2
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
                                left: 30,
                                right: 30,
                                top: 30,
                                bottom: 30,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
                        if let Some(idx) = self.selected_index {
                            if let Some(content) = scene.available_contents.get(idx) {
                                let mut font = egui::FontSelection::Default.resolve(ui.style());
                                font.size *= 1.5;
                                ui.add(RenderText::new(content.title.clone(), CharOrientation::Horizontal).with_font(font));
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
                                                    ui.add(RenderLineWithRuby::new(line.clone(), CharOrientation::Horizontal));
                                                }
                                            });
                                        });
                                    });
                                });
                                if ui.add_sized(Vec2::new(button_width, button_height), egui::Button::new("Start")).clicked() {
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
                                left: 30,
                                right: 30,
                                top: 30,
                                bottom: 30,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
                        ui.heading("Menu");
                        // Button to trigger file open dialog
                        if ui.button("Add Contents").clicked() {
                            // Native environment: use synchronous file dialog for multiple files
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                if let Some(paths) = FileDialog::new()
                                    .add_filter("Text File", &["txt", "ntq"])
                                    .pick_files()
                                {
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
                            }
                            // WASM environment: use asynchronous file dialog for multiple files
                            #[cfg(target_arch = "wasm32")]
                            {
                                let ctx_clone = ctx.clone();
                                wasm_bindgen_futures::spawn_local(async move {
                                    if let Some(files) = AsyncFileDialog::new()
                                        .add_filter("Text File", &["txt", "ntq"])
                                        .pick_files()
                                        .await
                                    {
                                        // For each selected file, read its content
                                        for file in files {
                                            let bytes = file.read().await;
                                            if let Ok(text) = String::from_utf8(bytes) {
                                                // Log file content and name to the browser console
                                                web_sys::console::log_1(&format!("File: {} \nContent: {}", file.file_name(), text).into());
                                                // In a complete implementation, update the UI state accordingly
                                            } else {
                                                web_sys::console::log_1(&"Invalid UTF-8 data.".into());
                                            }
                                        }
                                    } else {
                                        web_sys::console::log_1(&"No files selected.".into());
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
                                    if ui.add_sized(Vec2::new(button1_width, button_height), egui::Button::new(item.title.clone())).clicked() {
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
            },
            Model::TypingStart(scene) => {
                let content: Content = scene.content;
                egui::CentralPanel::default()
                    .frame(
                        egui::Frame {
                            fill: if self.dark_mode {
                                egui::Color32::from_rgb(6,5,10)
                            } else {
                                egui::Color32::from_rgb(243, 243, 253)
                            },
                            inner_margin: egui::Margin {
                                left: 30,
                                right: 30,
                                top: 30,
                                bottom: 30,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
                    });
                let mut font = egui::FontId::new(typing_font_size, egui::FontFamily::Proportional);
                if self.text_orientation == TextOrientation::Vertical {
                    egui::Area::new("centered_text2".into())
                        .fixed_pos(egui::Pos2::new(window_width/2.0+typing_font_size, 0.0))
                        .show(ctx, |ui| {
                            ui.add(RenderLineWithRuby::new(content.lines[0].clone(), CharOrientation::Vertical).with_font(font.clone()).with_offset(-window_height*0.5));
                        });
                } else {
                    egui::Area::new("centered_text2".into())
                        .fixed_pos(egui::Pos2::new(0.0, window_height/2.0-typing_font_size*2.5))
                        .show(ctx, |ui| {
                            ui.add(RenderLineWithRuby::new(content.lines[0].clone(), CharOrientation::Horizontal).with_font(font.clone()).with_offset(-window_width*0.5));
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
                                    // キーが押されたときの処理
                                    match key {
                                        egui::Key::Space => {
                                            let scrollmax = match self.text_orientation {
                                                TextOrientation::Horizontal => window_width,
                                                TextOrientation::Vertical => window_height,
                                            };
                                            self.typing = update(self.typing.clone(),Msg::TypingStart(TypingStartMsg::StartTyping));
                                            self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::ScrollTo((-scrollmax*0.5) as f64, -scrollmax as f64)));
                                        }
                                        egui::Key::Escape => {
                                            self.typing = update(self.typing.clone(),Msg::TypingStart(TypingStartMsg::Cancel));
                                        }
                                        _ => {}
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
                let content: Content = scene.content;
                egui::CentralPanel::default()
                    .frame(
                        egui::Frame {
                            fill: if self.dark_mode {
                                egui::Color32::from_rgb(6,5,10)
                            } else {
                                egui::Color32::from_rgb(243, 243, 253)
                            },
                            inner_margin: egui::Margin {
                                left: 30,
                                right: 30,
                                top: 30,
                                bottom: 30,
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
                if self.text_orientation == TextOrientation::Vertical {
                    egui::Area::new("centered_text1".into())
                        .fixed_pos(egui::Pos2::new(window_width/2.0-typing_font_size, 0.0))
                        .show(ctx, |ui| {
                            let line = RenderTypingLine::new(content.lines[scene.status.line as usize].clone(), scene.typing_correctness.lines[scene.status.line as usize].clone(), scene.status.clone(), CharOrientation::Vertical).with_font(font.clone()).with_offset(scene.scroll.scroll as f32);
                            let scrollto = line.calc_size(ui).0+window_height*0.5;
                            self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::ScrollTo(scrollto as f64, -scrollmax as f64)));
                            ui.add(line);
                        });
                    egui::Area::new("centered_text2".into())
                        .fixed_pos(egui::Pos2::new(window_width/2.0+typing_font_size, 0.0))
                        .show(ctx, |ui| {
                            ui.add(RenderLineWithRuby::new(content.lines[scene.status.line as usize].clone(), CharOrientation::Vertical).with_font(font.clone()).with_offset(scene.scroll.scroll as f32));
                        });
                } else {
                    egui::Area::new("centered_text1".into())
                    .fixed_pos(egui::Pos2::new(0.0, window_height/2.-typing_font_size*0.5))
                        .show(ctx, |ui| {
                            let line = RenderTypingLine::new(content.lines[scene.status.line as usize].clone(), scene.typing_correctness.lines[scene.status.line as usize].clone(), scene.status.clone(), CharOrientation::Horizontal).with_font(font.clone()).with_offset(scene.scroll.scroll as f32);
                            let scrollto = line.calc_size(ui).0-window_width*0.5;
                            let now = scene.scroll.scroll as f32;
                            let d = scrollto-now;
                            let new = now+d* (d*d/(10000000.0+d*d));
                            self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::ScrollTo(new as f64, -scrollmax as f64)));
                            ui.add(line);
                        });
                    egui::Area::new("centered_text2".into())
                    .fixed_pos(egui::Pos2::new(0.0, window_height/2.0-typing_font_size*2.5))
                        .show(ctx, |ui| {
                            ui.add(RenderLineWithRuby::new(content.lines[scene.status.line as usize].clone(), CharOrientation::Horizontal).with_font(font.clone()).with_offset(scene.scroll.scroll as f32));
                        });
                }
                ctx.input(|i| {
                    for event in &i.events {
                        match event {
                            egui::Event::Key { key, pressed, .. } => {
                                if *pressed {
                                    // キーが押されたときの処理
                                    match key {
                                        egui::Key::Escape => {
                                            self.escape_released = false;
                                            self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::Pause));
                                        }
                                        _ => {}
                                    }
                                } else {
                                    // キーが離されたときの処理
                                    match key {
                                        egui::Key::Escape => {
                                            self.escape_released = true;
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
            },
            Model::Pause(scene) => {
                let content: Content = scene.typing_model.content;
                egui::CentralPanel::default()
                    .frame(
                        egui::Frame {
                            fill: if self.dark_mode {
                                egui::Color32::from_rgb(6,5,10)
                            } else {
                                egui::Color32::from_rgb(243, 243, 253)
                            },
                            inner_margin: egui::Margin {
                                left: 30,
                                right: 30,
                                top: 30,
                                bottom: 30,
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
                if self.text_orientation == TextOrientation::Vertical {
                    egui::Area::new("centered_text1".into())
                        .fixed_pos(egui::Pos2::new(window_width/2.0-typing_font_size, 0.0))
                        .show(ctx, |ui| {
                            let line = RenderTypingLine::new(content.lines[scene.typing_model.status.line as usize].clone(), scene.typing_model.typing_correctness.lines[scene.typing_model.status.line as usize].clone(), scene.typing_model.status.clone(), CharOrientation::Vertical).with_font(font.clone()).with_offset(scene.typing_model.scroll.scroll as f32);
                            let scrollto = line.calc_size(ui).0+window_height*0.5;
                            self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::ScrollTo(scrollto as f64, -scrollmax as f64)));
                            ui.add(line);
                        });
                    egui::Area::new("centered_text2".into())
                        .fixed_pos(egui::Pos2::new(window_width/2.0+typing_font_size, 0.0))
                        .show(ctx, |ui| {
                            ui.add(RenderLineWithRuby::new(content.lines[scene.typing_model.status.line as usize].clone(), CharOrientation::Vertical).with_font(font.clone()).with_offset(scene.typing_model.scroll.scroll as f32));
                        });
                } else {
                    egui::Area::new("centered_text1".into())
                    .fixed_pos(egui::Pos2::new(0.0, window_height/2.-typing_font_size*0.5))
                        .show(ctx, |ui| {
                            let line = RenderTypingLine::new(content.lines[scene.typing_model.status.line as usize].clone(), scene.typing_model.typing_correctness.lines[scene.typing_model.status.line as usize].clone(), scene.typing_model.status.clone(), CharOrientation::Horizontal).with_font(font.clone()).with_offset(scene.typing_model.scroll.scroll as f32);
                            let scrollto = line.calc_size(ui).0-window_width*0.5;
                            self.typing = update(self.typing.clone(),Msg::Typing(TypingMsg::ScrollTo(scrollto as f64, -scrollmax as f64)));
                            ui.add(line);
                        });
                    egui::Area::new("centered_text2".into())
                    .fixed_pos(egui::Pos2::new(0.0, window_height/2.0-typing_font_size*2.5))
                        .show(ctx, |ui| {
                            ui.add(RenderLineWithRuby::new(content.lines[scene.typing_model.status.line as usize].clone(), CharOrientation::Horizontal).with_font(font.clone()).with_offset(scene.typing_model.scroll.scroll as f32));
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
                                    if *pressed {
                                        match key {
                                            egui::Key::Space => {
                                                self.typing = update(self.typing.clone(),Msg::Pause(PauseMsg::Resume));
                                            }
                                            egui::Key::Escape => {
                                                if self.escape_released {
                                                    self.typing = update(self.typing.clone(),Msg::Pause(PauseMsg::Cancel));
                                                }
                                                self.escape_released = false;
                                            }
                                            _ => {}
                                        }
                                    }
                                    else {
                                        match key {
                                            egui::Key::Escape => {
                                                self.escape_released = true;
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
                                left: 30,
                                right: 30,
                                top: 30,
                                bottom: 30,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
                        ui.heading("Result");
                        // タイトル
                        ui.add_space(50.0);
                        ui.vertical_centered(|ui| {
                            let mut font = egui::FontSelection::Default.resolve(ui.style());
                            font.size *= 3.0;
                            ui.add(RenderText::new(content.title.clone(), CharOrientation::Horizontal).with_font(font));
                        });
                        ui.add_space(100.0);

                        // 結果表示
                        ui.vertical_centered(|ui| {
                            let mut font = egui::FontSelection::Default.resolve(ui.style());
                            font.size *= 1.5;

                            // 速度（WPM/KPM）
                            let kps = stat.speed;
                            ui.label(egui::RichText::new(format!("Speed: {:.2} KPS", kps)).font(font.clone()));
                            ui.add_space(20.0);

                            // 正確性
                            ui.label(egui::RichText::new(format!("Accuracy: {:.1}%", stat.accuracy * 100.0)).font(font.clone()));
                            ui.add_space(20.0);

                            // タイプ数とミス数
                            ui.label(egui::RichText::new(format!("Total Keystrokes: {}", stat.type_count + stat.miss_count)).font(font.clone()));
                            ui.label(egui::RichText::new(format!("Mistyped: {} ({:.1}%)", 
                                stat.miss_count, 
                                (stat.miss_count as f64 / (stat.type_count + stat.miss_count) as f64) * 100.0
                            )).font(font.clone()));
                            ui.add_space(20.0);

                            // 時間
                            let total_seconds = stat.total_time / 1000.0;
                            let minutes = (total_seconds / 60.0).floor();
                            let seconds = total_seconds % 60.0;
                            ui.label(egui::RichText::new(format!("Time: {:02}:{:02.0}", minutes, seconds)).font(font.clone()));
                        });

                        ui.add_space(100.0);

                        // ボタン
                        ui.vertical_centered(|ui| {
                            let button_width = 300.0;
                            let button_height = 50.0;
                            if ui.add_sized([button_width, button_height], egui::Button::new("Return to Menu")).clicked() {
                                self.typing = update(self.typing.clone(), Msg::Result(ResultMsg::BackToMenu));
                            }
                            ui.add_space(20.0);
                            if ui.add_sized([button_width, button_height], egui::Button::new("Retry")).clicked() {
                                self.typing = update(self.typing.clone(), Msg::Result(ResultMsg::Retry));
                            }
                        });
                    });
            }
        }
        ctx.request_repaint();
    }
}
