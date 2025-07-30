use eframe::egui;
use eframe::egui::ColorImage;
use eframe::egui::Key;
use eframe::egui::MenuBar;
use eframe::egui::TextureHandle;
use eframe::egui::TextureOptions;
use std::env;
use std::process;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::JoinHandle;

// TODO: Make the code more robust (gracefully handle all errors)
// TODO: Implement pause, loading rom, keymap config (via gui)
// TODO: Add tests

use chip8::{Chip8, FrameBuffer, KeyMatrix, Message};

fn main() -> eframe::Result {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Please provide ROM file.");
        process::exit(1);
    }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 320.0])
            .with_resizable(false)
            .with_title("Chip8"),
        ..Default::default()
    };

    eframe::run_native(
        "chip8",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, &args[1])))),
    )
}

struct App {
    key_matrix: Arc<Mutex<KeyMatrix>>,
    frame_buffer: Arc<Mutex<FrameBuffer>>,

    texture: TextureHandle,

    handle: Option<JoinHandle<()>>,

    sender: Option<Sender<Message>>,
    receiver: Receiver<Message>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, rom_file_path: &str) -> Self {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        let texture = cc.egui_ctx.load_texture(
            "framebuffer",
            ColorImage::default(),
            TextureOptions::NEAREST,
        );

        let frame_buffer = Arc::new(Mutex::new(FrameBuffer::new()));
        let key_matrix = Arc::new(Mutex::new(KeyMatrix::new()));

        let handle = Some(Chip8::spawn_thread(
            frame_buffer.clone(),
            key_matrix.clone(),
            tx2,
            rx1,
            rom_file_path,
        ));

        Self {
            frame_buffer,
            key_matrix,
            texture,
            handle,
            sender: Some(tx1),
            receiver: rx2,
        }
    }

    fn set_new_handle(&mut self, rom_file_path: &str) {
        if let Some(handle) = self.handle.take() {
            if let Some(sender) = self.sender.take() {
                let _ = sender.send(Message::Shutdown);
            }

            handle.join().unwrap()
        }

        let frame_buffer = Arc::new(Mutex::new(FrameBuffer::new()));
        let key_matrix = Arc::new(Mutex::new(KeyMatrix::new()));

        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        self.handle = Some(Chip8::spawn_thread(
            frame_buffer.clone(),
            key_matrix.clone(),
            tx2,
            rx1,
            rom_file_path,
        ));
        self.sender = Some(tx1);
        self.receiver = rx2;
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
}

impl Drop for App {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(Message::Shutdown);
        }

        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            for event in &i.raw.events {
                match event {
                    egui::Event::Key {
                        key, pressed: true, ..
                    } => {
                        let key_index = get_keymap_index(key);

                        if let Some(key_index) = key_index {
                            self.key_matrix.lock().unwrap().press(key_index as usize);
                            if let Some(ref sender) = self.sender {
                                let _ = sender.send(Message::KeyPressed(key_index));
                            }
                        }
                    }
                    egui::Event::Key {
                        key,
                        pressed: false,
                        ..
                    } => {
                        let key_index = get_keymap_index(key);

                        if let Some(key_index) = key_index {
                            self.key_matrix.lock().unwrap().release(key_index as usize);
                        }
                    }
                    _ => {}
                }
            }
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(0))
            .show(ctx, |ui| {
                MenuBar::new().ui(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Load ROM").clicked() {
                            println!("load a rom")
                        }
                    });
                });
                ui.image((self.texture.id(), egui::vec2(640.0, 320.0)))
            });

        if let Ok(Message::Draw) = self.receiver.try_recv() {
            self.set_texture();
            ctx.request_repaint();
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
