include!(concat!(env!("OUT_DIR"), "/schemas.rs"));

pub use codegenerated::*;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Error {
        description: String,
        #[serde(rename = "error_code")]
        code: usize,
    },
    Success {
        result: T,
    },
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON decode error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API Error (code = {code}): {desc}")]
    Api { code: usize, desc: String },
}

impl<T> Response<T> {
    pub fn into_api_result(self) -> ApiResult<T> {
        match self {
            Self::Error { description, code } => {
                ApiResult::Err(ApiError::Api {
                    code,
                    desc: description,
                })
            }

            Self::Success { result } => Ok(result),
        }
    }
}
