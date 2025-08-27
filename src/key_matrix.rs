use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Chip8Key {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    KA,
    KB,
    KC,
    KD,
    KE,
    KF,
}

impl From<Chip8Key> for u8 {
    fn from(key: Chip8Key) -> Self {
        key as Self
    }
}

impl From<Chip8Key> for &'static str {
    fn from(key: Chip8Key) -> Self {
        match key {
            Chip8Key::K0 => "0",
            Chip8Key::K1 => "1",
            Chip8Key::K2 => "2",
            Chip8Key::K3 => "3",
            Chip8Key::K4 => "4",
            Chip8Key::K5 => "5",
            Chip8Key::K6 => "6",
            Chip8Key::K7 => "7",
            Chip8Key::K8 => "8",
            Chip8Key::K9 => "9",
            Chip8Key::KA => "A",
            Chip8Key::KB => "B",
            Chip8Key::KC => "C",
            Chip8Key::KD => "D",
            Chip8Key::KE => "E",
            Chip8Key::KF => "F",
        }
    }
}

impl TryFrom<u8> for Chip8Key {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Chip8Key::K0),
            0x1 => Ok(Chip8Key::K1),
            0x2 => Ok(Chip8Key::K2),
            0x3 => Ok(Chip8Key::K3),
            0x4 => Ok(Chip8Key::K4),
            0x5 => Ok(Chip8Key::K5),
            0x6 => Ok(Chip8Key::K6),
            0x7 => Ok(Chip8Key::K7),
            0x8 => Ok(Chip8Key::K8),
            0x9 => Ok(Chip8Key::K9),
            0xA => Ok(Chip8Key::KA),
            0xB => Ok(Chip8Key::KB),
            0xC => Ok(Chip8Key::KC),
            0xD => Ok(Chip8Key::KD),
            0xE => Ok(Chip8Key::KE),
            0xF => Ok(Chip8Key::KF),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KeyMatrix {
    bitmask: u16,
}

impl KeyMatrix {
    pub fn new() -> Self {
        Self { bitmask: 0 }
    }

    pub fn load(&mut self, key_matrix: KeyMatrix) {
        self.bitmask = key_matrix.bitmask;
    }

    pub fn is_pressed(&self, key: Chip8Key) -> bool {
        (self.bitmask >> u8::from(key)) & 1 == 1
    }

    pub fn press(&mut self, key: Chip8Key) {
        self.bitmask |= 1 << u8::from(key);
    }

    pub fn release(&mut self, key: Chip8Key) {
        self.bitmask &= !(1 << u8::from(key));
    }
}

impl Default for KeyMatrix {
    fn default() -> Self {
        Self::new()
    }
}
