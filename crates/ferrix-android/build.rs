use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let fsx_file = "../../examples/counter/component.fsx";
    println!("cargo:rerun-if-changed={}", fsx_file);

    let output_file = out_dir.join("counter_generated.rs");
    if let Err(e) = ferrix_compiler::compile_file(fsx_file, &output_file) {
        panic!("Failed to compile {} to Rust: {}", fsx_file, e);
    }
}
