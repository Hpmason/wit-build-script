use std::path::PathBuf;

use wit_guest_codegen::GenOptions;

fn main() {
    let wit_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wit");

    let gen = GenOptions::new()
        .with_rustfmt()
        .build(wit_path);

    gen.generate("plugin", "./src/gen").unwrap();
    // Only run if a .wit file changes, or this build script changes
    println!("cargo:rerun-if-changed=../wit");
    println!("cargo:rerun-if-changed=build.rs");
}
