use eframe::egui;
use eframe::egui::mutex::Mutex;
use std::hint::spin_loop;
use std::sync::mpsc::{Sender, TryRecvError};
use std::sync::{Arc, mpsc};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use std::{env, process, thread};

use chip8::Chip8;

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

    let chip8 = Arc::new(Mutex::new(Chip8::new()));
    chip8.lock().load_rom(&args[1]).unwrap();

    eframe::run_native(
        "chip8",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, chip8)))),
    )
}

struct App {
    frame_buffer: Arc<Mutex<egui::TextureHandle>>,
    handle: Option<JoinHandle<()>>,
    tx: Sender<()>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, chip8: Arc<Mutex<Chip8>>) -> Self {
        let (tx, rx) = mpsc::channel();

        let texture = cc.egui_ctx.load_texture(
            "framebuffer",
            egui::ColorImage::default(),
            egui::TextureOptions::NEAREST,
        );

        let frame_buffer = Arc::new(Mutex::new(texture));

        let frame_buffer_clone = frame_buffer.clone();
        chip8.lock().set_draw_callback(move |frame_buffer| {
            let size = [64, 32];
            let gray_iter = frame_buffer
                .iter()
                .flatten()
                .map(|&v| if v { 255u8 } else { 0u8 });
            let image = egui::ColorImage::from_gray_iter(size, gray_iter);
            frame_buffer_clone
                .lock()
                .set(image, egui::TextureOptions::NEAREST);
        });

        let handle = Some(thread::spawn(move || {
            let tick_rate = Duration::from_millis(2);
            let tick_60hz = Duration::from_millis(17);

            let mut last_update_60hz = Instant::now();

            while let Err(TryRecvError::Empty) = rx.try_recv() {
                let now = Instant::now();

                if last_update_60hz.elapsed() >= tick_60hz {
                    chip8.lock().update_60hz();
                    last_update_60hz = Instant::now();
                }

                chip8.lock().tick();

                while now.elapsed() < tick_rate {
                    spin_loop();
                }
            }
        }));

        Self {
            frame_buffer,
            handle,
            tx,
        }
    }
}

impl std::ops::Drop for App {
    fn drop(&mut self) {
        let _ = self.tx.send(());
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(0))
            .show(ctx, |ui| {
                ui.image((self.frame_buffer.lock().id(), egui::vec2(640.0, 320.0)))
            });

        ctx.request_repaint();
    }
}
