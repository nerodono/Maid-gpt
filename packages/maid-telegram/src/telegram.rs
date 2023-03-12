use std::{
    fmt::Display,
    sync::Arc,
};

use reqwest::{
    Client,
    ClientBuilder,
    Url,
};
use serde::Serialize;

use crate::{
    error::ApiResult,
    schemas::{
        Response,
        Update,
        User,
    },
};

#[derive(Clone)]
pub struct Telegram {
    token: Arc<String>,
    client: Client,
}

impl Telegram {
    pub async fn send_chat_action(
        &self,
        action: &str,
        chat_id: i64,
    ) -> ApiResult<()> {
        #[derive(Serialize)]
        struct Params<'a> {
            chat_id: i64,
            action: &'a str,
        }
        self.client
            .get(self.method_path("sendChatAction"))
            .query(&Params { chat_id, action })
            .send()
            .await?;
        Ok(())
    }

    pub async fn send_message(
        &self,
        text: &str,
        chat_id: i64,
        reply_to: Option<i64>,
        parse_mode: &str,
    ) -> ApiResult<()> {
        #[derive(Serialize)]
        struct Params<'a> {
            chat_id: i64,
            reply_to_message_id: Option<i64>,
            text: &'a str,
            parse_mode: &'a str,
        }

        self.client
            .post(self.method_path("sendMessage"))
            .form(&Params {
                chat_id,
                text,
                reply_to_message_id: reply_to,
                parse_mode,
            })
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_updates(
        &self,
        offset: Option<u64>,
        timeout: usize,
    ) -> ApiResult<Vec<Update>> {
        #[derive(Serialize)]
        struct Params {
            timeout: usize,
            offset: Option<u64>,
            allowed_updates: &'static str,
        }

        self.client
            .get(self.method_path("getUpdates"))
            .query(&Params {
                timeout,
                offset,
                allowed_updates: "message",
            })
            .send()
            .await?
            .json::<Response<_>>()
            .await?
            .into_result()
    }

    pub async fn get_me(&self) -> ApiResult<User> {
        self.client
            .get(self.method_path("getMe"))
            .send()
            .await?
            .json::<Response<_>>()
            .await?
            .into_result()
    }
}

impl Telegram {
    fn method_path(&self, method: impl Display) -> Url {
        Url::parse(&format!(
            "https://api.telegram.org/bot{}/{method}",
            self.token
        ))
        .unwrap()
    }

    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: Arc::new(token.into()),
            client: ClientBuilder::new()
                .build()
                .expect("Failed to create http client"),
        }
    }
}
