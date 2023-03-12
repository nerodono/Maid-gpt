use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StructField {
    SingleType(String),
    Detailed {
        #[serde(rename = "default")]
        default_value: String,

        #[serde(rename = "type")]
        type_: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum EnumField {
    SingleIdent(String),
    Detailed { field: String, rename: String },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SchemaItem {
    Struct {
        fields: HashMap<String, StructField>,
    },
    Enum {
        items: Vec<EnumField>,
    },
}

#[derive(Debug, Deserialize)]
pub struct SchemaModel {
    pub name: String,

    #[serde(flatten)]
    pub item: SchemaItem,
}

#[derive(Debug, Deserialize)]
pub struct Schema {
    pub models: Vec<SchemaModel>,
}

impl Schema {
    pub fn emit(&self) -> String {
        let mut items = String::new();
        let mut top_level = String::new();
        for model in &self.models {
            items.push_str(&model.emit(&mut top_level));
            items.push('\n');
        }

        format!("mod codegenerated {{ {top_level}\n\n{items} }}")
    }

    pub fn parse_schema(input: &str) -> Self {
        match serde_yaml::from_str(input) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{e}");
                panic!()
            }
        }
    }
}
