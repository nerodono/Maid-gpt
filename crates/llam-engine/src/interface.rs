use async_trait::async_trait;

use crate::types::Backend;

#[async_trait]
pub trait ILam {
    fn backend(&self) -> Backend;
}
