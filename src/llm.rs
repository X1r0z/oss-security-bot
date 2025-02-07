use serde_json::json;

pub struct Summarizer {
    base_url: String,
    api_key: String,
    model: String,
    system: String,
    user: String,
}

impl Summarizer {
    pub fn new(
        base_url: String,
        api_key: String,
        model: String,
        system: String,
        user: String,
    ) -> Self {
        Self {
            base_url,
            api_key,
            model,
            system,
            user,
        }
    }

    pub async fn summarize(&self, text: &str) -> anyhow::Result<String> {
        let url = format!("{}/chat/completions", self.base_url);

        let req_data = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": self.system,
                },
                {
                    "role": "user",
                    "content": self.user.replace("{TEXT}", text)
                },
            ]
        });

        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .bearer_auth(&self.api_key)
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
