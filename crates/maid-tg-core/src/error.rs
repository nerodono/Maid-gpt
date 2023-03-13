use thiserror::Error;

pub type ApiResult<T> = Result<T, RequestError>;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Failed to make request: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Failed to parse JSON: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("API Error (code = {error_code}): {description}")]
    Api {
        error_code: usize,
        description: String,
    },
}
