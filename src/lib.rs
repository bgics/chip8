use key_matrix::Chip8Key;

pub mod app;
pub mod channel;
pub mod chip8;
pub mod chip8_state;
pub mod cpu;
pub mod error;
pub mod file_picker;
pub mod frame_buffer;
pub mod handle;
pub mod instruction;
pub mod key_mapping;
pub mod key_matrix;
pub mod memory;
pub mod remap;

pub enum Message {
    Draw,
    Shutdown,
    Pause,
    Unpause,
    Save(String),
    KeyReleased(Chip8Key),
}
