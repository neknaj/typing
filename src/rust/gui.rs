// Import necessary crates and modules
use eframe::egui;
use egui::{ScrollArea, Vec2};
#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(target_arch = "wasm32")]
use rfd::AsyncFileDialog;

pub struct MyApp {
    // Vector storing menu item names
    menu_items: Vec<String>,
    // Currently selected menu item index
    selected_index: Option<usize>,
    // Flag to ensure font scaling is applied only once
    font_scaled: bool,
    // Vector to store tuples of (file name, file content)
    file_contents: Vec<(String, String)>,
}

impl Default for MyApp {
    fn default() -> Self {
        // Create several demo menu items
        let mut items = Vec::new();
        for i in 0..10 {
            items.push(format!("Menu Item {}", i));
        }
        Self {
            menu_items: items,
            selected_index: None,
            font_scaled: false,
            file_contents: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply font scaling once
        if !self.font_scaled {
            let mut style = (*ctx.style()).clone();
            for (_key, font_id) in style.text_styles.iter_mut() {
                font_id.size *= 3.0;
            }
            ctx.set_style(style);
            self.font_scaled = true;
        }

        egui::SidePanel::right("settings_panel")
            .resizable(false)
            .min_width(400.0)
            .max_width(400.0)
            .frame(
                egui::Frame {
                    fill: egui::Color32::from_rgb(6, 5, 10),
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
                if ui.button("Option 1").clicked() {
                    // Handle Option 1
                }
                if ui.button("Option 2").clicked() {
                    // Handle Option 2
                }
            });

            egui::TopBottomPanel::bottom("bottom_panel")
                .resizable(false)
                .min_height(400.0)
                .max_height(400.0)
                .frame(
                    egui::Frame {
                        fill: egui::Color32::from_rgb(6, 5, 10),
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
                });

        // Central Panel for Main Content
        egui::CentralPanel::default()
            .frame(
                egui::Frame {
                    fill: egui::Color32::from_rgb(6, 5, 10),
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
                                            self.file_contents.push((filename.to_string(), contents));
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
                    // Iterate through items with their index
                    let mut indices_to_remove = Vec::new();
                    for (index, item) in self.menu_items.iter().enumerate() {
                        ui.horizontal(|ui| {
                            // Menu item button
                            if ui.add_sized(button_size, egui::Button::new(item)).clicked() {
                                self.selected_index = Some(index);
                            }
                            // Delete button with a fixed small width
                            if ui.button("Delete").clicked() {
                            }
                        });
                        ui.add_space(spacing);
                    }
                    // Remove items after the loop to avoid borrow issues
                    // Remove from the end to avoid index shifting
                    for &index in indices_to_remove.iter().rev() {
                        self.menu_items.remove(index);
                        // If the deleted item was selected, clear the selection
                        if self.selected_index == Some(index) {
                            self.selected_index = None;
                        }
                    }
                });
                
                // Display selected menu item
                if let Some(idx) = self.selected_index {
                    ui.label(format!("Selected: {}", self.menu_items[idx]));
                }
                
                // Display file contents if available
                if !self.file_contents.is_empty() {
                    ui.separator();
                    ui.heading("Loaded Files:");
                    for (filename, contents) in &self.file_contents {
                        ui.collapsing(format!("File: {}", filename), |ui| {
                            ui.text_edit_multiline(&mut contents.clone());
                        });
                        ui.add_space(spacing);
                    }
                }
            });
        ctx.request_repaint();
    }
}
