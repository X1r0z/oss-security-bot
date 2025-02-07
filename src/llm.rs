use serde_json::json;

use crate::model::LLMConfig;

pub struct OpenAI {
    config: LLMConfig,
}

impl OpenAI {
    pub fn new(config: LLMConfig) -> Self {
        Self { config }
    }

    pub async fn create_completion(&self, text: &str) -> anyhow::Result<String> {
        let url = format!("{}/chat/completions", self.config.base_url);
        let req_data = json!({
            "model": self.config.model,
            "messages": [
                {
                    "role": "system",
                    "content": self.config.system,
                },
                {
                    "role": "user",
                    "content": self.config.user.replace("{TEXT}", text),
                },
            ]
        });

        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .bearer_auth(&self.config.api_key)
            .json(&req_data)
            .send()
            .await?;

        let resp_data = resp.json::<serde_json::Value>().await?;
        let completion = resp_data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("Failed to get completion");

        Ok(completion.to_string())
    }
}
