use std::sync::{Arc, Mutex};

use eframe::{
    Frame,
    egui::{self, ColorImage, Context, Key, MenuBar, TextureHandle, TextureOptions},
};

use crate::{
    file_picker::{Config, FilePicker, FilePickerResult},
    frame_buffer::{FRAME_BUFFER_COLS, FRAME_BUFFER_ROWS, FrameBuffer},
    handle::{Chip8Handle, Chip8Source},
    key_mapping::KeyMapping,
    key_matrix::{Chip8Key, KeyMatrix},
    remap::RemapState,
};

pub struct App {
    texture: TextureHandle,

    frame_buffer: Arc<Mutex<FrameBuffer>>,
    key_matrix: Arc<Mutex<KeyMatrix>>,

    handle: Option<Chip8Handle>,

    file_picker: FilePicker,
    key_mapping: KeyMapping,

    remap_state: RemapState,
    open_color_config: bool,
    color_config: [[u8; 3]; 2],
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
            remap_state: RemapState::new(),
            open_color_config: false,
            color_config: [[0u8, 0u8, 0u8], [255u8, 255u8, 255u8]],
        }
    }

    fn set_new_handle(&mut self, source: Chip8Source) {
        let _ = self.handle.take();

        let frame_buffer = Arc::new(Mutex::new(FrameBuffer::new()));
        let key_matrix = Arc::new(Mutex::new(KeyMatrix::new()));

        self.handle = Some(Chip8Handle::new(
            key_matrix.clone(),
            frame_buffer.clone(),
            source,
        ));

        self.frame_buffer = frame_buffer;
        self.key_matrix = key_matrix;
    }

    fn set_texture(&mut self) {
        let image = {
            let frame_buffer = self
                .frame_buffer
                .lock()
                .unwrap()
                .get_ref()
                .map(|v| {
                    if v {
                        self.color_config[1]
                    } else {
                        self.color_config[0]
                    }
                })
                .concat();
            ColorImage::from_rgb([FRAME_BUFFER_COLS, FRAME_BUFFER_ROWS], &frame_buffer)
        };

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

    fn save(&self, path: String) {
        if let Some(ref handle) = self.handle {
            handle.save(path);
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
                        let key = self.key_mapping.get_chip8_key(key);

                        if let Some(key) = key {
                            self.press_key(key);
                        }
                    }
                    egui::Event::Key {
                        key,
                        pressed: false,
                        ..
                    } => {
                        let key = self.key_mapping.get_chip8_key(key);

                        if let Some(key) = key {
                            self.release_key(key);
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
                        self.file_picker.open_file_picker(Config::ROM);
                    }
                    if ui.button("Save State").clicked() {
                        self.pause();
                        self.file_picker.open_file_picker(Config::Save);
                    }
                    if ui.button("Load State").clicked() {
                        self.pause();
                        self.file_picker.open_file_picker(Config::Load);
                    }
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Remap Keys").clicked() {
                        self.pause();
                        self.remap_state.open_main = true;
                    }

                    if ui.button("Reset keymapping").clicked() {
                        self.key_mapping.reset_keymap();
                    }
                });

                ui.menu_button("Config", |ui| {
                    if ui.button("Color Config").clicked() {
                        self.pause();
                        self.open_color_config = true;
                    }
                })
            });
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(ctx.style().visuals.window_fill))
            .show(ctx, |ui| {
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        let scale = 12.0;
                        ui.image((self.texture.id(), egui::vec2(64.0 * scale, 32.0 * scale)));
                    },
                );
            });

        if self.open_color_config {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("color config"),
                egui::ViewportBuilder::default()
                    .with_title("Color Config")
                    .with_inner_size([220.0, 70.0])
                    .with_resizable(false)
                    .with_always_on_top(),
                |ctx, _| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("OFF Color");
                                ui.add(
                                    egui::DragValue::new(&mut self.color_config[0][0])
                                        .range(0..=255),
                                );
                                ui.add(
                                    egui::DragValue::new(&mut self.color_config[0][1])
                                        .range(0..=255),
                                );
                                ui.add(
                                    egui::DragValue::new(&mut self.color_config[0][2])
                                        .range(0..=255),
                                );
                            });
                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                ui.label("ON Color ");
                                ui.add(
                                    egui::DragValue::new(&mut self.color_config[1][0])
                                        .range(0..=255),
                                );
                                ui.add(
                                    egui::DragValue::new(&mut self.color_config[1][1])
                                        .range(0..=255),
                                );
                                ui.add(
                                    egui::DragValue::new(&mut self.color_config[1][2])
                                        .range(0..=255),
                                );
                            });
                        })
                    });
                    ctx.input(|i| {
                        if i.viewport().close_requested() {
                            self.unpause();
                            self.open_color_config = false;
                        }
                    })
                },
            );
        }

        if self.remap_state.open_selection {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("edit key"),
                egui::ViewportBuilder::default()
                    .with_title(format!(
                        "Edit key for {}",
                        self.remap_state
                            .target_key
                            .map(<&'static str>::from)
                            .unwrap_or("")
                    ))
                    .with_inner_size([300.0, 90.0])
                    .with_resizable(false)
                    .with_always_on_top(),
                |ctx, _| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let key = self.remap_state.selected_key.map(|k| k.name()).unwrap_or(
                            self.key_mapping
                                .get_key(self.remap_state.target_key.unwrap())
                                .map(|k| k.name())
                                .unwrap_or("N/A"),
                        );

                        ui.vertical_centered(|ui| {
                            ui.add_space(5.0);
                            ui.label(
                                "Press and release the desired key, then press ENTER to confirm",
                            );
                            ui.add_space(10.0);
                            ui.label(format!("Current key => {key}"));
                        })
                    });
                    ctx.input(|i| {
                        if i.viewport().close_requested() {
                            self.remap_state.reset_selection();
                        }
                        for event in &i.raw.events {
                            if let egui::Event::Key {
                                key,
                                pressed: false,
                                ..
                            } = event
                            {
                                if let Key::Enter = key {
                                    if let (Some(target_key), Some(selected_key)) =
                                        (self.remap_state.target_key, self.remap_state.selected_key)
                                    {
                                        self.key_mapping.remap(target_key, selected_key);
                                    }
                                    self.remap_state.reset_selection();
                                } else {
                                    self.remap_state.selected_key = Some(*key);
                                }
                            }
                        }
                    });
                },
            )
        }

        if self.remap_state.open_main {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("remap window"),
                egui::ViewportBuilder::default()
                    .with_title("Key Remap")
                    .with_inner_size([500.0, 260.0])
                    .with_resizable(false)
                    .with_always_on_top(),
                |ctx, _vi| {
                    let key_layout = [
                        [Chip8Key::K1, Chip8Key::K2, Chip8Key::K3, Chip8Key::KC],
                        [Chip8Key::K4, Chip8Key::K5, Chip8Key::K6, Chip8Key::KD],
                        [Chip8Key::K7, Chip8Key::K8, Chip8Key::K9, Chip8Key::KE],
                        [Chip8Key::KA, Chip8Key::K0, Chip8Key::KB, Chip8Key::KF],
                    ];

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.add_space(10.0);
                        egui::Grid::new("key mapping")
                            .spacing([20.0, 20.0])
                            .show(ui, |ui| {
                                for row in key_layout {
                                    for col in row {
                                        ui.vertical(|ui| {
                                            ui.add_space(10.0);
                                            ui.horizontal(|ui| {
                                                ui.add_space(10.0);
                                                ui.label(format!(
                                                    "{} => {}",
                                                    <&'static str>::from(col),
                                                    self.key_mapping
                                                        .get_key(col)
                                                        .map(|k| k.name())
                                                        .unwrap_or("N/A")
                                                ));
                                                if ui.button("Edit").clicked() {
                                                    self.remap_state.open_selection = true;
                                                    self.remap_state.target_key = Some(col);
                                                }
                                                ui.add_space(10.0);
                                            });
                                            ui.add_space(10.0);
                                        });
                                    }
                                    ui.end_row();
                                }
                            })
                    });
                    ctx.input(|i| {
                        if i.viewport().close_requested() {
                            self.unpause();
                            self.remap_state.reset_selection();
                            self.remap_state.open_main = false;
                        }
                    });
                },
            );
        }

        match self.file_picker.check_file_picker() {
            Some(FilePickerResult::ROM(path)) => {
                self.set_new_handle(Chip8Source::ROM(path));
                self.remap_state.reset_selection();
                self.remap_state.open_main = false;
                self.open_color_config = false;
            }
            Some(FilePickerResult::Load(path)) => {
                self.set_new_handle(Chip8Source::SaveState(path));
                self.remap_state.reset_selection();
                self.remap_state.open_main = false;
                self.open_color_config = false;
            }
            Some(FilePickerResult::Save(path)) => {
                App::save(self, path);
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
    }
}
