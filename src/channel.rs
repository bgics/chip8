use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

use crate::Message;

pub struct Channel {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Channel {
    pub fn new() -> (Self, Self) {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        (
            Self {
                sender: tx1,
                receiver: rx2,
            },
            Self {
                sender: tx2,
                receiver: rx1,
            },
        )
    }

    pub fn send(&self, msg: Message) {
        let _ = self.sender.send(msg);
    }

    pub fn try_recv(&self) -> Result<Message, TryRecvError> {
        self.receiver.try_recv()
    }
}
