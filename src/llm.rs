use serde_json::json;

use crate::model::LLMConfig;

pub struct OpenAI {
    config: LLMConfig,
}

impl OpenAI {
    pub fn new(config: LLMConfig) -> Self {
        Self { config }
    }

    pub fn create_prompt(&self, text: &str) -> serde_json::Value {
        json!([
            {
                "role": "system",
                "content": self.config.system,
            },
            {
                "role": "user",
                "content": self.config.user.replace("{TEXT}", text),
            },
        ])
    }

    pub async fn create_completion(&self, input: &str) -> anyhow::Result<String> {
        let url = format!("{}/chat/completions", self.config.base_url);
        let req_data = json!({
            "model": self.config.model,
            "messages": self.create_prompt(input)
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
