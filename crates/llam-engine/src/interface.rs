use async_trait::async_trait;

use crate::{
    error::CompletionError,
    types::{
        Backend,
        Completion,
        Prompt,
    },
};

pub type CompletionResult<T> = Result<T, CompletionError>;

#[async_trait]
pub trait ILam {
    async fn try_complete(
        &self,
        prompts: &[Prompt],
        temperature: f64,
    ) -> CompletionResult<Completion>;

    fn backend(&self) -> Backend;

    fn cheap_clone(&self) -> Box<dyn ILam + Send + Sync>;
}
