use std::time::Duration;

use tokio::sync::mpsc::Receiver;
use tracing::{error, info};

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

    pub async fn run(&mut self) {
        loop {
            if let Some(message) = self.rx.recv().await {
                info!("send message to {} bot", self.bot.name());

                if let Err(e) = self.bot.send_message(message).await {
                    error!("failed to send message: {:?}", e);
                }

                tokio::time::sleep(Duration::from_secs(self.config.interval)).await;
            }
        }
    }
}
