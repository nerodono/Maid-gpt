use std::num::NonZeroUsize;

use derive_more::Display;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Completion {
    pub answer: Prompt,
    pub usage: Option<CompletionUsage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompletionUsage {
    #[serde(rename = "prompt_tokens")]
    pub prompt: NonZeroUsize,

    #[serde(rename = "completion_tokens")]
    pub completion: NonZeroUsize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    #[display(fmt = "system")]
    System,

    #[display(fmt = "assistant")]
    Assistant,

    #[display(fmt = "user")]
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display(fmt = "{role}: {content}")]
pub struct Prompt {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Display)]
pub enum Backend {
    #[display(fmt = "LLaMa")]
    Llama,

    #[display(fmt = "Meta OPT")]
    Opt,

    #[display(fmt = "OpenAI's {model}")]
    OpenAi { model: String },

    #[display(fmt = "GPT-Neo")]
    GptNeo,
}

impl Prompt {
    pub const fn user(content: String) -> Self {
        Self {
            content,
            role: Role::User,
        }
    }

    pub const fn assistant(content: String) -> Self {
        Self {
            content,
            role: Role::Assistant,
        }
    }

    pub const fn system(content: String) -> Self {
        Self {
            content,
            role: Role::System,
        }
    }
}
