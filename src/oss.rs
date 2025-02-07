use chrono::{DateTime, Datelike, Local};
use tokio::sync::mpsc::Sender;
use tracing::info;

use crate::{llm::Summarizer, util};

pub struct MailList {
    date: DateTime<Local>,
    num: u32,
    interval: u64,
    summarizer: Summarizer,
    tx: Sender<(String, String)>,
}

impl MailList {
    pub fn new(
        now: DateTime<Local>,
        interval: u64,
        summarizer: Summarizer,
        tx: Sender<(String, String)>,
    ) -> Self {
        Self {
            date: now,
            num: 1,
            interval,
            summarizer,
            tx,
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            let now = Local::now();
            self.update_date(now);

            info!("fetch mail list");
            self.fetch_mail_list().await?;

            tokio::time::sleep(std::time::Duration::from_secs(self.interval)).await;
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
            info!("mail subject: {}", subject);
            info!("summarize mail content");

            let summary = self
                .summarizer
                .summarize(&text)
                .await
                .unwrap_or("Failed to summarize".to_string());
            let text = format!("{}\n\n{}", summary, url);

            self.tx.send((subject, text)).await?;
            self.num += 1;
        }

        Ok(())
    }
}
