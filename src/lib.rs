use std::{fs, path::Path};

use bot::{dingtalk::DingTalkBot, lark::LarkBot, wechat::WeChatBot, Push};
use config::Config;
use model::{AppConfig, BotType};
use tokio::{join, sync::mpsc};
use webhook::Webhook;

pub mod bot;
pub mod db;
pub mod llm;
pub mod mail;
pub mod model;
pub mod util;
pub mod webhook;

const CONFIG_NAME: &str = "Config.toml";
const CONFIG_TEMPLATE: &str = include_str!("../templates/Default.toml");

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

    let llm = llm::OpenAI::new(config.llm);
    let bot: Box<dyn Push> = match config.bot.bot_type {
        BotType::DingTalk => Box::new(DingTalkBot::new(config.bot)),
        BotType::Lark => Box::new(LarkBot::new(config.bot)),
        BotType::WeChat => Box::new(WeChatBot::new(config.bot)),
    };

    let mut mail_list = mail::MailList::new(config.mail, llm, tx);
    let mut webhook = Webhook::new(config.webhook, bot, rx);

    join!(mail_list.run(), webhook.run());
    Ok(())
}
