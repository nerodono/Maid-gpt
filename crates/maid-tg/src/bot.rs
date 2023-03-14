use std::sync::Arc;

use reqwest::{
    Client,
    ClientBuilder,
    Url,
};

use crate::core::{
    error::*,
    request::*,
    schemas::*,
};

struct BotInner {
    token: String,
}

#[derive(Clone)]
pub struct Bot {
    data: Arc<BotInner>,
    client: Client,
}

include!(concat!(env!("OUT_DIR"), "/methods_generated.rs"));

impl Bot {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            data: Arc::new(BotInner {
                token: token.into(),
            }),
            client: ClientBuilder::new()
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    fn client(&self) -> &Client {
        &self.client
    }

    fn url_for(&self, for_: &str) -> Url {
        format!(
            "https://api.telegram.org/bot{}/{for_}",
            self.data.token
        )
        .parse()
        .unwrap()
    }
}
