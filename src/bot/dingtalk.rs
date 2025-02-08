use async_trait::async_trait;
use base64::Engine;
use hmac::{Hmac, Mac};
use serde_json::json;
use sha2::Sha256;

use crate::model::{BotConfig, Message};

use super::Push;

pub struct DingTalkBot {
    config: BotConfig,
}

impl DingTalkBot {
    pub fn new(config: BotConfig) -> Self {
        Self { config }
    }

    fn generate_signature(&self, timestamp: u128) -> anyhow::Result<String> {
        let secret_key = self.config.secret_key.as_ref().unwrap();
        let string_to_sign = format!("{}\n{}", timestamp, secret_key);

        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())?;
        mac.update(string_to_sign.as_bytes());

        let signature_bytes = mac.finalize().into_bytes();
        Ok(base64::prelude::BASE64_STANDARD.encode(signature_bytes))
    }
}

#[async_trait]
impl Push for DingTalkBot {
    fn name(&self) -> String {
        "dingtalk".to_string()
    }

    async fn send_message(&self, message: Message) -> anyhow::Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis();
        let signature = self.generate_signature(timestamp)?;

        let url = format!(
            "https://oapi.dingtalk.com/robot/send?access_token={}&timestamp={}&sign={}",
            self.config.access_token,
            timestamp,
            urlencoding::encode(&signature)
        );
        let text = format!(
            "# {title}\n{content}<br/><br/>[{reference}]({reference})",
            title = message.title,
            content = message.content,
            reference = message.reference
        );

        let data = json!({
            "msgtype": "markdown",
            "markdown": {
                "title": message.title,
                "text": text,
            }
        });

        let client = reqwest::Client::new();
        client.post(url).json(&data).send().await?;

        Ok(())
    }
}
