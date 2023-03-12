use std::sync::Arc;

use maid::{
    config::*,
    lam::ILam,
};
use maid_openai::chat::ChatGpt;
use tokio::runtime::Builder;

async fn proceed(
    config: Arc<Config>,
    lam: impl ILam + Send + Sync,
) -> anyhow::Result<()> {
    let lam = Arc::new(lam);

    todo!()
}

async fn async_main(config: Config) -> anyhow::Result<()> {
    let config = Arc::new(config);

    // Yeah, monomorphization...
    match &config.ai.lam_backend {
        LamBackend::OpenAi { model, token } => {
            let gpt = ChatGpt::new(token.clone(), model.clone());
            proceed(config, gpt).await
        }
    }
}

fn main() -> anyhow::Result<()> {
    let config = Config::try_load("assets/config.toml")
        .expect("Failed to load config from the file");
    let rt = match config.rt.threads.map(|i| i.get()) {
        Some(1 | 0) | None => Builder::new_current_thread(),
        Some(n) => {
            let mut b = Builder::new_multi_thread();
            b.worker_threads(n);
            b
        }
    }
    .enable_all()
    .build()
    .expect("Failed to build tokio runtime");

    rt.block_on(async_main(config))
}
