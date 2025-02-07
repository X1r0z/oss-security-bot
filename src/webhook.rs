use tokio::sync::mpsc::Receiver;

use crate::{bot::Push, model::Message};

pub struct Webhook {
    bot: Box<dyn Push>,
    rx: Receiver<Message>,
}

impl Webhook {
    pub fn new(bot: Box<dyn Push>, rx: Receiver<Message>) -> Self {
        Self { bot, rx }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            if let Some(message) = self.rx.recv().await {
                self.bot.send_message(message).await?;
            }
        }
    }
}
