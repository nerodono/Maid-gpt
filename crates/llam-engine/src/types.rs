use derive_more::Display;

#[derive(Debug, Clone, Display)]
pub enum Backend {
    #[display(fmt = "LLaMa")]
    Llama,

    #[display(fmt = "Meta OPT")]
    Opt,

    #[display(fmt = "OpenAI's {model}")]
    OpenAi { model: String },

    #[display(fmt = "GPT-Neo")]
    GptNeo,
}
