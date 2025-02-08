use std::time::Duration;

use tokio::sync::mpsc::Receiver;

use crate::{
    bot::Push,
    model::{Message, WebhookConfig},
};

pub struct Webhook {
    config: WebhookConfig,
    bot: Box<dyn Push>,
    rx: Receiver<Message>,
}

impl Webhook {
    pub fn new(config: WebhookConfig, bot: Box<dyn Push>, rx: Receiver<Message>) -> Self {
        Self { config, bot, rx }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            if let Some(message) = self.rx.recv().await {
                self.bot.send_message(message).await?;
                tokio::time::sleep(Duration::from_secs(self.config.interval)).await;
            }
        }
    }
}
