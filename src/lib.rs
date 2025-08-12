pub mod app;
pub mod channel;
pub mod chip8;
pub mod cpu;
pub mod error;
pub mod file_picker;
pub mod frame_buffer;
pub mod handle;
pub mod instruction;
pub mod key_matrix;
pub mod memory;

pub enum Message {
    Draw,
    Shutdown,
    Pause,
    Unpause,
    KeyReleased(u8),
    NewROM(String),
    NoFileFound,
}
