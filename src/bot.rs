use async_trait::async_trait;

use crate::model::Message;

pub mod dingtalk;
pub mod lark;
pub mod wechat;

#[async_trait]
pub trait Push {
    async fn send_message(&self, message: Message) -> anyhow::Result<()>;
}
