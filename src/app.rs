use std::sync::{Arc, Mutex};

use eframe::{
    Frame,
    egui::{self, ColorImage, Context, MenuBar, TextureHandle, TextureOptions},
};

use crate::{
    file_picker::{FilePicker, FilePickerResult},
    frame_buffer::FrameBuffer,
    handle::Chip8Handle,
    key_mapping::KeyMapping,
    key_matrix::{Chip8Key, KeyMatrix},
};

pub struct App {
    texture: TextureHandle,

    frame_buffer: Arc<Mutex<FrameBuffer>>,
    key_matrix: Arc<Mutex<KeyMatrix>>,

    handle: Option<Chip8Handle>,

    file_picker: FilePicker,
    key_mapping: KeyMapping,

    open_remap_window: bool,
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
            key_mapping: KeyMapping::new(),
            open_remap_window: false,
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

    fn press_key(&self, key: Chip8Key) {
        self.key_matrix.lock().unwrap().press(key);
    }

    fn release_key(&self, key: Chip8Key) {
        self.key_matrix.lock().unwrap().release(key);
        if let Some(ref handle) = self.handle {
            handle.send_key_release_message(key);
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
                        let key_index = self.key_mapping.get_chip8_key(key);

                        if let Some(key_index) = key_index {
                            self.press_key(key_index);
                        }
                    }
                    egui::Event::Key {
                        key,
                        pressed: false,
                        ..
                    } => {
                        let key_index = self.key_mapping.get_chip8_key(key);

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

                    if ui.button("Remap Keys").clicked() {
                        self.open_remap_window = true
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

        egui::Window::new("Remap Keys")
            .open(&mut self.open_remap_window)
            .show(ctx, |ui| {
                let keys = [
                    [Chip8Key::K1, Chip8Key::K2, Chip8Key::K3, Chip8Key::KC],
                    [Chip8Key::K4, Chip8Key::K5, Chip8Key::K6, Chip8Key::KD],
                    [Chip8Key::K7, Chip8Key::K8, Chip8Key::K9, Chip8Key::KE],
                    [Chip8Key::KA, Chip8Key::K0, Chip8Key::KB, Chip8Key::KF],
                ];

                egui::Grid::new("key mapping")
                    .spacing([20.0, 20.0])
                    .show(ui, |ui| {
                        for row in keys {
                            for col in row {
                                ui.vertical(|ui| {
                                    ui.add_space(10.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(10.0);
                                        ui.label(format!(
                                            "{} => {}",
                                            <&'static str>::from(col),
                                            self.key_mapping.get_key(col).name()
                                        ));
                                        if ui.button("Edit").clicked() {
                                            println!("Edit {}", <&'static str>::from(col));
                                        }
                                        ui.add_space(10.0);
                                    });
                                    ui.add_space(10.0);
                                });
                            }
                            ui.end_row();
                        }
                    });
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
