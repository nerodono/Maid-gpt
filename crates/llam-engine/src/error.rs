use thiserror::Error;

/// Common error type for all backends
#[derive(Debug, Error)]
pub enum CompletionError {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON Decode error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Completion error: {0}")]
    Plaintext(#[from] String),
}
