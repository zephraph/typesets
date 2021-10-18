use std::{env, fs, path::Path};

use codegen::build;

fn main() {
    let content = build();
    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");
    fs::write(out_file, &content).unwrap();
}
