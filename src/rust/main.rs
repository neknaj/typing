// main.rs
#![cfg(not(feature = "web"))]

mod gui;

fn main() {
    // cargo run で実行した時に動く処理

    println!("Hello World in Native");

    // gui
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_cc| Ok(Box::new(gui::MyApp::default())))
    ).ok();
}