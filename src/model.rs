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
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub access_token: String,
    pub secret_key: String,
}

#[derive(Deserialize)]
pub struct LLMConfig {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub system: String,
    pub user: String,
}

pub struct Message {
    pub title: String,
    pub content: String,
    pub reference: String,
}
