use eframe::egui;
use crate::textrender::{RenderText, RenderLineWithRuby, CharOrientation};
use crate::parser::{parse_problem,Content,Line};


#[cfg(feature = "web")]
fn timestamp() -> f64 {
    web_sys::window()
    .expect("should have a window")
    .performance()
    .expect("performance should be available")
    .now()
}

#[cfg(not(feature = "web"))]
fn timestamp() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    // Calculate the duration since UNIX_EPOCH.
    let duration = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_millis() as f64
}

/// Sample application to demonstrate the usage of RenderChar, RenderText, and FPS calculation.
pub struct MyApp {
    name: String,
    age: u32,
    dark_mode: bool,
    fps: u32,
    frame_count: u32,                  // Count of frames within the 1-second interval
    last_fps_update: Option<f64>, // Timestamp (in milliseconds) when the frame count was last reset
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_string(),
            age: 42,
            dark_mode: true,
            fps: 0,
            frame_count: 0,
            last_fps_update: None,   // Initialize with None.
        }
    }
}

impl eframe::App for MyApp {
    // Setup function to initialize app settings
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let font = egui::FontId::new(30.0, egui::FontFamily::Proportional);

        // Side panel to toggle the theme.
        egui::SidePanel::left("side_panel")
        .frame(
            egui::Frame {
                fill: if self.dark_mode {
                    egui::Color32::from_rgb(6,12,22)
                } else {
                    egui::Color32::from_rgb(237, 238, 222)
                },
                inner_margin: egui::Margin {
                    left  : 20,
                    right : 20,
                    top   : 20,
                    bottom: 20,
                },
                ..Default::default()
            }
        ).show(ctx, |ui| {
            if ui.button("Toggle Theme").clicked() {
                self.dark_mode = !self.dark_mode;
                let visuals = if self.dark_mode {
                    egui::Visuals::dark()
                } else {
                    egui::Visuals::light()
                };
                ctx.set_visuals(visuals);
            }
        });

        // Get current time.
        let now = timestamp();

        // Frame count for FPS calculation:
        self.frame_count += 1;

        // Initialize last_fps_update if it's not set.
        if self.last_fps_update.is_none() {
            self.last_fps_update = Some(now);
        }

        if let Some(last_update) = self.last_fps_update {
            if now - last_update >= 1000.0 {
                // Update FPS, reset the frame count, and update the timestamp.
                self.fps = self.frame_count;
                self.frame_count = 0;
                self.last_fps_update = Some(now);
            }
        }

