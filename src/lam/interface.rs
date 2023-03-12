use std::future::Future;

use maid_openai::{
    error::CompletionResult,
    schemas::{
        Answer,
        Prompt,
    },
};

pub trait ILam {
    type Future<'a>: Future<Output = CompletionResult<Answer>> + 'a
    where
        Self: 'a;

    fn try_complete<'a>(
        &'a self,
        prompts: &'a [Prompt],
        temperature: f64,
    ) -> Self::Future<'a>;
}
