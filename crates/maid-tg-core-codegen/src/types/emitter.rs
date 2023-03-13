use std::{
    collections::HashMap,
    fmt::Display,
};

use codegen::{
    Scope,
    Struct,
    Variant,
};

use super::schemas::{
    IntegralEnumField,
    Model,
    ModelItem,
};
use crate::types::schemas::Field;

fn get_doc(name: &str, doc: &Option<String>) -> String {
    format!(
        "`{name}` model.\n{}",
        if let Some(ref s) = doc {
            format!(
                "- Corresponds to the following Telegram type: {s}"
            )
        } else {
            String::new()
        }
    )
}

fn emit_model(model: &Model, scope: &mut Scope) {
    let doc = get_doc(&model.name, &model.url);
    match &model.item {
        ModelItem::IntegralEnum { int_variants } => {
            {
                let enum_ = scope
                    .new_enum(&model.name)
                    .derive("Debug")
                    .derive("serde::Serialize")
                    .derive("serde::Deserialize")
                    .derive("Clone")
                    .derive("Copy")
                    .derive("PartialEq")
                    .derive("Eq")
                    .vis("pub");
                enum_
                    .r#macro("#[serde(rename_all = \"snake_case\")]")
                    .doc(&doc);
                for variant in int_variants {
                    match variant {
                        IntegralEnumField::Ident(i) => {
                            enum_.push_variant(Variant::new(i));
                        }

                        IntegralEnumField::Bound { name, bind } => {
                            let mut var = Variant::new(bind);
                            var.annotation(format!(
                                "#[serde(rename = \"{name}\")]"
                            ));

                            enum_.push_variant(var);
                        }
                    }
                }
            }

            let trait_ = scope
                .new_impl(&model.name)
                .impl_trait("std::fmt::Display");
            trait_
                .new_fn("fmt")
                .arg_ref_self()
                .arg("f", "&mut std::fmt::Formatter<'_>")
                .ret("std::fmt::Result")
                .line("f.write_fmt(format_args!(\"{self:?}\"))");
        }

        ModelItem::Struct { fields, optional } => {
            let struct_ = scope.new_struct(&model.name);
            struct_
                .derive("Debug")
                .derive("serde::Serialize")
                .derive("serde::Deserialize")
                .doc(&doc);

            fn emit_field(
                name: &str,
                field: &Field,
                rename: Option<String>,
                s: &mut Struct,
                opt: bool,
            ) {
                let mut ty = field.to_ty();
                if opt {
                    ty = format!("Option<{ty}>");
                }
                let mut f = codegen::Field::new(name, ty);
                f.vis("pub");

                #[allow(clippy::single_match)]
                match field {
                    Field::Custom(c) if c == "bool?" => {
                        f.annotation(
                            "#[serde(default = \"__default_false\")]",
                        );
                    }

                    _ => {}
                }

                if let Some(rename) = rename {
                    f.annotation(&format!(
                        "#[serde(rename = {rename:?})]"
                    ));
                }

                s.push_field(f);
            }

            fn emit_fields(
                fields: &HashMap<String, Field>,
                s: &mut Struct,
                opt: bool,
            ) {
                for (name, field) in fields {
                    let mut name = name.clone();
                    let rename = if name.strip_suffix('!').is_some() {
                        let mut cloned = name.clone();
                        name.pop().unwrap();
                        name.push('_');

                        cloned.pop();
                        Some(cloned)
                    } else {
                        None
                    };

                    emit_field(&name, field, rename, s, opt);
                }
            }

            struct_.vis("pub");

            emit_fields(fields, struct_, false);
            emit_fields(optional, struct_, true);
        }
    }
}

pub fn emit_models(models: &[Model]) -> String {
    let mut scope = Scope::new();

    scope
        .new_fn("__default_false")
        .ret("bool")
        .line("false");

    models
        .iter()
        .for_each(|m| emit_model(m, &mut scope));

    scope.to_string()
}
