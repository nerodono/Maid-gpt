use serde::{
    Deserialize,
    Serialize,
};

use crate::error::{
    ApiResult,
    RequestError,
};

mod details {
    include!(concat!(env!("OUT_DIR"), "/schemas_generated.rs"));
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Fail {
        error_code: usize,
        description: String,
    },
    Ok {
        result: T,
    },
}

impl<T> From<ApiResponse<T>> for ApiResult<T> {
    fn from(value: ApiResponse<T>) -> Self {
        match value {
            ApiResponse::Fail {
                error_code,
                description,
            } => Err(RequestError::Api {
                error_code,
                description,
            }),

            ApiResponse::Ok { result } => Ok(result),
        }
    }
}

pub use details::*;
