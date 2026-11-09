use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/component.fsx");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("component_generated.rs");

    if let Err(err) = ferrix_compiler::compile_file("src/component.fsx", &dest_path) {
        panic!("Failed to compile .fsx UI file: {}", err);
    }
}
