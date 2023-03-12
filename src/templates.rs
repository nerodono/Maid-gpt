use askama::Template;
use maid_openai::schemas::Prompt;

use crate::template_data::{
    BotInformation,
    UserInformation,
};

pub struct TemplateData<'a> {
    pub user: UserInformation<'a>,
    pub bot: BotInformation<'a>,
    pub assistant_reply: Option<&'a str>,
}

macro_rules! setup_template {
    ($path:literal, $name:ident) => {
        #[derive(Template)]
        #[template(path = $path)]
        pub struct $name<'a> {
            pub data: &'a TemplateData<'a>,
        }

        impl<'a> $name<'a> {
            pub fn to_assistant_reply(&self) -> Prompt {
                Prompt::assistant(self.render().unwrap())
            }

            pub fn to_prompt(&self) -> Prompt {
                Prompt::system(self.render().unwrap())
            }
        }
    };
}

setup_template!("pre-message.txt", PreMessageTemplate);
setup_template!("character.txt", CharacterTemplate);
setup_template!("abilities.txt", AbilitiesTemplate);
setup_template!("reply.txt", ReplyTemplate);

impl<'a> TemplateData<'a> {
    pub fn into_prompts(&self) -> Vec<Prompt> {
        vec![
            CharacterTemplate { data: self }.to_prompt(),
            CharacterTemplate { data: self }.to_prompt(),
            AbilitiesTemplate { data: self }.to_prompt(),
            ReplyTemplate { data: self }.to_prompt(),
            PreMessageTemplate { data: self }.to_prompt(),
        ]
    }
}
