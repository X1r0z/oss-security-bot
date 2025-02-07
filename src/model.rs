use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub mail: MailConfig,
    pub bot: BotConfig,
    pub llm: LLMConfig,
}

#[derive(Deserialize)]
pub struct MailConfig {
    pub interval: u64,
    pub filters: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct BotConfig {
    #[serde(rename = "type")]
    pub bot_type: BotType,
    pub access_token: String,
    pub secret_key: Option<String>,
}

#[derive(Deserialize)]
pub enum BotType {
    #[serde(rename = "dingtalk")]
    DingTalk,
    #[serde(rename = "lark")]
    Lark,
    #[serde(rename = "wechat")]
    WeChat,
}

#[derive(Deserialize)]
pub struct LLMConfig {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub system: String,
    pub user: String,
}

#[derive(Debug)]
pub struct Message {
    pub title: String,
    pub content: String,
    pub reference: String,
}
