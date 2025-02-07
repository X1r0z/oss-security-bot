use std::{fs, path::Path};

use chrono::Local;
use config::Config;
use model::AppConfig;
use tokio::sync::mpsc;

pub mod bot;
pub mod llm;
pub mod mail;
pub mod model;
pub mod util;

const CONFIG_NAME: &str = "Config.toml";
const CONFIG_TEMPLATE: &str = include_str!("../templates/Default.toml");
const CARD_TEMPLATE: &str = include_str!("../templates/Card.json");

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
    let (tx, rx) = mpsc::channel(32);

    let open_ai = llm::OpenAI::new(config.llm);
    let mut lark_bot = bot::LarkBot::new(config.bot, rx);
    let mut mail_list = mail::MailList::new(config.mail, open_ai, Local::now(), tx);

    let (r1, r2) = tokio::join!(lark_bot.run(), mail_list.run());
    (r1?, r2?);

    Ok(())
}
