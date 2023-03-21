use std::{
    env,
    path::PathBuf,
    sync::Arc,
};

use llam_engine::{
    backends::openai::OpenAiBackend,
    chat::{
        Chat,
        DynBackend,
    },
};
use maid::{
    config::{
        Config,
        LamBackend,
    },
    society,
};
use owo_colors::OwoColorize;
use tokio::runtime::Builder;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[allow(clippy::needless_late_init)]
fn main() -> anyhow::Result<()> {
    let config = Arc::new(Config::load(get_config_path()));
    let backend: DynBackend;
    let chat: Chat<DynBackend>;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    match &config.ai.lam_backend {
        LamBackend::OpenAi { model, token } => {
            backend = Box::new(OpenAiBackend::new(
                token.to_owned(),
                model.to_owned(),
            ));
        }
    }

    chat = Chat::new(backend, config.ai.temperature);
    tracing::info!("Using LLaM backend: {}", chat.backend().cyan());

    let rt = match config.rt.threads.map(|k| k.get()) {
        Some(0 | 1) | None => Builder::new_current_thread(),
        Some(wt) => {
            let mut b = Builder::new_multi_thread();
            b.enable_all().worker_threads(wt);
            b
        }
    }
    .build()
    .expect("Failed to create tokio runtime");

    let telegram = society::telegram::run(Arc::clone(&config), chat);
    rt.block_on(telegram)
}

fn get_config_path() -> PathBuf {
    match env::var_os("MAID_CONFIG") {
        Some(p) => p.into(),
        None => PathBuf::from("assets/config.toml"),
    }
}
