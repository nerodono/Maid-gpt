use std::num::NonZeroUsize;

pub use appconf::interface::ParserFunctionality;
use appconf::macros::decl;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "snake_case")]
pub enum LamBackend {
    #[serde(rename = "openai")]
    OpenAi { model: String, token: String },
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
pub struct AiConfig {
    pub lam_backend: LamBackend,
    pub temperature: f64,
}

#[decl]
pub struct BotConfig {
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
