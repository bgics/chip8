use std::{
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

use crate::Message;

pub enum FilePickerResult {
    Path(String),
    None,
}

pub struct FilePicker {
    handle: Option<JoinHandle<()>>,
    receiver: Option<Receiver<Message>>,
}

impl FilePicker {
    pub fn new() -> Self {
        Self {
            handle: None,
            receiver: None,
        }
    }

    pub fn open_file_picker(&mut self) {
        if self.handle.is_some() {
            return;
        }

        let (sender, receiver) = mpsc::channel();

        self.handle = Some(thread::spawn(move || {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                let _ = sender.send(Message::NewROM(path.display().to_string()));
            } else {
                let _ = sender.send(Message::NoFileFound);
            }
        }));

        self.receiver = Some(receiver);
    }

    fn join_handle(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }

    pub fn check_file_picker(&mut self) -> Option<FilePickerResult> {
        let receiver = self.receiver.as_ref()?;

        match receiver.try_recv() {
            Ok(Message::NewROM(path)) => {
                self.join_handle();
                Some(FilePickerResult::Path(path))
            }
            Ok(Message::NoFileFound) => {
                self.join_handle();
                Some(FilePickerResult::None)
            }
            _ => None,
        }
    }
}

impl Drop for FilePicker {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}
