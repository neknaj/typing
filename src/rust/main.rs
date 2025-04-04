// main.rs
#![cfg(not(feature = "web"))]

mod gui;

fn main() {
    // Print a greeting message for native execution
    println!("Hello World in Native");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| {
            // Configure font definitions
            let mut fonts = egui::FontDefinitions::default();

            // Insert YujiSyuku font
            fonts.font_data.insert(
                "YujiSyuku".to_owned(),
                egui::FontData::from_static(include_bytes!("../fonts/YujiSyuku-Regular.ttf")).into(),
            );

            // Insert Noto Serif JP font
            fonts.font_data.insert(
                "NotoSerifJP".to_owned(),
                egui::FontData::from_static(include_bytes!("../fonts/NotoSerifJP-VariableFont_wght.ttf")).into(),
            );

            // Configure the Proportional font family with YujiSyuku as primary and NotoSerifJP as fallback
            if let Some(proportional) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                proportional.clear();
                proportional.push("YujiSyuku".to_owned());
                proportional.push("NotoSerifJP".to_owned());
            }

            // Optionally, configure the Monospace font family similarly
            if let Some(monospace) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
                monospace.clear();
                monospace.push("YujiSyuku".to_owned());
                monospace.push("NotoSerifJP".to_owned());
            }

            // Apply the customized fonts to the egui context
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(gui::MyApp::default()))
        }),
    ).ok();
}