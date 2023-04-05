use std::path::PathBuf;

use wit_component_build_script::*;

fn main() {
    let wit_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wit");
    
    let gen = GenOptions::new()
        .with_rustfmt()
        .build(wit_path);
    
    gen.generate("plugin", "./wasm-host.rs").unwrap();
}