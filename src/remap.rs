use eframe::egui::Key;

use crate::key_matrix::Chip8Key;

pub struct RemapState {
    pub open_main: bool,
    pub open_selection: bool,
    pub target_key: Option<Chip8Key>,
    pub selected_key: Option<Key>,
}

impl RemapState {
    pub fn new() -> Self {
        Self {
            open_main: false,
            open_selection: false,
            target_key: None,
            selected_key: None,
        }
    }

    pub fn reset_selection(&mut self) {
        self.open_selection = false;
        self.target_key = None;
        self.selected_key = None;
    }
}

impl Default for RemapState {
    fn default() -> Self {
        Self::new()
    }
}
