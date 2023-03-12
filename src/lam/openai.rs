use std::future::Future;

use maid_openai::{
    chat::ChatGpt,
    error::CompletionResult,
    schemas::Answer,
};

use super::interface::ILam;

impl ILam for ChatGpt {
    type Future<'a> =
        impl Future<Output = CompletionResult<Answer>> + 'a;

    fn try_complete<'a>(
        &'a self,
        prompts: &'a [maid_openai::schemas::Prompt],
        temperature: f64,
    ) -> Self::Future<'a> {
        ChatGpt::complete(self, prompts, temperature)
    }
}
