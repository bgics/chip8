use std::{
    fs::File,
    hint::spin_loop,
    io::{self, Read},
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender, TryRecvError},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub mod cpu;
pub mod error;
pub mod frame_buffer;
pub mod handle;
pub mod instruction;
pub mod key_matrix;
pub mod memory;

pub mod app;

use cpu::Cpu;
use error::Result;
use memory::Memory;

pub use frame_buffer::FrameBuffer;
pub use key_matrix::KeyMatrix;

#[derive(Debug)]
pub enum Message {
    Draw,
    Shutdown,
    Pause,
    Unpause,
    KeyReleased(u8),
    NewROM(String),
    NoFileFound,
}

pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,

    frame_buffer: Arc<Mutex<FrameBuffer>>,
    key_matrix: Arc<Mutex<KeyMatrix>>,

    paused: bool,

    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Chip8 {
    pub fn new(
        frame_buffer: Arc<Mutex<FrameBuffer>>,
        key_matrix: Arc<Mutex<KeyMatrix>>,
        sender: Sender<Message>,
        receiver: Receiver<Message>,
    ) -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            paused: false,
            frame_buffer,
            key_matrix,
            sender,
            receiver,
        }
    }

    pub fn new_thread_handle(
        frame_buffer: Arc<Mutex<FrameBuffer>>,
        key_matrix: Arc<Mutex<KeyMatrix>>,
        sender: Sender<Message>,
        receiver: Receiver<Message>,
        rom_file_path: &str,
    ) -> JoinHandle<()> {
        let mut chip8 = Chip8::new(frame_buffer, key_matrix, sender, receiver);
        chip8.load_rom(rom_file_path).unwrap();

        thread::spawn(move || {
            let tick_rate = Duration::from_millis(2);
            let tick_60hz = Duration::from_millis(17);

            let mut last_update_60hz = Instant::now();
            let mut pause_delta: Duration = Duration::ZERO;

            loop {
                let mut recv_message = None;
                match chip8.receiver.try_recv() {
                    Ok(Message::Shutdown) | Err(TryRecvError::Disconnected) => break,
                    Ok(Message::Pause) => {
                        pause_delta = Instant::now() - last_update_60hz;
                        chip8.paused = true;
                    }
                    Ok(Message::Unpause) => {
                        last_update_60hz = Instant::now() - pause_delta;
                        chip8.paused = false;
                    }
                    Ok(msg @ Message::KeyReleased(_)) => recv_message = Some(msg),
                    _ => {}
                }

                if !chip8.paused {
                    let now = Instant::now();

                    if last_update_60hz.elapsed() >= tick_60hz {
                        chip8.tick_60hz();
                        last_update_60hz = Instant::now();
                    }

                    let _ = chip8.tick(recv_message);

                    while now.elapsed() < tick_rate {
                        spin_loop();
                    }
                }
            }
        })
    }

    pub fn load_rom(&mut self, file_name: &str) -> io::Result<()> {
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        self.memory.load_rom(&buffer);

        Ok(())
    }

    pub fn tick_60hz(&mut self) {
        self.cpu.tick_60hz();
    }

    pub fn tick(&mut self, recv_message: Option<Message>) -> Result<()> {
        self.cpu.tick(
            &mut self.memory,
            self.frame_buffer.clone(),
            self.key_matrix.clone(),
            self.sender.clone(),
            recv_message,
        )
    }
}
