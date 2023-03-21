use std::ops::Deref;

use crate::{
    interface::{
        CompletionResult,
        ILam,
    },
    types::{
        Completion,
        Prompt,
    },
};

pub type DynBackend = Box<dyn ILam + Send + Sync>;

pub struct Chat<Backend> {
    pub temperature: f64,

    backend: Backend,
}

impl<Backend> Chat<Backend>
where
    Backend: Deref<Target = dyn ILam + Send + Sync>,
{
    pub fn new(backend: Backend, temperature: f64) -> Self {
        Self {
            backend,
            temperature,
        }
    }

    pub async fn try_complete(
        &self,
        prompts: &[Prompt],
    ) -> CompletionResult<Completion> {
        self.backend
            .try_complete(prompts, self.temperature)
            .await
    }

    pub fn backend(&self) -> crate::types::Backend {
        self.backend.backend()
    }
}

impl Clone for Chat<DynBackend> {
    fn clone(&self) -> Self {
        Self {
            temperature: self.temperature,
            backend: self.backend.cheap_clone(),
        }
    }
}
