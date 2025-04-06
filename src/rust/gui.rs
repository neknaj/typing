// Import necessary crates and modules
use eframe::egui;
use egui::{ScrollArea, Vec2};
#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(target_arch = "wasm32")]
use rfd::AsyncFileDialog;

use crate::model::{Model, MenuModel, TypingStartModel, TypingModel, PauseModel, ResultModel, TypingStatus, TextConvert, ErrorMsg, KeyboardRemapping, TypingScroll,TypingSession};
use crate::msg::{Msg, MenuMsg, TypingStartMsg, TypingMsg, PauseMsg, ResultMsg};
use crate::parser::{parse_problem, Content};
use crate::update::update;
use std::collections::HashMap;


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
}

impl Default for TypingApp {
    fn default() -> Self {
        let layout: Vec<(String, Vec<String>)> = serde_json::from_str::<HashMap<String, Vec<String>>>(&include_str!("../../layouts/japanese.json")).unwrap().into_iter().collect();
        Self {
            init: false,
            text_orientation: TextOrientation::Vertical,
            selected_index: None,
            dark_mode: true,
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
                font_id.size *= 3.0;
            }
            ctx.set_style(style);
            self.init = true;
        }

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
                                left: 50,
                                right: 50,
                                top: 50,
                                bottom: 50,
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
                        .min_height(400.0)
                        .max_height(400.0)
                        .frame(
                            egui::Frame {
                                fill: if self.dark_mode {
                                    egui::Color32::from_rgb(6,9,15)
                                } else {
                                    egui::Color32::from_rgb(237, 238, 222)
                                },
                                inner_margin: egui::Margin {
                                    left: 50,
                                    right: 50,
                                    top: 50,
                                    bottom: 50,
                                },
                                ..Default::default()
                            }
                        )
                        .show(ctx, |ui| {
                            ui.heading("Preview");
                            if let Some(idx) = self.selected_index {
                                ui.label(format!("Selected: {}", scene.available_contents[idx].title));
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
                                left: 50,
                                right: 50,
                                top: 50,
                                bottom: 50,
                            },
                            ..Default::default()
                        }
                    )
                    .show(ctx, |ui| {
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
                        let button_width = ui.available_width() - 100.0; // Adjust width for delete button
                        let button_size = Vec2::new(button_width, button_height);
                        let spacing = ui.spacing().item_spacing.y;

                        // Display menu items in a scrollable area with delete buttons
                        ui.heading("Menu");
                        ui.add_space(spacing);
                        ScrollArea::vertical().show(ui, |ui| {
                            for (index, item) in scene.available_contents.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    // Menu item button
                                    if ui.add_sized(button_size, egui::Button::new(item.title.clone())).clicked() {
                                        self.selected_index = Some(index);
                                    }
                                    // Delete button with a fixed small width
                                    if ui.button("Delete").clicked() {
                                    }
                                });
                                ui.add_space(spacing);
                            }
                        });
                    });
            },
            _ => {}
        }
        ctx.request_repaint();
    }
}
