use std::sync::Arc;

use reqwest::{
    Client,
    ClientBuilder,
};
use serde::Serialize;

use crate::{
    error::CompletionResult,
    schemas::{
        Answer,
        Prompt,
    },
};

#[derive(Debug)]
struct InnerData {
    pub token: String,
    pub model: String,
}

#[derive(Debug, Clone)]
pub struct ChatGpt {
    client: Client,
    data: Arc<InnerData>,
}

impl ChatGpt {
    pub async fn complete(
        &self,
        prompts: &[Prompt],
        temperature: f64,
    ) -> CompletionResult<Answer> {
        #[derive(Serialize)]
        struct Input<'a> {
            model: &'a str,
            messages: &'a [Prompt],
            temperature: f64,
        }

        self.client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.data.token)
            .json(&Input {
                model: &self.data.model,
                messages: prompts,
                temperature,
            })
            .send()
            .await?
            .json::<Answer>()
            .await
            .map_err(|e| e.into())
    }
}

impl ChatGpt {
    pub fn new(token: String, model: String) -> Self {
        Self {
            data: Arc::new(InnerData { token, model }),
            client: ClientBuilder::new()
                .build()
                .expect("Failed to create http client"),
        }
    }
}
