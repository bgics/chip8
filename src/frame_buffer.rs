use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

pub const FRAME_BUFFER_ROWS: usize = 32;
pub const FRAME_BUFFER_COLS: usize = 64;

pub const FRAME_BUFFER_SIZE: usize = FRAME_BUFFER_COLS * FRAME_BUFFER_ROWS;

#[derive(Serialize, Deserialize, Clone)]
pub struct FrameBuffer {
    #[serde(with = "BigArray")]
    buffer: [bool; FRAME_BUFFER_SIZE],
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [false; FRAME_BUFFER_SIZE],
        }
    }

    pub fn load(&mut self, frame_buffer: FrameBuffer) {
        self.buffer = frame_buffer.buffer;
    }

    pub fn xor(&mut self, x: usize, y: usize, value: bool) -> bool {
        let pixel_pos = y * FRAME_BUFFER_COLS + x;
        let old_val = self.buffer[pixel_pos];
        let new_val = old_val ^ value;

        self.buffer[pixel_pos] = new_val;

        old_val && !new_val
    }

    pub fn get_ref(&self) -> &[bool; FRAME_BUFFER_SIZE] {
        &self.buffer
    }
}
