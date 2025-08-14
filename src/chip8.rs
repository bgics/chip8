use std::{
    fs::File,
    io::{self, Read},
    sync::{Arc, Mutex},
};

use crate::{
    cpu::Cpu,
    error::Result,
    frame_buffer::FrameBuffer,
    key_matrix::{Chip8Key, KeyMatrix},
    memory::Memory,
};

pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,

    frame_buffer: Arc<Mutex<FrameBuffer>>,
    key_matrix: Arc<Mutex<KeyMatrix>>,

    paused: bool,

    last_released_key_index: Option<Chip8Key>,
}

impl Chip8 {
    pub fn new(frame_buffer: Arc<Mutex<FrameBuffer>>, key_matrix: Arc<Mutex<KeyMatrix>>) -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            frame_buffer,
            key_matrix,
            paused: false,
            last_released_key_index: None,
        }
    }

    pub fn set_last_released_key_index(&mut self, key: Chip8Key) {
        self.last_released_key_index = Some(key);
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

    pub fn tick(&mut self) -> Result<bool> {
        self.cpu.tick(
            &mut self.memory,
            self.frame_buffer.clone(),
            self.key_matrix.clone(),
            self.last_released_key_index.take(),
        )
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }
    pub fn unpause(&mut self) {
        self.paused = false;
    }
    pub fn is_paused(&self) -> bool {
        self.paused
    }
}
