use std::{
    fmt::Display,
    num::NonZeroUsize,
};

pub use appconf::interface::ParserFunctionality;
use appconf::macros::decl;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sex {
    Male,
    Female,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LamBackend {
    #[serde(rename = "openai")]
    OpenAi { model: String, token: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MasterIdentification {
    Username(String),
    UserId(i64),
}

#[decl]
pub struct OpenaiConfig {
    pub model: String,
    pub token: String,
}

#[decl]
pub struct TelegramBotConfig {
    pub token: String,
}

#[decl]
pub struct Character {
    pub name: String,
    pub age: Option<NonZeroUsize>,
    pub sex: Sex,
}

#[decl]
pub struct AiCharacterConfig {
    #[serde(flatten)]
    pub common: Character,
}

#[decl]
pub struct AiMasterConfig {
    #[serde(flatten)]
    pub common: Character,
    pub identity: MasterIdentification,
}

#[decl]
pub struct AiConfig {
    pub lam_backend: LamBackend,
    pub temperature: f64,

    pub character: AiCharacterConfig,
    pub master: AiMasterConfig,
}

#[decl]
pub struct BotConfig {
    #[serde(with = "serde_regex")]
    pub prefix: regex::Regex,
    pub telegram: TelegramBotConfig,
}

#[decl]
pub struct Runtime {
    pub threads: Option<NonZeroUsize>,
}

#[decl(loader = "toml")]
pub struct Config {
    pub rt: Runtime,
    pub ai: AiConfig,
    pub bot: BotConfig,
}

impl Display for Sex {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
