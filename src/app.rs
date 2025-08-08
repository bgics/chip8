use std::sync::{Arc, Mutex, mpsc};

use eframe::{
    Frame,
    egui::{
        self, ColorImage, Context, Key, MenuBar, TextureHandle, TextureOptions, TopBottomPanel,
    },
};

use crate::{FrameBuffer, KeyMatrix, handle::Chip8Handle};

pub struct App {
    texture: TextureHandle,
    handle: Option<Chip8Handle>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let texture = cc.egui_ctx.load_texture(
            "framebuffer",
            ColorImage::default(),
            TextureOptions::NEAREST,
        );
        Self {
            texture,
            handle: None,
        }
    }

    fn set_new_handle(&mut self, rom_file_path: &str) {
        if let Some(mut handle) = self.handle.take() {
            handle.shutdown();
        }

        let frame_buffer = Arc::new(Mutex::new(FrameBuffer::new()));
        let key_matrix = Arc::new(Mutex::new(KeyMatrix::new()));

        let channel_pair_1 = mpsc::channel();
        let channel_pair_2 = mpsc::channel();

        self.handle = Some(Chip8Handle::new(
            key_matrix,
            frame_buffer,
            channel_pair_1,
            channel_pair_2,
            rom_file_path,
        ))
    }
}

impl Drop for App {
    fn drop(&mut self) {
        if let Some(mut handle) = self.handle.take() {
            handle.shutdown();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.input(|i| {
            for event in &i.raw.events {
                match event {
                    egui::Event::Key {
                        key, pressed: true, ..
                    } => {
                        if let Some(ref mut handle) = self.handle {
                            let key_index = get_keymap_index(key);

                            if let Some(key_index) = key_index {
                                handle.press_key(key_index);
                            }
                        }
                    }
                    egui::Event::Key {
                        key,
                        pressed: false,
                        ..
                    } => {
                        if let Some(ref mut handle) = self.handle {
                            let key_index = get_keymap_index(key);

                            if let Some(key_index) = key_index {
                                handle.release_key(key_index);
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        TopBottomPanel::top("panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load ROM").clicked() {
                        // TODO: Open file picker
                        // TODO: Load the actual ROM
                        self.set_new_handle("/home/bhuvansh/Desktop/chip8/rom/hidden.ch8");
                    }
                });
            });
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(0))
            .show(ctx, |ui| {
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| ui.image((self.texture.id(), egui::vec2(640.0, 320.0))),
                )
            });

        if let Some(ref mut handle) = self.handle {
            if handle.check_draw_message() {
                handle.set_texture(self.texture.clone());
                ctx.request_repaint();
            }
        }

        ctx.request_repaint();
    }
}

fn get_keymap_index(key: &Key) -> Option<u8> {
    match key {
        Key::Num1 => Some(1),
        Key::Num2 => Some(2),
        Key::Num3 => Some(3),
        Key::Num4 => Some(12),
        Key::Q => Some(4),
        Key::W => Some(5),
        Key::E => Some(6),
        Key::R => Some(13),
        Key::A => Some(7),
        Key::S => Some(8),
        Key::D => Some(9),
        Key::F => Some(14),
        Key::Z => Some(10),
        Key::X => Some(0),
        Key::C => Some(11),
        Key::V => Some(15),
        _ => None,
    }
}
