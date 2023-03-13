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
    types::{
        emitter::*,
        schemas::*,
    },
};

fn write(to: impl AsRef<Path>, data: impl AsRef<[u8]>) {
    fs::write(to, data).unwrap();
}

fn read(p: impl AsRef<Path>) -> String {
    fs::read_to_string(p).unwrap()
}

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let models: Vec<Model> = parse_yaml(read("types.yml"));
    let methods: Vec<Method> = parse_yaml(read("methods.yml"));

    let (structures, fns) = emit_methods(&methods);

    write(out_dir.join("schemas_generated.rs"), emit_models(&models));
    write("local.rs", format!("{structures}\n\n{fns}"));

    println!("cargo:rerun-if-changed=methods.ron");
    println!("cargo:rerun-if-changed=types.ron");

    println!("cargo:rerun-if-changed=build.rs");
}
