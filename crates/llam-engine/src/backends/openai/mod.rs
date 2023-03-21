use std::sync::Arc;

use async_trait::async_trait;
use reqwest::{
    Client,
    ClientBuilder,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    error::CompletionError,
    interface::{
        CompletionResult,
        ILam,
    },
    types::{
        Backend,
        Completion,
        CompletionUsage,
        Prompt,
    },
};

struct OpenAiData {
    token: String,
    model: String,
}

#[derive(Clone)]
pub struct OpenAiBackend {
    data: Arc<OpenAiData>,
    client: Client,
}

impl OpenAiBackend {
    pub fn new(token: String, model: String) -> Self {
        Self {
            data: Arc::new(OpenAiData { token, model }),
            client: ClientBuilder::new().build().unwrap(),
        }
    }
}

#[async_trait]
impl ILam for OpenAiBackend {
    async fn try_complete(
        &self,
        prompts: &[Prompt],
        temperature: f64,
    ) -> CompletionResult<Completion> {
        #[derive(Deserialize)]
        struct Choice {
            message: Prompt,
        }

        #[derive(Deserialize)]
        struct ErrorMessage {
            message: String,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Response {
            Succ(ResponseOk),
            Error { error: ErrorMessage },
        }

        #[derive(Deserialize)]
        struct ResponseOk {
            usage: CompletionUsage,
            choices: Vec<Choice>,
        }

        #[derive(Serialize)]
        struct Request<'a> {
            model: &'a str,
            messages: &'a [Prompt],
            temperature: f64,
        }

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.data.token)
            .json(&Request {
                model: &self.data.model,
                messages: prompts,
                temperature,
            })
            .send()
            .await?
            .json::<Response>()
            .await?;

        match response {
            Response::Succ(mut ok) => Ok(Completion {
                answer: ok.choices.pop().unwrap().message,
                usage: Some(ok.usage),
            }),

            Response::Error { error } => {
                Err(CompletionError::Plaintext(error.message))
            }
        }
    }

    fn backend(&self) -> Backend {
        Backend::OpenAi {
            model: self.data.model.clone(),
        }
    }

    fn cheap_clone(&self) -> Box<dyn ILam + Send + Sync> {
        Box::new(Self {
            data: Arc::clone(&self.data),
            client: self.client.clone(),
        })
    }
}

mod schemas;
