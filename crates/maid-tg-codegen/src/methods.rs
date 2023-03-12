use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Copy)]
#[serde(rename_all = "snake_case")]
pub enum BuilderPattern {
    Mutable,
    Moving,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ParameterDeclaration {
    OptionalType(String),
    Detailed {
        #[serde(rename = "type")]
        type_: String,
        required: bool,
    },
}

#[derive(Deserialize)]
pub struct Method {
    pub parameters: HashMap<String, ParameterDeclaration>,
    pub returns: String,
}

#[derive(Deserialize)]
pub struct MethodsSchema {
    pub pattern: BuilderPattern,
    pub methods: HashMap<String, Method>,
}

impl MethodsSchema {
    pub fn parse_schema(s: &str) -> Self {
        serde_yaml::from_str(s).unwrap()
    }
}

impl MethodsSchema {
    pub fn emit(&self) -> String {
        let mut s = String::new();
        for (name, method) in &self.methods {
            s.push_str(&method.emit(self.pattern, name));
        }

        format!(
            "mod codegenerated {{ use reqwest::Client; use \
             crate::schemas::*; {s} }}"
        )
    }
}

impl BuilderPattern {
    pub fn emit_type(self) -> &'static str {
        match self {
            Self::Moving => "Self",
            Self::Mutable => "&mut Self",
        }
    }

    pub fn emit_self(self) -> &'static str {
        match self {
            Self::Moving => "mut self",
            Self::Mutable => "&mut self",
        }
    }
}
