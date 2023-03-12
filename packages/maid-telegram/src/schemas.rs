use serde::{
    Deserialize,
    Serialize,
};

use crate::error::{
    ApiResult,
    RequestError,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Success {
        result: T,
    },
    Error {
        error_code: usize,
        description: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub update_id: u64,
    pub message: Message,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub chat: Chat,
    pub text: Option<String>,
    pub from: Option<User>,

    #[serde(rename = "reply_to_message")]
    pub reply: Option<Box<Message>>,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    #[default]
    Private,
    Group,
    #[serde(rename = "supergroup")]
    SuperGroup,
    Channel,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,

    pub first_name: String,

    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,

    #[serde(rename = "type")]
    pub type_: ChatType,

    pub title: Option<String>,
}

impl User {
    pub fn full_name(&self) -> String {
        match &self.last_name {
            Some(last) => format!("{} {last}", self.first_name),
            None => self.first_name.clone(),
        }
    }
}

impl<T> Response<T> {
    pub fn into_result(self) -> ApiResult<T> {
        match self {
            Self::Error {
                error_code,
                description,
            } => ApiResult::Err(RequestError::Api {
                code: error_code,
                text: description,
            }),

            Self::Success { result } => ApiResult::Ok(result),
        }
    }
}
