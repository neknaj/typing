// main_interactive.rs
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute, terminal,
};
use egui::{CentralPanel, Context, RichText, FontDefinitions, FontFamily};
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use futures::channel::oneshot;
use image::{GrayImage, Luma};
use std::io::{stdout, Write};
use std::num::NonZeroU32;
use std::time::Duration;
use wgpu;

// Function to convert a grayscale image to ASCII art.
fn image_to_ascii(img: &GrayImage) -> String {
    // Define ASCII characters from dark to light.
    let mut ascii_chars = "@@@@@@@@@@@@$B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,'\"^`.            ".chars().collect::<Vec<char>>();
    ascii_chars.reverse();
    let mut ascii_art = String::new();
    for y in 0..img.height() {
        for x in 0..img.width() {
            let Luma([pixel]) = img.get_pixel(x, y);
            let index = (*pixel as f32 / 255.0 * (ascii_chars.len() - 1) as f32).round() as usize;
            // Print each character twice for a better aspect ratio.
            ascii_art.push(ascii_chars[index]);
            // ascii_art.push(ascii_chars[index]);
        }
        ascii_art.push('\n');
    }
    ascii_art
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // --- Terminal Setup ---
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, EnableMouseCapture)?;

    // --- wgpu Initialization (Headless Mode) ---
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        flags: wgpu::InstanceFlags::default(),
        backend_options: Default::default(),
    });
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // Offscreen texture size.
    let width: u32 = 640;
    let height: u32 = 480;
    let texture_extent = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    // Create offscreen texture.
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Offscreen Texture"),
        size: texture_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    // --- egui Initialization ---
    let mut egui_ctx = Context::default();
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "YujiSyuku".to_owned(),
        egui::FontData::from_static(include_bytes!("../fonts/YujiSyuku-Regular.ttf")).into(),
    );
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "YujiSyuku".to_owned());
    egui_ctx.set_fonts(fonts);

    let mut render_pass = RenderPass::new(&device, texture.format(), 1);

    // --- Application State ---
    let mut counter: i32 = 0;

    // --- Main Interactive Loop ---
    'main_loop: loop {
        // Process keyboard input (non-blocking poll)
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                // Only process physical key presses (ignore auto-repeat)
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Esc => break 'main_loop,
                        KeyCode::Up => counter += 1,
                        KeyCode::Down => counter -= 1,
                        _ => {}
                    }
                }
            }
        }

        // Render egui UI with the updated counter.
        egui_ctx.begin_pass(Default::default());
        CentralPanel::default().show(&egui_ctx, |ui| {
            ui.label(RichText::new(format!("カウンタ: {}", counter)).size(25.0));
            ui.label(RichText::new(format!("いろはにほへどちりぬるをわがよたれぞつねならむうゐのおくやまけふこえて")).size(10.0));
            ui.label(RichText::new(format!("色は匂へど散りぬるを")).size(15.0));
            // ui.label(RichText::new(format!("Press 'Escape' to quit.")).size(11.0));
        });
        let full_output = egui_ctx.end_pass();
        let shapes = full_output.shapes;
        let clipped_primitives = egui_ctx.tessellate(shapes, 1.0);

        let screen_desc = ScreenDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor: 1.0,
        };

        // Create command encoder and render UI.
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Encoder") });
        let textures_delta = full_output.textures_delta;
        render_pass.add_textures(&device, &queue, &textures_delta).unwrap();
        render_pass.update_buffers(&device, &queue, &clipped_primitives, &screen_desc);
        render_pass
            .execute(&mut encoder, &view, &clipped_primitives, &screen_desc, None)
            .unwrap();
        queue.submit(Some(encoder.finish()));

        // Copy texture to buffer.
        let buffer_size = (4 * width * height) as wgpu::BufferAddress;
        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Copy Encoder"),
        });
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(NonZeroU32::new(4 * width).unwrap().into()),
                    rows_per_image: Some(NonZeroU32::new(height).unwrap().into()),
                },
            },
            texture_extent,
        );
        queue.submit(Some(encoder.finish()));

        // Map the buffer and convert to ASCII art.
        let buffer_slice = output_buffer.slice(..);
        let (sender, receiver) = oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            sender.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        receiver.await.expect("Failed to receive mapping").unwrap();

        let data = buffer_slice.get_mapped_range();
        // Extract a subregion for the ASCII art.
        let sub_h = 100;
        let sub_w = 180;
        let offset_y = 0;
        let mut gray_image = GrayImage::new(sub_w, sub_h);
        for y in offset_y..sub_h + offset_y {
            for x in 0..sub_w {
                let i = ((y * width + x) * 4) as usize;
                let b = data[i] as f32;
                let g = data[i + 1] as f32;
                let r = data[i + 2] as f32;
                let gray = (0.2126 * r + 0.7152 * g + 0.0722 * b) as u8;
                gray_image.put_pixel(x, y - offset_y, Luma([gray]));
            }
        }
        drop(data);
        output_buffer.unmap();

        let ascii_art = image_to_ascii(&gray_image);
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        println!("{}", ascii_art);
    }

    // --- Cleanup Terminal ---
    execute!(stdout, terminal::LeaveAlternateScreen, DisableMouseCapture)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
