use llam_engine::types::Prompt;
use regex::Regex;

use crate::{
    ai::templates::get_base_prompts,
    platform::Platform,
};

#[derive(Debug)]
pub struct PromptUser<'a> {
    pub full_name: &'a str,
    pub id: i64,
    pub username: Option<&'a str>,
}

#[derive(Debug)]
pub struct PromptMessage<'a> {
    pub from: PromptUser<'a>,
    pub reply: Option<Box<PromptMessage<'a>>>,
    pub text: &'a str,

    pub is_private: bool,
    pub platform: Platform,
}

pub fn get_prompts_for_message(
    prefix_regex: &Regex,
    message: &'_ PromptMessage<'_>,
) -> Option<Vec<Prompt>> {
    let prefix_required = !message.is_private;
    let mut base = get_base_prompts(message);
    let text = match prefix_regex.find(message.text) {
        Some(m) => String::from(message.text[m.end()..].trim()),
        None if prefix_required => return None,
        _ => message.text.to_owned(),
    };

    base.push(Prompt::user(text));

    Some(base)
}
