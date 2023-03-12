use std::sync::Arc;

use teloxide::prelude::*;

use crate::config::Config;

pub async fn run_telegram_bot(config: Arc<Config>) {
    let bot = Bot::new(&config.bot.telegram.token);
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let text = msg.text().unwrap_or("");
        Ok(())
    })
    .await;
}
