use crate::schemas::*;

const BS: char = '{';
const BE: char = '}';

pub fn translate_keyword(s: &str) -> String {
    match s {
        "type" => "type_".into(),
        _ => s.into(),
    }
}

impl EnumField {
    pub fn emit(&self) -> String {
        match self {
            Self::SingleIdent(i) => i.to_string(),
            Self::Detailed { field, rename } => {
                format!("#[serde(rename = {rename:?})] {field}")
            }
        }
    }
}

impl StructField {
    pub fn emit(&self, top: &mut String) -> (String, String) {
        match self {
            Self::SingleType(ty) => (String::new(), ty.to_string()),
            Self::Detailed {
                default_value,
                type_,
            } => {
                let fn_name = format!("__default_{}", top.len());
                top.push_str(&format!(
                    "fn {fn_name}() -> {type_} {BS} {default_value} \
                     {BE}\n"
                ));
                (
                    format!("#[serde(default = \"{fn_name}\")]"),
                    type_.clone(),
                )
            }
        }
    }
}

impl SchemaItem {
    pub fn emit(&self, top: &mut String) -> (&'static str, String) {
        let (ty, result) = match self {
            Self::Enum { items } => {
                let mut variants = String::new();
                for field in items {
                    variants.push_str(&field.emit());
                    variants.push_str(",\n");
                }

                ("enum", variants)
            }

            Self::Struct { fields } => {
                let mut string_fields = String::new();
                for (name, field) in fields {
                    let (attrs, ty) = field.emit(top);
                    string_fields.push_str(&format!(
                        "#[serde(rename = {name:?})] {attrs} pub \
                         {name}: {ty},\n",
                        name = translate_keyword(name)
                    ));
                }
                ("struct", string_fields)
            }
        };

        (ty, result)
    }
}

impl SchemaModel {
    #[rustfmt::skip]
    pub fn emit(&self, top: &mut String) -> String {
        let (item_name, item_body) = self.item.emit(top);
        let inner = format!(
            "#[derive(Debug, serde::Serialize, serde::Deserialize)] \
             #[serde(rename_all = \"snake_case\")]\
             pub {item_name} {name} {BS} {item_body} {BE}",
            name = self.name,
        );

        inner
    }
}
