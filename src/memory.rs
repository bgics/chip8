const FONT_DATA: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

pub const FONT_START_ADDR: u16 = 0x050;
pub const ROM_START_ADDR: u16 = 0x200;

pub const MEMORY_SIZE: usize = 4096;

use crate::error::{Chip8Error, Result};

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Memory {
        let mut memory = Memory {
            data: [0u8; MEMORY_SIZE],
        };
        memory.load_font();
        memory
    }

    pub fn read(&self, addr: u16) -> Result<u8> {
        match self.data.get(addr as usize) {
            Some(value) => Result::Ok(*value),
            None => Result::Err(Chip8Error::OutOfBoundsAccess),
        }
    }

    pub fn write(&mut self, addr: u16, byte: u8) -> Result<()> {
        match self.data.get_mut(addr as usize) {
            Some(value) => {
                *value = byte;
                Result::Ok(())
            }
            None => Result::Err(Chip8Error::OutOfBoundsAccess),
        }
    }

    pub fn load_rom(&mut self, buffer: &[u8]) {
        for (i, &byte) in buffer.iter().enumerate() {
            match self.data.get_mut(ROM_START_ADDR as usize + i) {
                Some(value) => *value = byte,
                None => break,
            }
        }
    }

    fn load_font(&mut self) {
        for (offset, &byte) in FONT_DATA.iter().enumerate() {
            let _ = self.write(offset as u16 + FONT_START_ADDR, byte);
        }
    }
}
