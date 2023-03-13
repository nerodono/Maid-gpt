use codegen::Scope;
use heck::{
    ToPascalCase,
    ToSnakeCase,
};

use super::schemas::*;

fn generate_args_from_constructor(args: &[ConstructorArg]) -> String {
    let mut ret = String::new();
    for arg in args {
        ret.push_str(&arg.name);
        ret.push_str(", ");
    }

    ret
}

fn generate_args_from_builder(args: &[BuilderItem]) -> String {
    let mut ret = String::new();

    for arg in args {
        ret.push_str(&format!(
            "{name}: {val}",
            name = arg.name,
            val = if arg.default.is_some() || !arg.required {
                format!("self.{}", arg.name)
            } else if arg.required {
                format!(
                    "self.{name}.expect(\"`{name}` was not set\")",
                    name = arg.name
                )
            } else {
                unreachable!()
            }
        ));
        ret.push_str(", ");
    }

    ret
}

fn generate_builder(
    scope: &mut Scope,
    items: &[BuilderItem],
    method: &Method,
) {
    let struct_name =
        format!("{}Request", method.name.to_pascal_case());
    {
        let struct_ = scope
            .new_struct(&struct_name)
            .generic("'client")
            .field("client", "&'client reqwest::Client")
            .field("url", "reqwest::Url");

        for arg in &method.constructor {
            let mut field = codegen::Field::new(
                &arg.name,
                if arg.required {
                    arg.type_.clone()
                } else {
                    format!("Option<{}>", arg.type_)
                },
            );

            field.vis("pub");

            struct_.push_field(field);
        }

        for item in items {
            if item.default.is_some() {
                let mut field =
                    codegen::Field::new(&item.name, &item.type_);
                field.vis("pub");
                struct_.push_field(field);
            } else {
                let mut field = codegen::Field::new(
                    &item.name,
                    format!("Option<{}>", item.type_),
                );
                field.vis("pub");
                struct_.push_field(field);
            }
        }
    }

    let impl_ = scope.new_impl(&struct_name);
}

fn generate_builderless(scope: &mut Scope, method: &Method) {
    let fn_ = scope
        .new_fn(&method.name.to_snake_case())
        .ret(format!("ApiResult<{}>", method.returns))
        .arg_ref_self();

    fn_.line("#[derive(serde::Serialize)]");
    fn_.line("struct Params {");

    for arg in &method.constructor {
        let ty = if arg.required {
            arg.type_.clone()
        } else {
            format!("Option<{}>", arg.type_)
        };

        fn_.arg(&arg.name, &ty);
        fn_.line(format!("    {}: {ty}, ", arg.name));
    }

    fn_.line("}");

    #[rustfmt::skip]
    fn_.line(format!(
        "self.client()\n\
             .post(self.url_for({method_name}))\n\
             .form(&Params {{ {params_args} }})\n\
             .send()\n\
             .await?\n\
             .json::<ApiResponse<_>>()\n\
             .await?
             .into()",
        method_name = method.name,
        params_args = generate_args_from_constructor(&method.constructor)
    ));
}

fn emit_method(
    structures: &mut Scope,
    fns: &mut Scope,
    method: &Method,
) {
    match &method.builder {
        Some(items) => generate_builder(structures, items, method),
        None => generate_builderless(fns, method),
    }
}

pub fn emit_methods(methods: &[Method]) -> (String, String) {
    let mut structures = Scope::new();
    let mut fns = Scope::new();
    methods
        .iter()
        .for_each(|m| emit_method(&mut structures, &mut fns, m));

    (structures.to_string(), fns.to_string())
}
