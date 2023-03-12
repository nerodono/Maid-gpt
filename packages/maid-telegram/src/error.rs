use thiserror::Error;

pub type ApiResult<T> = Result<T, RequestError>;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Failed to make request: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON Deserialize error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API Error ({code}): {text}")]
    Api { code: usize, text: String },
}
