use std::time::Duration;

use maid_telegram::telegram::Telegram;
use tokio::sync::oneshot;

struct TypeEndToken;

pub struct Typer {
    tx: Option<oneshot::Sender<TypeEndToken>>,
}

impl Typer {
    pub fn start_typing(telegram: Telegram, chat_id: i64) -> Self {
        let (tx, mut rx) = oneshot::channel();
        tokio::spawn(async move {
            loop {
                if let Ok(_) = rx.try_recv() {
                    break;
                }

                if telegram
                    .send_chat_action("typing", chat_id)
                    .await
                    .is_err()
                {
                    break;
                }
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        });
        Self { tx: Some(tx) }
    }
}

impl Drop for Typer {
    fn drop(&mut self) {
        let tx = std::mem::take(&mut self.tx).unwrap();
        tx.send(TypeEndToken).unwrap_or_default();
    }
}
