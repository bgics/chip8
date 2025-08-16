use std::{
    fs::File,
    io::{self, Read},
    sync::{Arc, Mutex},
};

use crate::{
    chip8_state::Chip8State,
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

    last_released_key: Option<Chip8Key>,
}

impl Chip8 {
    pub fn new(frame_buffer: Arc<Mutex<FrameBuffer>>, key_matrix: Arc<Mutex<KeyMatrix>>) -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            frame_buffer,
            key_matrix,
            paused: false,
            last_released_key: None,
        }
    }

    pub fn new_from_save_state(
        frame_buffer: Arc<Mutex<FrameBuffer>>,
        key_matrix: Arc<Mutex<KeyMatrix>>,
        state: Chip8State,
    ) -> Self {
        frame_buffer.lock().unwrap().load(state.frame_buffer);
        key_matrix.lock().unwrap().load(state.key_matrix);

        Self {
            cpu: state.cpu,
            memory: state.memory,
            frame_buffer,
            key_matrix,
            paused: false,
            last_released_key: state.last_released_key,
        }
    }

    pub fn to_chip8_state(&self) -> Chip8State {
        Chip8State {
            cpu: self.cpu.clone(),
            memory: self.memory.clone(),
            frame_buffer: self.frame_buffer.lock().unwrap().clone(),
            key_matrix: self.key_matrix.lock().unwrap().clone(),
            last_released_key: self.last_released_key.clone(),
        }
    }

    pub fn set_last_released_key(&mut self, key: Chip8Key) {
        self.last_released_key = Some(key);
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
            self.last_released_key.take(),
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
