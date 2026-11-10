//! # ferrix-compiler
//!
//! AOT compiler for `.fsx` declarative UI components into native Rust.

pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod token;

use std::fs;
use std::path::Path;

pub use codegen::Codegen;
pub use lexer::Lexer;
pub use parser::Parser;
pub use semantic::{Diagnostic, SemanticAnalyzer};

/// Compiles an FSX source string directly into native Rust code.
pub fn compile_fsx_to_rust(source: &str, file_path: &str) -> Result<String, String> {
    compile_fsx_to_rust_with_base_dir(source, file_path, None)
}

/// Compiles an FSX source string with a base directory for resolving local module paths.
pub fn compile_fsx_to_rust_with_base_dir(
    source: &str,
    file_path: &str,
    base_dir: Option<&Path>,
) -> Result<String, String> {
    let mut parser = Parser::new(source).map_err(|e| format!("{}", e))?;
    let ast = parser.parse_source_file().map_err(|e| format!("{}", e))?;

    let mut analyzer = SemanticAnalyzer::new(file_path);
    if let Err(diagnostics) = analyzer.analyze(&ast) {
        let rendered: Vec<String> = diagnostics.iter().map(|d| d.render(source)).collect();
        return Err(rendered.join("\n"));
    }

    let codegen = match base_dir {
        Some(dir) => Codegen::with_base_dir(dir.to_path_buf()),
        None => Codegen::new(),
    };
    Ok(codegen.generate(&ast))
}

/// Compiles an FSX file from disk and writes the generated Rust file to `output_path`.
pub fn compile_file(
    input_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let input = input_path.as_ref();
    let output = output_path.as_ref();

    let source = fs::read_to_string(input)?;
    let base_dir = input.parent();
    let rust_code = compile_fsx_to_rust_with_base_dir(
        &source,
        input.to_str().unwrap_or("component.fsx"),
        base_dir,
    )
    .map_err(|err| format!("Compilation failed:\n{}", err))?;

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(output, rust_code)?;
    Ok(())
}
