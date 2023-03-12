use reqwest::Error as RequestError;
use serde_json::error::Error;
use thiserror::Error;

pub type CompletionResult<T> = Result<T, CompletionError>;

#[derive(Debug, Error)]
pub enum CompletionError {
    #[error("Request error: {0}")]
    Reqwest(#[from] RequestError),

    #[error("Failed to deserialize JSON: {0}")]
    Json(#[from] Error),
}
