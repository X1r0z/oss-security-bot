use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use base64::Engine;
use hmac::{Hmac, Mac};
use serde_json::json;
use sha2::Sha256;

use crate::model::{BotConfig, Message};

use super::Push;

const CARD_TEMPLATE: &str = include_str!("../../templates/LarkCard.json");

pub struct LarkBot {
    config: BotConfig,
}

impl LarkBot {
    pub fn new(config: BotConfig) -> Self {
        Self { config }
    }

    fn generate_signature(&self, timestamp: u64) -> anyhow::Result<String> {
        let secret_key = self.config.secret_key.as_ref().unwrap();
        let string_to_sign = format!("{}\n{}", timestamp, secret_key);
        let mac = Hmac::<Sha256>::new_from_slice(string_to_sign.as_bytes())?;
        let signature_bytes = mac.finalize().into_bytes();

        Ok(base64::prelude::BASE64_STANDARD.encode(signature_bytes))
    }
}

#[async_trait]
impl Push for LarkBot {
    fn name(&self) -> String {
        "lark".to_string()
    }

    async fn send_message(&self, message: Message) -> anyhow::Result<()> {
        let url = format!(
            "https://open.feishu.cn/open-apis/bot/v2/hook/{}",
            self.config.access_token
        );
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
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
