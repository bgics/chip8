use std::{
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

pub enum Config {
    ROM,
    Load,
    Save,
}

pub enum FilePickerResult {
    ROM(String),
    Load(String),
    Save(String),
    None,
}

pub struct FilePicker {
    handle: Option<JoinHandle<()>>,
    receiver: Option<Receiver<FilePickerResult>>,
}

impl FilePicker {
    pub fn new() -> Self {
        Self {
            handle: None,
            receiver: None,
        }
    }

    pub fn open_file_picker(&mut self, config: Config) {
        if self.handle.is_some() {
            return;
        }

        let (sender, receiver) = mpsc::channel();

        self.handle = Some(thread::spawn(move || match config {
            Config::ROM => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("chip8", &["ch8"])
                    .pick_file()
                {
                    let _ = sender.send(FilePickerResult::ROM(path.display().to_string()));
                } else {
                    let _ = sender.send(FilePickerResult::None);
                }
            }
            Config::Load => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("sav", &["sav"])
                    .pick_file()
                {
                    let _ = sender.send(FilePickerResult::Load(path.display().to_string()));
                } else {
                    let _ = sender.send(FilePickerResult::None);
                }
            }
            Config::Save => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("sav", &["sav"])
                    .save_file()
                {
                    let _ = sender.send(FilePickerResult::Save(path.display().to_string()));
                } else {
                    let _ = sender.send(FilePickerResult::None);
                }
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
            Ok(result) => {
                self.join_handle();
                Some(result)
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
