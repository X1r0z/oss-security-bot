use std::{fs, path::Path};

use chrono::Local;
use config::Config;
use serde::Deserialize;
use tokio::sync::mpsc;

pub mod bot;
pub mod llm;
pub mod oss;
pub mod util;

const CONFIG_NAME: &str = "Config.toml";
const CONFIG_TEMPLATE: &str = include_str!("../Default.toml");

#[derive(Deserialize)]
struct AppConfig {
    oss: OssConfig,
    bot: BotConfig,
    llm: LLMConfig,
}

#[derive(Deserialize)]
struct OssConfig {
    interval: u64,
}

#[derive(Deserialize)]
struct BotConfig {
    access_token: String,
    secret_key: String,
}

#[derive(Deserialize)]
struct LLMConfig {
    base_url: String,
    api_key: String,
    model: String,
    system: String,
    user: String,
}

fn load_config() -> anyhow::Result<AppConfig> {
    if !Path::new(CONFIG_NAME).exists() {
        fs::write(CONFIG_NAME, CONFIG_TEMPLATE)?;
        return Err(anyhow::anyhow!(
            "Config file not found, created a default one"
        ));
    }

    let config = Config::builder()
        .add_source(config::File::with_name(CONFIG_NAME))
        .build()?;

    Ok(config.try_deserialize::<AppConfig>()?)
}

pub async fn run() -> anyhow::Result<()> {
    let config = load_config()?;

    let summarizer = llm::Summarizer::new(
        config.llm.base_url,
        config.llm.api_key,
        config.llm.model,
        config.llm.system,
        config.llm.user,
    );

    let (tx, rx) = mpsc::channel(32);
    let mut bot = bot::LarkBot::new(config.bot.access_token, config.bot.secret_key, rx);
    let mut mail_list = oss::MailList::new(Local::now(), config.oss.interval, summarizer, tx);

    let (r1, r2) = tokio::join!(bot.run(), mail_list.run());
    (r1?, r2?);

    Ok(())
}
