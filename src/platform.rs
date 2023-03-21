use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Platform {
    #[display(fmt = "telegram")]
    Telegram,

    #[display(fmt = "vkontakte")]
    Vkontakte,
}
