use std::fs::File;

use bincode::{Decode, Encode};

use crate::{
    cpu::Cpu,
    frame_buffer::FrameBuffer,
    key_matrix::{Chip8Key, KeyMatrix},
    memory::Memory,
};

#[derive(Encode, Decode)]
pub struct Chip8State {
    #[bincode(with_serde)]
    pub cpu: Cpu,
    #[bincode(with_serde)]
    pub memory: Memory,
    #[bincode(with_serde)]
    pub frame_buffer: FrameBuffer,
    #[bincode(with_serde)]
    pub key_matrix: KeyMatrix,
    #[bincode(with_serde)]
    pub last_released_key: Option<Chip8Key>,
}

impl Chip8State {
    pub fn save(&self, path: &str) {
        let mut file = File::create(path).expect("Failed to create save file");
        bincode::encode_into_std_write(self, &mut file, bincode::config::standard()).unwrap();
    }

    pub fn load(path: &str) -> Self {
        let mut file = File::open(path).expect("Failed to open save file");
        bincode::decode_from_std_read(&mut file, bincode::config::standard()).unwrap()
    }
}
