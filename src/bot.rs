use base64::Engine;
use hmac::{Hmac, Mac};
use serde_json::json;
use sha2::Sha256;
use tokio::sync::mpsc::Receiver;

pub struct LarkBot {
    access_token: String,
    secret_key: String,
    rx: Receiver<(String, String)>,
}

impl LarkBot {
    pub fn new(access_token: String, secret_key: String, rx: Receiver<(String, String)>) -> Self {
        Self {
            access_token,
            secret_key,
            rx,
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            if let Some((title, content)) = self.rx.recv().await {
                self.send_msg(title, content).await?;
            }
        }
    }

    pub fn generate_signature(&self, timestamp: i64) -> anyhow::Result<String> {
        let string_to_sign = format!("{}\n{}", timestamp, self.secret_key);
        let mac = Hmac::<Sha256>::new_from_slice(string_to_sign.as_bytes())?;
        let signature_bytes = mac.finalize().into_bytes();

        Ok(base64::prelude::BASE64_STANDARD.encode(signature_bytes))
    }

    pub async fn send_msg(&self, title: String, content: String) -> anyhow::Result<()> {
        let url = format!(
            "https://open.feishu.cn/open-apis/bot/v2/hook/{}",
            self.access_token
        );
        let timestamp = chrono::Utc::now().timestamp();
        let signature = self.generate_signature(timestamp)?;

        let data = json!({
            "timestamp": timestamp,
            "sign": signature,
            "msg_type": "post",
            "content": {
                "post": {
                    "zh_cn": {
                        "title": title,
                        "content": [[{"tag": "text", "text": content}]]
                    }
                }
            }
        });

        let client = reqwest::Client::new();
        client.post(url).json(&data).send().await?;

        Ok(())
    }
}
