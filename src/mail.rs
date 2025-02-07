use chrono::{DateTime, Datelike, Local};
use tokio::sync::mpsc::Sender;
use tracing::info;

use crate::{
    llm::OpenAI,
    model::{MailConfig, Message},
    util,
};

pub struct MailList {
    config: MailConfig,
    open_ai: OpenAI,
    date: DateTime<Local>,
    tx: Sender<Message>,
    num: u32,
}

impl MailList {
    pub fn new(
        config: MailConfig,
        open_ai: OpenAI,
        date: DateTime<Local>,
        tx: Sender<Message>,
    ) -> Self {
        Self {
            config,
            open_ai,
            date,
            tx,
            num: 1,
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            let now = Local::now();
            self.update_date(now);

            info!("fetch mail list");
            self.fetch_mail_list().await?;

            tokio::time::sleep(std::time::Duration::from_secs(self.config.interval)).await;
        }
    }

    pub fn update_date(&mut self, now: DateTime<Local>) {
        let duration = now.signed_duration_since(&self.date).num_days();

        if duration >= 1 {
            self.date = now;
            self.num = 1;
        }
    }

    pub async fn fetch_mail_list(&mut self) -> anyhow::Result<()> {
        let url = format!(
            "https://www.openwall.com/lists/oss-security/{}/{}/{}/{}",
            self.date.year(),
            format!("{:02}", self.date.month()),
            format!("{:02}", self.date.day()),
            self.num
        );

        let client = reqwest::Client::new();
        let resp = client.get(&url).send().await?;
        let body = resp.text().await?;

        if body.contains("No such Message") {
            return Ok(());
        }

        let subject = util::capture_subject(&body);
        let text = util::capture_content(&body);

        if let (Some(subject), Some(text)) = (subject, text) {
            if let Some(filters) = &self.config.filters {
                if !filters.iter().any(|filter| subject.contains(filter)) {
                    info!("skip mail subject: {}", subject);
                    return Ok(());
                }
            }

            info!("fetch mail subject: {}", subject);
            info!("summarize mail content");

            let summary = self
                .open_ai
                .create_completion(&text)
                .await
                .unwrap_or("Failed to summarize".to_string());

            let message = Message {
                title: subject,
                content: summary,
                reference: url,
            };
            self.tx.send(message).await?;
            self.num += 1;
        }

        Ok(())
    }
}
