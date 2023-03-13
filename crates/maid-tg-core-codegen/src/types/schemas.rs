use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    Custom(String),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IntegralEnumField {
    Ident(String),
    Bound { name: String, bind: String },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ModelItem {
    Struct {
        fields: HashMap<String, Field>,
        optional: HashMap<String, Field>,
    },

    IntegralEnum {
        int_variants: Vec<IntegralEnumField>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub name: String,
    pub url: Option<String>,

    #[serde(flatten)]
    pub item: ModelItem,
}

impl Field {
    pub fn to_ty(&self) -> String {
        match self {
            Self::Custom(c) => match c.as_str() {
                "bool?" => "bool".into(),
                "string" => "String".into(),
                "integer" => "i64".into(),
                c => c.to_owned(),
            },
        }
    }
}
