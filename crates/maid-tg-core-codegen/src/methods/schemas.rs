use std::fmt::Display;

use serde::Deserialize;

const fn false_() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct SerializeOptions {
    #[serde(rename = "as")]
    pub as_: String,

    #[serde(rename = "fn")]
    pub fn_: String,
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

    pub serialize: Option<SerializeOptions>,
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

fn option(s: impl Display) -> String {
    format!("Option<{s}>")
}

impl BuilderItem {
    pub fn to_field(&self) -> codegen::Field {
        let mut field =
            codegen::Field::new(&self.name, self.to_opt_ty());
        field.vis("pub");

        field
    }

    fn wrap_list(list: bool, l: &str) -> String {
        if list {
            format!("Vec<{l}>")
        } else {
            l.to_owned()
        }
    }

    pub fn to_ser_ty(&self) -> String {
        if let Some(ser) = &self.serialize {
            ser.as_.clone()
        } else {
            self.to_opt_ty()
        }
    }

    pub fn to_opt_ty(&self) -> String {
        if self.default.is_some() {
            Self::wrap_list(self.list, &self.type_)
        } else {
            option(Self::wrap_list(self.list, &self.type_))
        }
    }
}

impl ConstructorArg {
    pub fn to_ty(&self) -> String {
        if self.required {
            self.type_.clone()
        } else {
            option(&self.type_)
        }
    }

    pub fn to_field<T: Display>(
        &self,
        note: impl Into<Option<T>>,
    ) -> codegen::Field {
        let mut field = codegen::Field::new(&self.name, self.to_ty());
        if let Some(note) = note.into() {
            field.annotation(&format!("// {note}"));
        }
        field.vis("pub");
        field
    }
}
