// main.rs
#![cfg(not(feature = "web"))]

mod gui;
mod textrender;
mod parser;

fn main() {
    // Print a greeting message for native execution
    // println!("Hello World in Native");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 1300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| {
            // Configure font definitions
            let mut fonts = egui::FontDefinitions::default();

            fonts.font_data.insert(
                "KaiseiHarunoUmi".to_owned(),
                egui::FontData::from_static(include_bytes!("../fonts/KaiseiHarunoUmi-Bold.ttf")).into(),
            );

            fonts.font_data.insert(
                "Merienda".to_owned(),
                egui::FontData::from_static(include_bytes!("../fonts/Merienda-Regular.ttf")).into(),
            );

            fonts.font_data.insert(
                "YujiSyuku".to_owned(),
                egui::FontData::from_static(include_bytes!("../fonts/YujiSyuku-Regular.ttf")).into(),
            );

            fonts.font_data.insert(
                "NotoSerifJP".to_owned(),
                egui::FontData::from_static(include_bytes!("../fonts/NotoSerifJP-VariableFont_wght.ttf")).into(),
            );

            // Configure the Proportional font family with YujiSyuku as primary and NotoSerifJP as fallback
            if let Some(proportional) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                proportional.clear();
                proportional.push("NotoSerifJP".to_owned());
            }

            fonts.families.insert(
                egui::FontFamily::Name("main".into()),
                vec!["Merienda".to_owned(), "YujiSyuku".to_owned(), "NotoSerifJP".to_owned()],
            );

            fonts.families.insert(
                egui::FontFamily::Name("kana".into()),
                vec!["KaiseiHarunoUmi".to_owned(), "YujiSyuku".to_owned()],
            );

            fonts.families.insert(
                egui::FontFamily::Name("ruby".into()),
                vec!["YujiSyuku".to_owned(), "KaiseiHarunoUmi".to_owned(), "NotoSerifJP".to_owned()],
            );

            // Apply the customized fonts to the egui context
            cc.egui_ctx.set_fonts(fonts);

            // Set the default theme to dark mode
            let style = egui::Style {
                visuals: egui::Visuals::dark(),
                ..egui::Style::default()
            };
            cc.egui_ctx.set_style(style);

            Ok(Box::new(gui::MyApp::default()))
        }),
    ).ok();
}