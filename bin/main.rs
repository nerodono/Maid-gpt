use maid::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load("assets/config.toml");
    Ok(())
}
