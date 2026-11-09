//! Ferrix CLI - AOT compiler and tooling for Ferrix UI apps.

use std::env;
use std::path::{Path, PathBuf};
use std::process;

fn print_usage() {
    eprintln!(
        r#"Ferrix CLI v{version}
Usage:
    ferrix compile <input.fsx> [-o <output.rs>]
    ferrix --version
    ferrix --help

Commands:
    compile    Compile a .fsx declarative UI file to native Rust code
"#,
        version = env!("CARGO_PKG_VERSION")
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "--version" | "-V" => {
            println!("ferrix {}", env!("CARGO_PKG_VERSION"));
        }
        "--help" | "-h" | "help" => {
            print_usage();
        }
        "compile" => {
            if args.len() < 3 {
                eprintln!("Error: 'compile' requires an input .fsx file.");
                print_usage();
                process::exit(1);
            }
            let input_path = &args[2];
            let mut output_path = None;
            let mut idx = 3;
            while idx < args.len() {
                if args[idx] == "-o" || args[idx] == "--output" {
                    if idx + 1 < args.len() {
                        output_path = Some(args[idx + 1].clone());
                        idx += 2;
                    } else {
                        eprintln!("Error: -o/--output requires a path argument.");
                        process::exit(1);
                    }
                } else {
                    eprintln!("Unknown option: {}", args[idx]);
                    process::exit(1);
                }
            }

            let input_p = Path::new(input_path);
            let out_p = output_path
                .map(PathBuf::from)
                .unwrap_or_else(|| input_p.with_extension("rs"));

            match ferrix_compiler::compile_file(input_p, &out_p) {
                Ok(()) => {
                    println!(
                        "Successfully compiled {} -> {}",
                        input_p.display(),
                        out_p.display()
                    );
                }
                Err(err) => {
                    eprintln!("{}", err);
                    process::exit(1);
                }
            }
        }
        other => {
            eprintln!("Unknown command: {}", other);
            print_usage();
            process::exit(1);
        }
    }
}
