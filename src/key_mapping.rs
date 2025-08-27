use std::collections::HashMap;

use eframe::egui::Key;

use crate::key_matrix::Chip8Key;

pub struct KeyMapping {
    map: HashMap<Key, Chip8Key>,
}

impl KeyMapping {
    pub fn new() -> Self {
        Self {
            map: Self::default_map(),
        }
    }

    fn default_map() -> HashMap<Key, Chip8Key> {
        let mut map = HashMap::with_capacity(16);
        map.insert(Key::Num1, Chip8Key::K1);
        map.insert(Key::Num2, Chip8Key::K2);
        map.insert(Key::Num3, Chip8Key::K3);
        map.insert(Key::Num4, Chip8Key::KC);
        map.insert(Key::Q, Chip8Key::K4);
        map.insert(Key::W, Chip8Key::K5);
        map.insert(Key::E, Chip8Key::K6);
        map.insert(Key::R, Chip8Key::KD);
        map.insert(Key::A, Chip8Key::K7);
        map.insert(Key::S, Chip8Key::K8);
        map.insert(Key::D, Chip8Key::K9);
        map.insert(Key::F, Chip8Key::KE);
        map.insert(Key::Z, Chip8Key::KA);
        map.insert(Key::X, Chip8Key::K0);
        map.insert(Key::C, Chip8Key::KB);
        map.insert(Key::V, Chip8Key::KF);
        map
    }

    pub fn reset_keymap(&mut self) {
        self.map = Self::default_map();
    }

    pub fn get_chip8_key(&self, key: &Key) -> Option<Chip8Key> {
        self.map.get(key).copied()
    }

    pub fn get_key(&self, chip8_key: Chip8Key) -> Option<Key> {
        self.map
            .iter()
            .find_map(|(k, v)| if *v == chip8_key { Some(*k) } else { None })
    }

    pub fn remap(&mut self, chip8_key: Chip8Key, key: Key) {
        if let Some(old_key) = self
            .map
            .iter()
            .find_map(|(k, v)| if *v == chip8_key { Some(*k) } else { None })
        {
            self.map.remove(&old_key);
        }

        self.map.insert(key, chip8_key);
    }
}

impl Default for KeyMapping {
    fn default() -> Self {
        Self::new()
    }
}
