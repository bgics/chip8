use std::sync::{Arc, Mutex};

use eframe::{
    Frame,
    egui::{self, ColorImage, Context, Key, MenuBar, TextureHandle, TextureOptions},
};

use crate::{
    file_picker::{FilePicker, FilePickerResult},
    frame_buffer::FrameBuffer,
    handle::Chip8Handle,
    key_matrix::KeyMatrix,
};

pub struct App {
    texture: TextureHandle,

    frame_buffer: Arc<Mutex<FrameBuffer>>,
    key_matrix: Arc<Mutex<KeyMatrix>>,

    handle: Option<Chip8Handle>,

    file_picker: FilePicker,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let texture = cc.egui_ctx.load_texture(
            "framebuffer",
            ColorImage::default(),
            TextureOptions::NEAREST,
        );

        let frame_buffer = Arc::new(Mutex::new(FrameBuffer::new()));
        let key_matrix = Arc::new(Mutex::new(KeyMatrix::new()));

        Self {
            texture,
            frame_buffer,
            key_matrix,
            handle: None,
            file_picker: FilePicker::new(),
        }
    }

    fn set_new_handle(&mut self, rom_file_path: &str) {
        let _ = self.handle.take();

        let frame_buffer = Arc::new(Mutex::new(FrameBuffer::new()));
        let key_matrix = Arc::new(Mutex::new(KeyMatrix::new()));

        self.handle = Some(Chip8Handle::new(
            key_matrix.clone(),
            frame_buffer.clone(),
            rom_file_path,
        ));

        self.frame_buffer = frame_buffer;
        self.key_matrix = key_matrix;
    }

    fn set_texture(&mut self) {
        let frame_buffer = self.frame_buffer.lock().unwrap();
        let frame_buffer_ref = frame_buffer.get_ref();
        let gray_iter = frame_buffer_ref
            .iter()
            .flat_map(|row| row.iter().map(|&v| if v { 255u8 } else { 0u8 }));

        let size = [64, 32];
        let image = ColorImage::from_gray_iter(size, gray_iter);

        self.texture.set(image, TextureOptions::NEAREST);
    }

    fn press_key(&self, key_index: u8) {
        self.key_matrix.lock().unwrap().press(key_index as usize);
    }

    fn release_key(&self, key_index: u8) {
        self.key_matrix.lock().unwrap().release(key_index as usize);
        if let Some(ref handle) = self.handle {
            handle.send_key_release_message(key_index);
        }
    }

    fn pause(&self) {
        if let Some(ref handle) = self.handle {
            handle.send_pause_message();
        }
    }

    fn unpause(&self) {
        if let Some(ref handle) = self.handle {
            handle.send_unpause_message();
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
                        let key_index = get_keymap_index(key);

                        if let Some(key_index) = key_index {
                            self.press_key(key_index);
                        }
                    }
                    egui::Event::Key {
                        key,
                        pressed: false,
                        ..
                    } => {
                        let key_index = get_keymap_index(key);

                        if let Some(key_index) = key_index {
                            self.release_key(key_index);
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
                        self.pause();
                        self.file_picker.open_file_picker();
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

        match self.file_picker.check_file_picker() {
            Some(FilePickerResult::Path(path)) => {
                self.set_new_handle(&path);
                self.unpause();
            }
            Some(FilePickerResult::None) => {
                self.unpause();
            }
            None => {}
        }

        if let Some(ref handle) = self.handle {
            if handle.check_draw_message() {
                self.set_texture();
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
