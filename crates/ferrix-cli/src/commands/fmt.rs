//! Lossless formatting command for .fsx components.

use std::fs;
use walkdir::WalkDir;

use crate::args::FmtArgs;
use ferrix_fmt::format_source;

pub fn execute(args: &FmtArgs) -> Result<(), String> {
    let target_path = args.path.as_deref().unwrap_or(".");
    let check_mode = args.check;

    if check_mode {
        println!(
            "Checking formatting for .fsx files in '{}'...\n",
            target_path
        );
    } else {
        println!("Formatting .fsx files in '{}'...\n", target_path);
    }

    let mut unformatted_files = Vec::new();
    let mut formatted_count = 0;

    for entry in WalkDir::new(target_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "fsx"))
    {
        let path = entry.path();
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read {}: {}", path.display(), e);
                continue;
            }
        };

        let formatted = format_source(&content);

        if formatted != content {
            if check_mode {
                unformatted_files.push(path.to_path_buf());
                println!("  ❌ Unformatted: {}", path.display());
            } else {
                if let Err(e) = fs::write(path, formatted) {
                    eprintln!("  Failed to write {}: {}", path.display(), e);
                } else {
                    formatted_count += 1;
                    println!("  ✨ Formatted: {}", path.display());
                }
            }
        }
    }

    if check_mode {
        if !unformatted_files.is_empty() {
            Err(format!(
                "{} file(s) require formatting. Run 'ferrix fmt' to format them.",
                unformatted_files.len()
            ))
        } else {
            println!("✅ All .fsx files are properly formatted.");
            Ok(())
        }
    } else {
        println!(
            "\n✅ Formatting complete. {} file(s) updated.",
            formatted_count
        );
        Ok(())
    }
}
