use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
};

use eframe::{
    Frame,
    egui::{self, ColorImage, Context, Key, MenuBar, TextureHandle, TextureOptions},
};

use crate::{FrameBuffer, KeyMatrix, Message, handle::Chip8Handle};

pub struct App {
    texture: TextureHandle,
    handle: Option<Chip8Handle>,
    file_picker_handle: Option<JoinHandle<()>>,
    file_picker_channel: (Sender<Message>, Receiver<Message>),
    open_file_picker: bool,
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
            file_picker_handle: None,
            file_picker_channel: mpsc::channel(),
            open_file_picker: false,
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

        if let Some(handle) = self.file_picker_handle.take() {
            handle.join().unwrap();
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

        egui::TopBottomPanel::top("panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load ROM").clicked() {
                        if let Some(ref handle) = self.handle {
                            handle.pause();
                        }
                        if self.file_picker_handle.is_none() {
                            self.open_file_picker = true;
                        }
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

        if self.open_file_picker {
            self.open_file_picker = false;
            let sender = self.file_picker_channel.0.clone();

            self.file_picker_handle = Some(thread::spawn(move || {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    let _ = sender.send(Message::NewROM(path.display().to_string()));
                } else {
                    let _ = sender.send(Message::NoFileFound);
                }
            }));
        }

        match self.file_picker_channel.1.try_recv() {
            Ok(Message::NewROM(path)) => {
                self.set_new_handle(&path);
                if let Some(ref handle) = self.handle {
                    handle.unpause();
                }
                if let Some(handle) = self.file_picker_handle.take() {
                    handle.join().unwrap();
                }
            }
            Ok(Message::NoFileFound) => {
                if let Some(ref handle) = self.handle {
                    handle.unpause();
                }
                if let Some(handle) = self.file_picker_handle.take() {
                    handle.join().unwrap();
                }
            }
            _ => {}
        }

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
