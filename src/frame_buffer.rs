pub struct FrameBuffer {
    buffer: [[bool; 64]; 32],
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [[false; 64]; 32],
        }
    }

    pub fn xor(&mut self, x: usize, y: usize, value: bool) -> bool {
        let old_val = self.buffer[y][x];
        let new_val = old_val ^ value;

        self.buffer[y][x] = new_val;

        old_val && !new_val
    }

    pub fn get_ref(&self) -> &[[bool; 64]; 32] {
        &self.buffer
    }
}
