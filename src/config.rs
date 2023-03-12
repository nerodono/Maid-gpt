use std::{
    fmt::Display,
    fs,
    num::{
        NonZeroU8,
        NonZeroUsize,
    },
    path::Path,
};

use regex::Regex;

macro_rules! enums {
    ($(
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variants:tt)*
        }
    )*) => {
        $(
            #[derive(Debug, serde::Serialize, serde::Deserialize)]
            #[serde(rename_all = "snake_case")]
            $(#[$meta])*
            $vis enum $name {
                $($variants)*
            }
        )*
    };

    (
        $(
            $(#[$meta:meta])*
            $vis:vis data $id:ident = $(
                $( #[$attr_meta:meta] )*
                $variant:ident
            )|*
        );*
        $(;)?
    ) => {
        $(
            #[derive(
                Debug, Clone, Copy, PartialEq,
                Eq, serde::Serialize, serde::Deserialize,
            )]
            #[serde(rename_all = "snake_case")]
            $(#[$meta])*
            $vis enum $id {
                $(
                    $(#[$attr_meta])*
                    $variant
                ),*
            }
        )*
    };
}

macro_rules! structs {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis struct $name:ident {
                $(
                    $(#[$field_meta:meta])*
                    $field:ident : $field_ty:ty
                ),*
                $(,)?
            }
        )*
    ) => {
        $(
            #[derive(Debug, serde::Serialize, serde::Deserialize)]
            $(#[$meta])*
            $vis struct $name {
                $(
                    $(#[$field_meta])*
                    pub $field : $field_ty
                ),*
            }
        )*
    };
}

enums! {
    pub data Sex = Male | Female;
}

enums! {
    pub enum TelegramPullMethod {
        #[serde(rename = "longpoll")]
        LongPolling(LongPollingSettings),

        #[serde(rename = "webhook")]
        WebHook(WebhookSettings)
    }

    pub enum LamBackend {
        #[serde(rename = "openai")]
        OpenAi {
            model: String,
            token: String,
        }
    }

    pub enum MasterIdentity {
        Username(String),
        UserId(i64),
    }
}

structs! {
    // Helpers

    pub struct CharacterItem {
        name: String,
        age: Option<NonZeroUsize>,
        sex: Sex,
    }

    // Components

    pub struct LongPollingSettings {
        wait_secs: u8,
        limit: NonZeroU8,
    }

    pub struct WebhookSettings {
        endpoint: String,
        secret_token: String,
    }

    pub struct TelegramConfig {
        token: String,
        pull: TelegramPullMethod
    }

    pub struct VkontakteConfig {
        token: String,
    }

    pub struct MasterConfig {
        #[serde(flatten)]
        character: CharacterItem,

        alternative_name: String,
        identify_by: MasterIdentity,
    }

    // Root elements

    pub struct Runtime {
        threads: Option<NonZeroUsize>,
    }

    pub struct BotConfig {
        #[serde(with = "serde_regex")]
        prefix: Regex,

        telegram: TelegramConfig,
        vkontakte: VkontakteConfig,
    }

    pub struct AiConfig {
        language: String,
        lam_backend: LamBackend,

        character: CharacterItem,
        master: MasterConfig,

        temperature: f64,
    }

    //

    pub struct Config {
        rt: Runtime,
        ai: AiConfig,
        bot: BotConfig,
    }
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let content = fs::read_to_string(path)
            .expect("Failed to read config file");
        match toml::from_str(&content) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{e}");
                panic!()
            }
        }
    }
}

impl Display for Sex {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(match self {
            Self::Male => "male",
            Self::Female => "female",
        })
    }
}
