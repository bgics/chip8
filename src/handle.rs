use std::{
    hint::spin_loop,
    sync::{Arc, Mutex, mpsc::TryRecvError},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::{
    Message, channel::Channel, chip8::Chip8, frame_buffer::FrameBuffer, key_matrix::Chip8Key,
    key_matrix::KeyMatrix,
};

pub struct Chip8Handle {
    handle: Option<JoinHandle<()>>,
    channel: Option<Channel>,
}

impl Chip8Handle {
    pub fn new(
        key_matrix: Arc<Mutex<KeyMatrix>>,
        frame_buffer: Arc<Mutex<FrameBuffer>>,

        rom_file_path: &str,
    ) -> Self {
        let (channel_1, channel_2) = Channel::new();

        let mut chip8 = Chip8::new(frame_buffer, key_matrix);
        chip8.load_rom(rom_file_path).unwrap();

        let handle = thread::spawn(move || {
            let tick_rate = Duration::from_millis(2);
            let tick_60hz = Duration::from_millis(17);

            let mut last_update_60hz = Instant::now();
            let mut pause_delta: Duration = Duration::ZERO;

            loop {
                match channel_1.try_recv() {
                    Ok(Message::Shutdown) | Err(TryRecvError::Disconnected) => break,
                    Ok(Message::Pause) => {
                        pause_delta = Instant::now() - last_update_60hz;
                        chip8.pause();
                    }
                    Ok(Message::Unpause) => {
                        last_update_60hz = Instant::now() - pause_delta;
                        chip8.unpause();
                    }
                    Ok(Message::KeyReleased(val)) => {
                        if !chip8.is_paused() {
                            chip8.set_last_released_key(val);
                        }
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
                        let _ = channel_1.send(Message::Draw);
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
        return false;
    }

    pub fn send_key_release_message(&self, key: Chip8Key) {
        if let Some(ref channel) = self.channel {
            let _ = channel.send(Message::KeyReleased(key));
        }
    }

    pub fn send_pause_message(&self) {
        if let Some(ref channel) = self.channel {
            let _ = channel.send(Message::Pause);
        }
    }

    pub fn send_unpause_message(&self) {
        if let Some(ref channel) = self.channel {
            let _ = channel.send(Message::Unpause);
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
