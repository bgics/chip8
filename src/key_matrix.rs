pub struct KeyMatrix {
    matrix: [[bool; 4]; 4],
}

impl Default for KeyMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyMatrix {
    pub fn new() -> Self {
        Self {
            matrix: [[false; 4]; 4],
        }
    }

    pub fn is_pressed(&self, index: usize) -> bool {
        if index > 15 {
            return false;
        }

        let x = index % 4;
        let y = index / 4;

        self.matrix[y][x]
    }

    pub fn press(&mut self, index: usize) {
        if index > 15 {
            return;
        }

        let x = index % 4;
        let y = index / 4;

        self.matrix[y][x] = true;
    }

    pub fn release(&mut self, index: usize) {
        if index > 15 {
            return;
        }

        let x = index % 4;
        let y = index / 4;

        self.matrix[y][x] = false;
    }
}
