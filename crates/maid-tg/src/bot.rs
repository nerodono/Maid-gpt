use std::{
    fmt::Display,
    sync::Arc,
};

use maid_tg_core::methods::{
    GetMeBuilder,
    GetUpdatesBuilder,
};
use reqwest::{
    Client,
    ClientBuilder,
    Url,
};

macro_rules! methods {
    (
        $(
            $name:ident($method:expr) -> $ret_ty:ident
        );*
        $(;)?
    ) => {
        $(
            pub fn $name(&self) -> $ret_ty<'_> {
                $ret_ty::new(&self.client, self.url_for($method))
            }
        )*
    };
}

#[derive(Clone)]
pub struct Bot {
    token: Arc<String>,
    client: Client,
}

impl Bot {
    methods! {
        get_me("getMe") -> GetMeBuilder;
        get_updates("getUpdates") -> GetUpdatesBuilder;
    }
}

impl Bot {
    fn url_for(&self, method: impl Display) -> Url {
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
                .expect("Failed to build client"),
        }
    }
}
