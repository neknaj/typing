// lib.rs
#![cfg(feature = "web")]
#![cfg(target_arch = "wasm32")]

use eframe::wasm_bindgen::{self, prelude::*};

mod model;
mod msg;
mod update;
mod parser;
mod typing;
mod gui;
mod textrender;
mod timestamp;
mod jsapi;


#[wasm_bindgen]
pub async  fn start_gui() -> Result<(), JsValue> {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlCanvasElement;
    use crate::gui;

    // Redirect panic messages to the browser console
    console_error_panic_hook::set_once();

    // Get the canvas element and convert it to the correct type
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("screen")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    // Web (WASM) version
    let web_options = eframe::WebOptions::default();
    eframe::WebRunner::new()
        .start(
            canvas,
            web_options,
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
                    "Hurricane".to_owned(),
                    egui::FontData::from_static(include_bytes!("../fonts/Hurricane-Regular.ttf")).into(),
                );

                fonts.font_data.insert(
                    "YujiSyuku".to_owned(),
                    egui::FontData::from_static(include_bytes!("../fonts/YujiSyuku-Regular.ttf")).into(),
                );

                fonts.font_data.insert(
                    "ShipporiAntique".to_owned(),
                    egui::FontData::from_static(include_bytes!("../fonts/ShipporiAntique-Regular.ttf")).into(),
                );

                fonts.font_data.insert(
                    "NotoSerifJP".to_owned(),
                    egui::FontData::from_static(include_bytes!("../fonts/NotoSerifJP-VariableFont_wght.ttf")).into(),
                );

                fonts.font_data.insert(
                    "ZhiMangXing".to_owned(),
                    egui::FontData::from_static(include_bytes!("../fonts/ZhiMangXing-Regular.ttf")).into(),
                );

                fonts.font_data.insert(
                    "MaShanZheng".to_owned(),
                    egui::FontData::from_static(include_bytes!("../fonts/MaShanZheng-Regular.ttf")).into(),
                );

                fonts.font_data.insert(
                    "NotoSerif".to_owned(),
                    egui::FontData::from_static(include_bytes!("../fonts/NotoSerif-SemiBold.ttf")).into(),
                );

                // Configure the Proportional font family with YujiSyuku as primary and NotoSerifJP as fallback
                if let Some(proportional) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                    proportional.clear();
                    proportional.push("NotoSerifJP".to_owned());
                }

                fonts.families.insert(
                    egui::FontFamily::Name("main".into()),
                    vec!["Merienda".to_owned(),"MaShanZheng".to_owned(),"YujiSyuku".to_owned(), "ShipporiAntique".to_owned(), "NotoSerifJP".to_owned()],
                );

                fonts.families.insert(
                    egui::FontFamily::Name("kana".into()),
                    vec!["YujiSyuku".to_owned()],
                );

                fonts.families.insert(
                    egui::FontFamily::Name("ruby".into()),
                    vec!["NotoSerif".to_owned(),"ShipporiAntique".to_owned(), "YujiSyuku".to_owned(), "KaiseiHarunoUmi".to_owned(), "NotoSerifJP".to_owned()],
                );

                fonts.families.insert(
                    egui::FontFamily::Name("app_title".into()),
                    vec!["Hurricane".to_owned()],
                );

                // Apply the customized fonts to the egui context
                cc.egui_ctx.set_fonts(fonts);

                // Set the default theme to dark mode
                let style = egui::Style {
                    visuals: egui::Visuals::dark(),
                    ..egui::Style::default()
                };
                cc.egui_ctx.set_style(style);

                Ok(Box::new(gui::TypingApp::default()))
            }),
        )
        .await
        .expect("failed to start eframe");

    Ok(())
}