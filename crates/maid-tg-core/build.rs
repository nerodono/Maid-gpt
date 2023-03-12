use std::{
    env,
    fs,
    path::Path,
};

use maid_tg_codegen::{
    methods::MethodsSchema,
    schemas::*,
};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let schemas_path = Path::new(&out_dir).join("schemas.rs");
    let methods_path = Path::new(&out_dir).join("methods.rs");

    let schema = Schema::parse_schema(
        &fs::read_to_string("schemas.yaml").unwrap(),
    );
    let methods = MethodsSchema::parse_schema(
        &fs::read_to_string("methods.yaml").unwrap(),
    );

    fs::write(schemas_path, schema.emit()).unwrap();
    fs::write(methods_path, methods.emit()).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=methods.yaml");
    println!("cargo:rerun-if-changed=schemas.yaml");
}
