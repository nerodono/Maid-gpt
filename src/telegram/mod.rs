use std::sync::Arc;

use maid_openai::schemas::{
    Answer,
    Prompt,
};
use maid_telegram::{
    schemas::Update,
    telegram::Telegram,
};
use owo_colors::OwoColorize;

use crate::{
    config::Config,
    lam::ILam,
    template_data::{
        BotInformation,
        UserInformation,
    },
    templates::TemplateData,
};

pub async fn run_telegram_bot<Lam: ILam + Send + Sync>(
    config: Arc<Config>,
    lam: Arc<Lam>,
) -> anyhow::Result<()> {
    let telegram = Telegram::new(&config.bot.telegram.token);
    let me = telegram.get_me().await?;
    tracing::info!(
        "Started telegram bot ({name} {ref})",
        ref = format_args!("@{}", me.username.as_ref().unwrap()).green(),
        name = me.full_name().red(),
    );

    let bot_information = BotInformation {
        username: me.username.as_deref().unwrap(),
        id: me.id,
        self_config: &config.ai.character,
        master_config: &config.ai.master,
    };
    let mut offset = None;
    loop {
        let updates = telegram.get_updates(offset, 30).await?;
        if let Some(last) = updates.last() {
            offset = Some(last.update_id + 1);
        }

        for Update { message, .. } in updates {
            let Some(user) = message.from else { continue };
            let mut prefix_required = user.id != message.chat.id;
            let mut assistant_reply = None::<String>;
            if let Some(replied) = message.reply {
                if let Some(from) = replied.from {
                    if from.id == me.id {
                        prefix_required = false;
                        assistant_reply = replied.text;
                    }
                }
            }

            let data = TemplateData {
                bot: bot_information,
                user: UserInformation {
                    full_name: &user.full_name(),
                    id: user.id,
                    username: user.username.as_deref(),
                },
                assistant_reply: assistant_reply.as_deref(),
            };

            let mut text = message.text.unwrap_or(String::new());

            if prefix_required {
                match config.bot.prefix.find(&text) {
                    Some(match_) => {
                        let end = match_.end();
                        let (_, right) = text.split_at(end);
                        text = right.trim().into();
                    }
                    None if user.id != message.chat.id => continue,
                    None => {}
                }
            }
            let mut prompts = data.into_prompts();
            let _typer = typer::Typer::start_typing(
                telegram.clone(),
                message.chat.id,
            );

            tracing::info!(
                "Generating answer for the prompt {:?}...",
                &text,
            );
            prompts.push(Prompt::user(text));
            let answer = lam
                .try_complete(&prompts, config.ai.temperature)
                .await?;
            tracing::info!("Done");
            let parse_mode = "Markdown";
            match answer {
                Answer::Error { error } => {
                    println!("Error = {}", error.message);
                    telegram
                        .send_message(
                            "Rate-limit",
                            message.chat.id,
                            Some(message.message_id),
                            parse_mode,
                        )
                        .await?;
                }

                Answer::Success { result } => {
                    let choice = &result.choices[0];
                    let text = &choice.message.content;

                    telegram
                        .send_message(
                            text,
                            message.chat.id,
                            Some(message.message_id),
                            parse_mode,
                        )
                        .await?;
                }
            }
        }
    }
}

mod typer;
