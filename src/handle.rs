use std::{
    hint::spin_loop,
    sync::{Arc, Mutex, mpsc::TryRecvError},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::{
    Message,
    channel::Channel,
    chip8::Chip8,
    chip8_state::Chip8State,
    frame_buffer::FrameBuffer,
    key_matrix::{Chip8Key, KeyMatrix},
};

pub struct Chip8Handle {
    handle: Option<JoinHandle<()>>,
    channel: Option<Channel>,
}

pub enum Chip8Source {
    ROM(String),
    SaveState(String),
}

impl Chip8Handle {
    pub fn new(
        key_matrix: Arc<Mutex<KeyMatrix>>,
        frame_buffer: Arc<Mutex<FrameBuffer>>,

        source: Chip8Source,
    ) -> Self {
        let (channel_1, channel_2) = Channel::new();

        let mut chip8 = match source {
            Chip8Source::ROM(path) => {
                let mut chip8 = Chip8::new(frame_buffer, key_matrix);
                chip8.load_rom(&path).unwrap();
                chip8
            }
            Chip8Source::SaveState(path) => {
                Chip8::new_from_save_state(frame_buffer, key_matrix, Chip8State::load(&path))
            }
        };

        let handle = thread::spawn(move || {
            let tick_rate = Duration::from_millis(2);
            let tick_60hz = Duration::from_millis(17);

            let mut last_update_60hz = Instant::now();
            let mut pause_delta: Duration = Duration::ZERO;

            loop {
                match channel_1.try_recv() {
                    Ok(Message::Shutdown) | Err(TryRecvError::Disconnected) => break,
                    Ok(Message::Pause) => {
                        if !chip8.is_paused() {
                            pause_delta = Instant::now() - last_update_60hz;
                            chip8.pause();
                        }
                    }
                    Ok(Message::Unpause) => {
                        if chip8.is_paused() {
                            last_update_60hz = Instant::now() - pause_delta;
                            chip8.unpause();
                        }
                    }
                    Ok(Message::KeyReleased(val)) => {
                        if !chip8.is_paused() {
                            chip8.set_last_released_key(val);
                        }
                    }
                    Ok(Message::Save(path)) => {
                        chip8.to_chip8_state().save(&path);
                    }
                    _ => {}
                }

                if !chip8.is_paused() {
                    let now = Instant::now();

                    if last_update_60hz.elapsed() >= tick_60hz {
                        chip8.tick_60hz();
                        last_update_60hz = Instant::now();
                    }

                    if let Ok(true) = chip8.tick() {
                        channel_1.send(Message::Draw);
                    }

                    while now.elapsed() < tick_rate {
                        spin_loop();
                    }
                }
            }
        });

        Self {
            handle: Some(handle),
            channel: Some(channel_2),
        }
    }

    pub fn check_draw_message(&self) -> bool {
        if let Some(ref channel) = self.channel {
            if let Ok(Message::Draw) = channel.try_recv() {
                return true;
            }
        }
        false
    }

    pub fn send_key_release_message(&self, key: Chip8Key) {
        if let Some(ref channel) = self.channel {
            channel.send(Message::KeyReleased(key));
        }
    }

    pub fn send_pause_message(&self) {
        if let Some(ref channel) = self.channel {
            channel.send(Message::Pause);
        }
    }

    pub fn send_unpause_message(&self) {
        if let Some(ref channel) = self.channel {
            channel.send(Message::Unpause);
        }
    }

    pub fn save(&self, path: String) {
        if let Some(ref channel) = self.channel {
            channel.send(Message::Save(path));
        }
    }
}

impl Drop for Chip8Handle {
    fn drop(&mut self) {
        let _ = self.channel.take();
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}
