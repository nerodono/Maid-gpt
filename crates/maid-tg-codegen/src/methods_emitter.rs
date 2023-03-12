use heck::ToPascalCase;

use crate::methods::*;

impl Method {
    pub fn emit_parameters(&self) -> String {
        let mut params = String::new();

        for (name, decl) in &self.parameters {
            params.push_str(&format!(
                "{name}: Option<{ty}>,\n",
                ty = match decl {
                    ParameterDeclaration::Detailed {
                        type_, ..
                    } => type_,
                    ParameterDeclaration::OptionalType(o) => o,
                }
            ))
        }

        params
    }

    #[rustfmt::skip]
    pub fn emit(&self, pattern: BuilderPattern, name: &str) -> String {
        let pascal_name = name.to_pascal_case();
        let struct_definition = format!(
            "#[derive(Debug)] pub struct {method_name}Builder<'client> {{\n\
                {params_}\
                url: reqwest::Url,\
                client: &'client Client,\n\
            }}", 
            method_name = pascal_name,
            params_ = self.emit_parameters(),
        );

        let none_fields: String = self.parameters
                .keys()
                .map(|name| format!(
                    "{name}: None, ",
                ))
                .collect();
        let typed_fields: String = self.parameters
                .iter()
                .map(|(name, decl)| {
                    match decl {
                        ParameterDeclaration::Detailed {
                            type_, required: false
                        } | ParameterDeclaration::OptionalType(type_) => {
                            format!("{name}: Option<{type_}>, ")
                        }

                        ParameterDeclaration::Detailed { type_, .. } => {
                            format!("{name}: {type_}, ")
                        }
                    }
                }).collect();
        let unwrapped_fields: String = self.parameters
            .iter()
            .map(|(name, decl)| {
                match decl {
                    ParameterDeclaration::Detailed { required: true, .. } => {
                        format!("{name}: self.{name}.expect(\"`{name}` field was not set\"), ")
                    }
                    
                    ParameterDeclaration::OptionalType(
                        ..
                    ) | ParameterDeclaration::Detailed { .. } => {
                        format!("{name}: self.{name}, ")
                    }
                }
            })
            .collect();
        let methods: String = self.parameters
            .iter()
            .map(|(name, decl)| {
                let (
                    ParameterDeclaration::Detailed { type_, required: _ }
                    | ParameterDeclaration::OptionalType(type_)) = decl;
                format!(
                    "pub fn {name}({pat}, {name}: impl Into<Option<{type_}>>) -> {pat_ret} {{\
                        self.{name} = {name}.into();\
                        self\
                    }}",
                    pat = pattern.emit_self(),
                    pat_ret = pattern.emit_type(),
                )
            })
            .collect();
        let impl_definition = format!(
            "impl<'client> {pascal_name}Builder<'client> {{\
                {methods}\
                pub async fn send(self) -> ApiResult<{return_type}> {{\
                    #[derive(serde::Serialize)]\
                    struct Params {{\
                        {typed_fields}\
                    }}\
                    self.client.post(self.url)\
                        .form(&Params {{ {unwrapped_fields} }}) \
                        .send()\
                        .await?\
                        .json::<Response<_>>()\
                        .await?\
                        .into_api_result()\
                }}\
                pub fn new(client: &'client Client, url: reqwest::Url) -> Self {{ \
                    Self {{ client, url, {none_fields} }}
                }}\
            }}",
            return_type = self.returns
        );

        format!("{struct_definition}\n{impl_definition}")
    }
}
