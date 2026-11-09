//! Static type checking and linter validation for .fsx components.

use std::fs;
use walkdir::WalkDir;

use crate::args::CheckArgs;
use ferrix_compiler::Parser;
use ferrix_lint::LintEngine;

pub fn execute(args: &CheckArgs) -> Result<(), String> {
    let target_path = args.path.as_deref().unwrap_or(".");
    println!("🔍 Checking Ferrix UI components in '{}'...\n", target_path);

    let mut total_files = 0;
    let mut total_errors = 0;
    let mut total_warnings = 0;

    let engine = LintEngine::new();

    for entry in WalkDir::new(target_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "fsx"))
    {
        total_files += 1;
        let path = entry.path();
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("❌ Failed to read {}: {}", path.display(), e);
                total_errors += 1;
                continue;
            }
        };

        // 1. Syntax / AST Parse check
        let mut parser = match Parser::new(&content) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}: {}", path.display(), e);
                total_errors += 1;
                continue;
            }
        };

        if let Err(e) = parser.parse_source_file() {
            eprintln!("❌ Syntax Error in {}: {}", path.display(), e);
            total_errors += 1;
            continue;
        }

        // 2. Linter rules check
        let diagnostics = engine.lint(&content);
        for diag in diagnostics {
            match diag.severity {
                ferrix_lint::Severity::Error => {
                    total_errors += 1;
                    eprintln!(
                        "  ❌ {}:{}:{} [{}]: {}",
                        path.display(),
                        diag.line,
                        diag.col,
                        diag.rule,
                        diag.message
                    );
                }
                ferrix_lint::Severity::Warning => {
                    total_warnings += 1;
                    println!(
                        "  ⚠️ {}:{}:{} [{}]: {}",
                        path.display(),
                        diag.line,
                        diag.col,
                        diag.rule,
                        diag.message
                    );
                }
                ferrix_lint::Severity::Info => {
                    println!(
                        "  ℹ️ {}:{}:{} [{}]: {}",
                        path.display(),
                        diag.line,
                        diag.col,
                        diag.rule,
                        diag.message
                    );
                }
            }
        }
    }

    println!();
    if total_errors > 0 {
        Err(format!(
            "Check failed with {} error(s) and {} warning(s) across {} files.",
            total_errors, total_warnings, total_files
        ))
    } else {
        println!(
            "✅ Check passed: {} file(s) scanned, 0 errors, {} warning(s).",
            total_files, total_warnings
        );
        Ok(())
    }
}
