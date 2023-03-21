use askama::Template;
use llam_engine::types::{
    Prompt,
    Role,
};

use super::message::PromptMessage;

macro_rules! consume {
    ($any:tt, $ret:tt) => {
        $ret
    };
}

macro_rules! count_idents {
    ($($ident:ident)*) => {
        0 $(
            + consume!($ident, 1)
        )*
    };
}

macro_rules! templates {
    ($(
        $name:ident($path:literal)
    )*) => {
        const TEMPLATES: usize = count_idents!($($name)*);
        $(
            #[derive(Template)]
            #[template(path = $path)]
            pub struct $name<'a> {
                pub message: &'a PromptMessage<'a>
            }

            impl<'a> $name<'a> {
                pub fn to_prompt(&self) -> Prompt {
                    Prompt {
                        role: Role::System,
                        content: self.render().expect(concat!(
                            "Failed to render `", stringify!($name), "` prompt"
                        ))
                    }
                }
            }
        )*

        pub fn get_base_prompts(message: &'_ PromptMessage<'_>) -> Vec<Prompt> {
            let mut prompts = Vec::with_capacity(TEMPLATES);
            $(
                let prompt = $name { message }.to_prompt();
                if !prompt.content.is_empty() {
                    prompts.push(prompt);
                }
            )*

            prompts
        }
    };
}

templates! {
    CharacterTemplate("character.txt")
    ReplyTemplate("reply.txt")
    AbilitiesTemplate("abilities.txt")
    PreMessageTemplate("pre-message.txt")
}
