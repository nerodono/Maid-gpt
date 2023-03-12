use maid::config::Config;
use maid_tg::{
    bot::Bot,
    core::schemas::{
        Response,
        User,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load("assets/config.toml");
    let bot = Bot::new(&config.bot.telegram.token);

    let result = bot.get_me().send().await?;
    dbg!(result);

    Ok(())
}
