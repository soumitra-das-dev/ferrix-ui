//! AOT compiler command for single .fsx source files.

use std::path::{Path, PathBuf};

use crate::args::CompileArgs;

pub fn execute(args: &CompileArgs) -> Result<(), String> {
    let input_p = Path::new(&args.input);
    if !input_p.exists() {
        return Err(format!("Input file '{}' does not exist.", args.input));
    }

    let out_p = args
        .output
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| input_p.with_extension("rs"));

    println!(
        "⚡ Compiling {} -> {}...",
        input_p.display(),
        out_p.display()
    );

    match ferrix_compiler::compile_file(input_p, &out_p) {
        Ok(()) => {
            println!("✅ Compilation succeeded!");
            Ok(())
        }
        Err(err) => Err(format!("Compilation failed:\n{}", err)),
    }
}
