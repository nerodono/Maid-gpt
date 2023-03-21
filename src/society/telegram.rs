use std::{
    sync::Arc,
    time::Duration,
};

use llam_engine::{
    chat::{
        Chat,
        DynBackend,
    },
    error::CompletionError,
};
use owo_colors::OwoColorize;
use teloxide::{
    dispatching::UpdateFilterExt,
    payloads::SendMessageSetters,
    prelude::{
        Bot,
        Dispatcher,
    },
    requests::Requester,
    types::{
        ChatAction,
        Message,
        Recipient,
        Update,
    },
};
use tokio::sync::oneshot::{
    self,
    error::TryRecvError,
};

use crate::{
    ai::message::{
        get_prompts_for_message,
        PromptMessage,
        PromptUser,
    },
    config::Config,
    platform::Platform,
    utils::DeferAction,
};

async fn telegram_handler(
    bot: Bot,
    message: Message,
    config: Arc<Config>,
    chat: Chat<DynBackend>,
) -> anyhow::Result<()> {
    async fn typer(
        bot: Bot,
        mut chan: oneshot::Receiver<()>,
        rec: impl Into<Recipient> + Clone,
    ) -> anyhow::Result<()> {
        loop {
            match chan.try_recv() {
                Ok(()) | Err(TryRecvError::Closed) => break,
                Err(_) => {}
            }
            bot.send_chat_action(rec.clone(), ChatAction::Typing)
                .await?;
            tokio::time::sleep(Duration::from_secs(4)).await;
        }

        Ok(())
    }

    let Some(text) = message.text() else {
        return Ok(())
    };
    let Some(from) = message.from() else {
        return Ok(());
    };

    let prompt_message = PromptMessage {
        from: PromptUser {
            full_name: &from.full_name(),
            id: from.id.0 as i64,
            username: from.username.as_deref(),
        },
        text,
        reply: None,
        is_private: message.chat.is_private(),
        platform: Platform::Telegram,
    };
    let Some(prompts) = get_prompts_for_message(&config.bot.prefix, &prompt_message) else {
        return Ok(())
    };

    let (wchan, rchan) = oneshot::channel();
    let _stop_defer = DeferAction::defer(move || {
        wchan.send(()).unwrap_or_default()
    });
    tokio::spawn(typer(bot.clone(), rchan, message.chat.id));
    tracing::info!(
        "Got prompt {prompt} to proceed",
        prompt = text.black().on_white()
    );
    let response = match chat.try_complete(&prompts).await {
        Ok(r) => r,
        Err(CompletionError::Plaintext(text)) => {
            tracing::error!("Completion error: {text}");
            bot.send_message(message.chat.id, "LLaM Error")
                .await?;
            return Ok(());
        }
        Err(e) => {
            eprintln!("{e}");
            return Err(e.into());
        }
    };

    bot.send_message(message.chat.id, response.answer.content)
        .reply_to_message_id(message.id)
        .await?;

    Ok(())
}

pub async fn run(
    config: Arc<Config>,
    chat: Chat<DynBackend>,
) -> anyhow::Result<()> {
    let ignore_update = |_upd| Box::pin(async {});
    let bot = Bot::new(&config.bot.telegram.token);

    Dispatcher::<_, anyhow::Error, _>::builder(
        bot,
        Update::filter_message().endpoint(telegram_handler),
    )
    .dependencies(dptree::deps![chat, config])
    .default_handler(ignore_update)
    .build()
    .dispatch()
    .await;

    Ok(())
}
