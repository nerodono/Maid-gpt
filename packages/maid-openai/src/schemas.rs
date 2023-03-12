use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug, Serialize, Clone, Copy, PartialEq, Eq, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prompt {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokensUsage {
    #[serde(rename = "prompt_tokens")]
    pub prompt: u32,

    #[serde(rename = "completion_tokens")]
    pub completion: u32,

    #[serde(rename = "total_tokens")]
    pub total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Answer {
    Error {
        error: ErrorAnswer,
    },
    Success {
        #[serde(flatten)]
        result: SuccessfulAnswer,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorAnswer {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessfulAnswer {
    pub usage: TokensUsage,
    pub choices: Vec<AnswerChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerChoice {
    pub message: AnswerMessage,
    pub finish_reason: Option<FinishReason>,
}

impl Prompt {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }
}
