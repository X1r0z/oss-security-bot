use base64::Engine;
use hmac::{Hmac, Mac};
use serde_json::json;
use sha2::Sha256;
use tokio::sync::mpsc::Receiver;
use tracing::info;

use crate::{
    model::{BotConfig, Message},
    CARD_TEMPLATE,
};

pub struct LarkBot {
    config: BotConfig,
    rx: Receiver<Message>,
}

impl LarkBot {
    pub fn new(config: BotConfig, rx: Receiver<Message>) -> Self {
        Self { config, rx }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            if let Some(message) = self.rx.recv().await {
                self.send_message(message).await?;
            }
        }
    }

    pub fn generate_signature(&self, timestamp: i64) -> anyhow::Result<String> {
        let string_to_sign = format!("{}\n{}", timestamp, self.config.secret_key);
        let mac = Hmac::<Sha256>::new_from_slice(string_to_sign.as_bytes())?;
        let signature_bytes = mac.finalize().into_bytes();

        Ok(base64::prelude::BASE64_STANDARD.encode(signature_bytes))
    }

    pub async fn send_message(&self, message: Message) -> anyhow::Result<()> {
        info!("send message to lark bot");

        let url = format!(
            "https://open.feishu.cn/open-apis/bot/v2/hook/{}",
            self.config.access_token
        );
        let timestamp = chrono::Local::now().timestamp();
        let signature = self.generate_signature(timestamp)?;

        let mut card = serde_json::from_str::<serde_json::Value>(CARD_TEMPLATE)?;
        card["body"]["elements"][0]["content"] = format!("**{}**", message.title).into();
        card["body"]["elements"][1]["text"]["content"] = message.content.into();
        card["body"]["elements"][2]["content"] = message.reference.into();

        let data = json!({
            "timestamp": timestamp,
            "sign": signature,
            "msg_type": "interactive",
            "card": card,
        });

        let client = reqwest::Client::new();
        client.post(url).json(&data).send().await?;

        Ok(())
    }
}
