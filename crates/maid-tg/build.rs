use std::{
    env,
    path::PathBuf,
};

use maid_tg_core_codegen::{
    methods::{
        emitter::emit_methods,
        schemas::Method,
    },
    parse_yaml,
    read,
    write,
};

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let methods: Vec<Method> =
        parse_yaml(read("../maid-tg-core/methods.yml"));

    let (_, fns) = emit_methods(&methods);

    write(out_dir.join("methods_generated.rs"), fns);
    //write("local.rs", fns);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../maid-tg-core/methods.yml");
}
