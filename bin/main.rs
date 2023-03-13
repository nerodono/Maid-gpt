use maid::config::Config;
use maid_tg_core::schemas::Chat;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load("assets/config.toml");

    let a: Chat;

    Ok(())
}
