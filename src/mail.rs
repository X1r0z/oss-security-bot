use rusqlite::Connection;
use tokio::sync::mpsc::Sender;
use tracing::{error, info};

use crate::{
    db,
    llm::OpenAI,
    model::{MailConfig, Message},
    util,
};

pub struct MailList {
    config: MailConfig,
    conn: Connection,
    llm: OpenAI,
    tx: Sender<Message>,
}

impl MailList {
    pub fn new(config: MailConfig, llm: OpenAI, tx: Sender<Message>) -> Self {
        let conn = db::create_connection(&config.db_name).expect("Failed to create connection");

        Self {
            config,
            conn,
            llm,
            tx,
        }
    }

    pub async fn run(&mut self) {
        loop {
            info!("fetch mail list");

            if let Err(e) = self.fetch_mail_list().await {
                error!("failed to fetch mail list: {:?}", e);
            }

            tokio::time::sleep(std::time::Duration::from_secs(self.config.interval)).await;
        }
    }

    pub async fn fetch_mail_list(&mut self) -> anyhow::Result<()> {
        let url = "https://www.openwall.com/lists/oss-security/";

        let client = reqwest::Client::new();
        let resp = client.get(url).send().await?;
        let body = resp.text().await?;

        let mails = util::capture_mails(&body)
            .unwrap_or_default()
            .into_iter()
            .take(self.config.recent);

        for mail_date in mails {
            let exists = db::select_mail(&self.conn, &mail_date)?;

            if !exists {
                db::insert_mail(&self.conn, &mail_date)?;
                self.fetch_mail(mail_date).await?;
            }
        }

        Ok(())
    }

    pub async fn fetch_mail(&self, mail_date: String) -> anyhow::Result<()> {
        let url = format!("https://www.openwall.com/lists/oss-security/{}", mail_date);

        let client = reqwest::Client::new();
        let resp = client.get(&url).send().await?;
        let body = resp.text().await?;

        let subject = util::capture_subject(&body).unwrap();
        let text = util::capture_text(&body).unwrap();

        if let Some(filters) = &self.config.filters {
            if !filters.iter().any(|filter| subject.contains(filter)) {
                info!("skip mail subject: {}", subject);
                return Ok(());
            }
        }

        info!("fetch mail subject: {}", subject);

        let summary = self
            .llm
            .create_completion(&text)
            .await
            .unwrap_or("Failed to summarize".to_string());

        let message = Message {
            title: subject,
            content: summary,
            reference: url,
        };
        self.tx.send(message).await?;

        Ok(())
    }
}
