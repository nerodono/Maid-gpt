use maid::config::Config;
use maid_tg::bot::Bot;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load("assets/config.toml");
    let bot = Bot::new(&config.bot.telegram.token);
    let me = bot.get_me().await?;

    println!("{me:#?}");

    Ok(())
}
