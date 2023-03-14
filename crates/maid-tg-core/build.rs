use std::{
    env,
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use maid_tg_core_codegen::{
    methods::{
        emitter::emit_methods,
        schemas::*,
    },
    parse_yaml,
    read,
    types::{
        emitter::*,
        schemas::*,
    },
    write,
};

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let models: Vec<Model> = parse_yaml(read("types.yml"));
    let methods: Vec<Method> = parse_yaml(read("methods.yml"));

    let (structures, _) = emit_methods(&methods);

    write(out_dir.join("schemas_generated.rs"), emit_models(&models));
    write(out_dir.join("builders_generated.rs"), structures);

    println!("cargo:rerun-if-changed=methods.yml");
    println!("cargo:rerun-if-changed=types.yml");

    println!("cargo:rerun-if-changed=build.rs");
}
