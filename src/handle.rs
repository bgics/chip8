use std::{
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
    thread::JoinHandle,
};

use eframe::egui::{ColorImage, TextureHandle, TextureOptions};

use crate::{Chip8, FrameBuffer, KeyMatrix, Message};

pub struct Chip8Handle {
    handle: Option<JoinHandle<()>>,

    key_matrix: Arc<Mutex<KeyMatrix>>,
    frame_buffer: Arc<Mutex<FrameBuffer>>,

    sender: Option<Sender<Message>>,
    receiver: Receiver<Message>,
}

impl Chip8Handle {
    pub fn new(
        key_matrix: Arc<Mutex<KeyMatrix>>,
        frame_buffer: Arc<Mutex<FrameBuffer>>,
        channel_pair_1: (Sender<Message>, Receiver<Message>),
        channel_pair_2: (Sender<Message>, Receiver<Message>),
        rom_file_path: &str,
    ) -> Self {
        let (tx1, rx1) = channel_pair_1;
        let (tx2, rx2) = channel_pair_2;

        let handle = Some(Chip8::new_thread_handle(
            frame_buffer.clone(),
            key_matrix.clone(),
            tx2,
            rx1,
            rom_file_path,
        ));

        Self {
            handle,
            key_matrix,
            frame_buffer,
            sender: Some(tx1),
            receiver: rx2,
        }
    }

    pub fn shutdown(&mut self) {
        if let Some(handle) = self.handle.take() {
            if let Some(sender) = self.sender.take() {
                let _ = sender.send(Message::Shutdown);
            }

            handle.join().unwrap();
        }
    }

    pub fn set_texture(&self, mut texture_handle: TextureHandle) {
        let frame_buffer = self.frame_buffer.lock().unwrap();
        let frame_buffer_ref = frame_buffer.get_ref();
        let gray_iter = frame_buffer_ref
            .iter()
            .flat_map(|row| row.iter().map(|&v| if v { 255u8 } else { 0u8 }));

        let size = [64, 32];
        let image = ColorImage::from_gray_iter(size, gray_iter);

        texture_handle.set(image, TextureOptions::NEAREST);
    }

    pub fn check_draw_message(&mut self) -> bool {
        if let Ok(Message::Draw) = self.receiver.try_recv() {
            true
        } else {
            false
        }
    }

    pub fn press_key(&mut self, key_index: u8) {
        self.key_matrix.lock().unwrap().press(key_index as usize);
        if let Some(ref sender) = self.sender {
            let _ = sender.send(Message::KeyPressed(key_index));
        }
    }
    pub fn release_key(&mut self, key_index: u8) {
        self.key_matrix.lock().unwrap().release(key_index as usize);
    }
}
