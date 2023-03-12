use std::sync::Arc;

use maid::{
    config::*,
    lam::ILam,
    telegram,
};
use maid_openai::chat::ChatGpt;
use tokio::runtime::Builder;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

async fn proceed<L: ILam + Send + Sync + 'static>(
    config: Arc<Config>,
    lam: L,
) -> anyhow::Result<()> {
    let lam = Arc::new(lam);
    let tg_bot = tokio::spawn(telegram::run_telegram_bot(
        Arc::clone(&config),
        Arc::clone(&lam),
    ));

    tg_bot.await.unwrap_or(Ok(()))
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

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set default subscriber");

    rt.block_on(async_main(config))
}
