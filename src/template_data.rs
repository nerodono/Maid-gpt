use crate::config::{
    AiCharacterConfig,
    AiMasterConfig,
    MasterIdentification,
    Sex,
};

#[derive(Clone, Copy)]
pub struct BotInformation<'a> {
    pub id: i64,
    pub username: &'a str,

    pub self_config: &'a AiCharacterConfig,
    pub master_config: &'a AiMasterConfig,
}

pub struct UserInformation<'a> {
    pub full_name: &'a str,
    pub username: Option<&'a str>,

    pub id: i64,
}

impl BotInformation<'_> {
    pub fn sex(&self) -> Sex {
        self.self_config.common.sex
    }

    pub fn name(&self) -> &str {
        &self.self_config.common.name
    }
}

impl<'a> BotInformation<'a> {
    pub fn is_master(&self, user: &UserInformation<'_>) -> bool {
        match self.master_config.identity {
            MasterIdentification::UserId(id) => user.id == id,
            MasterIdentification::Username(ref username) => user
                .username
                .map(|uname| username == uname)
                .unwrap_or(false),
        }
    }
}