        egui::CentralPanel::default()
        .frame(
            egui::Frame {
                fill: if self.dark_mode {
                    egui::Color32::from_rgb(6,5,10)
                } else {
                    egui::Color32::from_rgb(243, 243, 253)
                },
                inner_margin: egui::Margin {
                    left  : 20,
                    right : 20,
                    top   : 20,
                    bottom: 20,
                },
                ..Default::default()
            }
        ).show(ctx, |ui| {
            ui.heading("My egui Application");
            // Display the calculated FPS.
            ui.label(format!("FPS: {:.1}", self.fps));
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.horizontal(|ui| {
                ui.label("Your age: ");
                ui.add(egui::DragValue::new(&mut self.age).speed(1.0));
            });
            ui.label(format!("Hello '{}', you are {} years old!", self.name, self.age));
            if ui.button("Reset").clicked() {
                *self = Self::default();
            }
            ui.separator();
            ui.heading("RenderText and RenderChar Samples");
            ui.separator();
            egui::Grid::new("layout_grid")
            .spacing([30.0, 30.0]) // Set spacing between grid cells.
            .show(ui, |ui| {
                // First grid cell with top_down layout.
            let sample_text = "#title test
    (最近/さいきん)、OpenAI、Microsoft、そしてSoftBankといった(大手/おおて)テック(企業/きぎょう)が、
    (革新的/かくしんてき)なAIおよびGPU(技術/ぎじゅつ)を(活用/かつよう)した(新製品/しんせいひん)・サービスで(大/おお)きく(注目/ちゅうもく)されています。
    OpenAIは(次世代/じせだい)モデル「GPT-4.5」の(開発/かいはつ)と(大規模/だいきぼ)な(資金調達/しきんちょうたつ)を(発表/はっぴょう)し、
    MicrosoftはNVIDIAのH200 GPUを(組/く)み(込/こ)んだCloud(基盤/きばん)の(拡張/かくちょう)を(進/すす)め、
    (先進的/せんしんてき)なAI(処理/しょり)を(加速/かそく)させています。また、SoftBankは(大阪/おおさか)の(旧/きゅう)Sharp LCD(工場/こうじょう)を
    (最新鋭/さいしんえい)のAI Data Centerへと(転換/てんかん)し、(革新的/かくしんてき)なAI Agentモデルの(商用展開/しょうようてんかい)を(目指/めざ)しています。
    
    ルビ(付/つ)きの(美/うつく)しい(横書/よこが)き(日本語/にほんご)を(描画/びょうが)したい
    (横書/よこが)きテキスト Horizontal Text
    I want to write text that combines Japanese and English!
    
    (色/いろ)は(匂/にほ)へど　(散/ち)りぬるを
    (我/わ)が(世/よ)(誰/たれ)ぞ　(常/つね)ならむ
    (有為/うゐ)の(奥山/おくやま)　(今日/けふ)(越/こ)えて
    (浅/あさ)き(夢/ゆめ)(見/み)し　(酔/ゑ)ひもせず
    ";
            let content: Content = parse_problem(sample_text);
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    for line in content.lines {
                        ui.add(RenderLineWithRuby::new(line, CharOrientation::Horizontal).with_font(font.clone()));
                    }
                });
                let sample_text = "#title test
(最近/さいきん)、OpenAI、Microsoft、そしてSoftBankといった(大手/おおて)テック(企業/きぎょう)が、
(革新的/かくしんてき)なAIおよびGPU(技術/ぎじゅつ)を(活用/かつよう)した(新製品/しんせいひん)・サービスで(大/おお)きく(注目/ちゅうもく)されています。
OpenAIは(次世代/じせだい)モデル「GPT-4.5」の(開発/かいはつ)と(大規模/だいきぼ)な(資金調達/しきんちょうたつ)を(発表/はっぴょう)し、
MicrosoftはNVIDIAのH200 GPUを(組/く)み(込/こ)んだCloud(基盤/きばん)の(拡張/かくちょう)を(進/すす)め、
(先進的/せんしんてき)なAI(処理/しょり)を(加速/かそく)させています。また、SoftBankは(大阪/おおさか)の(旧/きゅう)Sharp LCD(工場/こうじょう)を
(最新鋭/さいしんえい)のAI Data Centerへと(転換/てんかん)し、(革新的/かくしんてき)なAI Agentモデルの(商用展開/しょうようてんかい)を(目指/めざ)しています。

ルビ(付/つ)きの(美/うつく)しい(縦書/たてが)き(日本語/にほんご)を(描画/びょうが)したい
(縦書/たてが)きテキスト Vertical Text
I want to write text that combines Japanese and English!

(色/いろ)は(匂/にほ)へど　(散/ち)りぬるを
(我/わ)が(世/よ)(誰/たれ)ぞ　(常/つね)ならむ
(有為/うゐ)の(奥山/おくやま)　(今日/けふ)(越/こ)えて
(浅/あさ)き(夢/ゆめ)(見/み)し　(酔/ゑ)ひもせず
";
    let content: Content = parse_problem(sample_text);
                // Second grid cell with right_to_left layout.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    for line in content.lines {
                        ui.add(RenderLineWithRuby::new(line, CharOrientation::Vertical).with_font(font.clone()));
                    }
                });
                ui.end_row(); // Ends the current row in the grid.
                // Additional rows/cells can be added similarly.
            });
            ctx.set_pixels_per_point(1.5);
        });
        // Request a repaint to continuously update the FPS.
        ctx.request_repaint();
    }
}
