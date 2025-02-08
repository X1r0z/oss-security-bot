use async_trait::async_trait;
use serde_json::json;

use crate::model::{BotConfig, Message};

use super::Push;

pub struct WeChatBot {
    config: BotConfig,
}

impl WeChatBot {
    pub fn new(config: BotConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Push for WeChatBot {
    fn name(&self) -> String {
        "wechat".to_string()
    }

    async fn send_message(&self, message: Message) -> anyhow::Result<()> {
        let url = format!(
            "https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key={}",
            self.config.access_token
        );
        let content = format!(
            "# {title}\n\t\n{content}\n\t\n[{reference}]({reference})",
            title = message.title,
            content = message.content,
            reference = message.reference
        );

        let data = json!({
            "msgtype": "markdown",
            "markdown": {
                "content": content,
            }
        });

        let client = reqwest::Client::new();
        client.post(url).json(&data).send().await?;

        Ok(())
    }
}
