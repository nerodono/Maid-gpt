use serde::Deserialize;

const fn false_() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct ConstructorArg {
    pub name: String,

    #[serde(rename = "type")]
    pub type_: String,

    #[serde(default = "false_")]
    pub required: bool,
}

#[derive(Debug, Deserialize)]
pub struct BuilderItem {
    pub name: String,

    #[serde(rename = "type")]
    pub type_: String,

    pub default: Option<String>,

    #[serde(default = "false_")]
    pub list: bool,

    #[serde(default = "false_")]
    pub required: bool,
}

#[derive(Debug, Deserialize)]
pub struct Method {
    pub name: String,
    pub url: String,
    pub description: String,

    pub constructor: Vec<ConstructorArg>,
    pub builder: Option<Vec<BuilderItem>>,

    pub returns: String,
}
