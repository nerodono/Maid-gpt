use std::fmt::Display;

use codegen::{
    Function,
    Impl,
    Scope,
};
use heck::{
    ToPascalCase,
    ToSnakeCase,
};

use super::schemas::*;

fn generate_args_from_constructor(args: &[ConstructorArg]) -> String {
    let mut ret = String::new();
    for arg in args {
        ret.push_str(&format!(
            "{name}: &self.{name}",
            name = arg.name
        ));
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
            val = if let Some(options) = &arg.serialize {
                format!("&{}(&self.{})", options.fn_, arg.name)
            } else if arg.default.is_some() || !arg.required {
                format!("&self.{}", arg.name)
            } else if arg.required {
                format!(
                    "&self.{name}.expect(\"`{name}` was not set\")",
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

fn builder_name(name: &str) -> String {
    format!("{}Request", name.to_pascal_case())
}

fn generate_builder(
    scope: &mut Scope,
    items: &[BuilderItem],
    method: &Method,
) {
    let struct_name = builder_name(&method.name);
    {
        let struct_ = scope
            .new_struct(&struct_name)
            .generic("'client")
            .field("client", "&'client reqwest::Client")
            .field("url", "reqwest::Url")
            .vis("pub")
            .doc(&format!(
                "Request builder for the {} telegram method call.",
                method.name
            ));

        for arg in &method.constructor {
            struct_.push_field(arg.to_field("Constructor field"));
        }
        for item in items {
            struct_.push_field(item.to_field());
        }
    }

    let impl_ = scope
        .new_impl(&struct_name)
        .generic("'a")
        .target_generic("'a");
    {
        let new = impl_
            .new_fn("new")
            .vis("pub")
            .ret("Self")
            .doc(format!(
                "Creates [`{struct_name}`] request builder."
            ))
            .arg("client", "&'a reqwest::Client")
            .arg("url", "reqwest::Url");
        new.line("Self {");
        new.line("    client,\n    url,");

        for arg in &method.constructor {
            new.arg(&arg.name, arg.to_ty());
            new.line(format_args!("    {}, ", arg.name));
        }

        for arg in method.builder.iter().flatten() {
            new.line(format_args!(
                "    {}: {},",
                arg.name,
                if let Some(ref def) = arg.default {
                    def.clone()
                } else {
                    "None".to_owned()
                }
            ));
        }

        new.line("}");
    }

    fn push_setter(
        i: &mut Impl,
        name: &str,
        plural: &str,
        ty: impl Display,
        list: bool,
    ) {
        let setter = i
            .new_fn(name)
            .vis("pub")
            .arg_mut_self()
            .arg(name, format!("impl Into<{ty}>"))
            .ret("&mut Self")
            .doc(format!("Sets `{name}` field of the request."))
            .allow("clippy::useless_conversion");

        if list {
            setter.line(format_args!(
                "self.{plural}.push({name}.into());"
            ));
        } else {
            setter.line(format_args!(
                "self.{name} = {name}.into().into();"
            ));
        }
        setter.line("self");
    }

    for arg in &method.constructor {
        push_setter(impl_, &arg.name, "", arg.to_ty(), false);
    }

    for arg in method.builder.iter().flatten() {
        let name = if arg.list {
            let singular = arg
                .name
                .strip_suffix('s')
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| arg.name.clone());
            singular
        } else {
            arg.name.clone()
        };
        push_setter(impl_, &name, &arg.name, &arg.type_, arg.list);
    }

    let send_fn = impl_
        .new_fn("send")
        .arg_ref_self()
        .ret(format!("ApiResult<{}>", method.returns))
        .vis("pub")
        .set_async(true)
        .doc("Sends request and deserializes expected data");
    send_fn.line("#[derive(serde::Serialize)]");
    send_fn.line("struct Params<'rref> {");
    for constructor in &method.constructor {
        let ty = constructor.to_ty();
        send_fn
            .line(format!("    {}: &'rref {ty},", constructor.name));
    }
    for builder in method.builder.iter().flatten() {
        let ty = builder.to_ser_ty();
        let rref = "&'rref ";
        send_fn.line(format!("    {}: {rref}{ty},", builder.name));
    }
    send_fn.line("}");
    send_fn.line("let params: Params = Params {");
    send_fn.line(generate_args_from_constructor(&method.constructor));

    if let Some(ref b) = method.builder {
        send_fn.line(generate_args_from_builder(b));
    }

    send_fn.line("};");

    send_fn
        .line("self.client")
        .line("    .post(self.url.clone())")
        .line("    .form(&params)")
        .line("    .send()")
        .line("    .await?")
        .line("    .json::<ApiResponse<_>>()")
        .line("    .await?")
        .line("    .into()");
}

fn create_fn_from_constructor<'a>(
    scope: &'a mut Scope,
    method: &Method,
    ret: &str,
    r#async: bool,
) -> &'a mut Function {
    scope
        .new_fn(&method.name.to_snake_case())
        .ret(ret)
        .arg_ref_self()
        .vis("pub")
        .set_async(r#async)
}

fn generate_builder_new(scope: &mut Scope, method: &Method) {
    let fn_ = create_fn_from_constructor(
        scope,
        method,
        &format!("{}<'_>", builder_name(&method.name)),
        false,
    );

    fn_.line(format!(
        "{}::new(self.client(),",
        builder_name(&method.name)
    ));
    fn_.line(format!("self.url_for(\"{}\"), ", method.name));

    for arg in &method.constructor {
        fn_.arg(&arg.name, format!("impl Into<{}>", arg.to_ty()));
        fn_.line(format!("{}.into(), ", arg.name));
    }
    fn_.line(")");
}

fn generate_builderless(scope: &mut Scope, method: &Method) {
    let fn_ = create_fn_from_constructor(
        scope,
        method,
        &format!("ApiResult<{}>", method.returns),
        true,
    );

    fn_.line("#[derive(serde::Serialize)]");
    fn_.line("struct Params {");

    for arg in &method.constructor {
        let ty = arg.to_ty();

        fn_.arg(&arg.name, &ty);
        fn_.line(format!("    {}: {ty}, ", arg.name));
    }

    fn_.line("}");

    #[rustfmt::skip]
    fn_.line(format!(
        "self.client()\n\
             .post(self.url_for(\"{method_name}\"))\n\
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
        Some(items) => {
            generate_builder(structures, items, method);
            generate_builder_new(fns, method);
        }
        None => generate_builderless(fns, method),
    }
}

pub fn emit_methods(methods: &[Method]) -> (String, String) {
    let mut structures = Scope::new();
    let mut fns = Scope::new();
    methods
        .iter()
        .for_each(|m| emit_method(&mut structures, &mut fns, m));

    (
        structures.to_string(),
        format!("impl Bot {{ {} }}", fns.to_string()),
    )
}
