use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    let mut generated_mods = String::new();
    compile_fsx_directory(Path::new("src"), out_path, &mut generated_mods);

    let mod_index_path = out_path.join("generated_modules.rs");
    let _ = fs::write(mod_index_path, generated_mods);
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    let mut prev_is_sep = false;
    for (i, ch) in s.chars().enumerate() {
        if ch == '-' || ch == '_' || ch == '/' || ch == '\\' {
            if !prev_is_sep && !out.is_empty() {
                out.push('_');
            }
            prev_is_sep = true;
        } else if ch.is_uppercase() {
            if i > 0 && !prev_is_sep {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
            prev_is_sep = false;
        } else {
            out.push(ch);
            prev_is_sep = false;
        }
    }
    out
}

fn compile_fsx_directory(dir: &Path, out_dir: &Path, mods: &mut String) {
    if let Ok(entries) = fs::read_dir(dir) {
        let mut paths: Vec<_> = entries.flatten().map(|e| e.path()).collect();
        paths.sort();
        for path in paths {
            if path.is_dir() {
                compile_fsx_directory(&path, out_dir, mods);
            } else if path.extension().and_then(|s| s.to_str()) == Some("fsx") {
                println!("cargo:rerun-if-changed={}", path.display());
                let rel_path = path.strip_prefix("src").unwrap_or(&path);
                let safe_stem = rel_path
                    .to_string_lossy()
                    .trim_start_matches('/')
                    .replace(['/', '\\'], "_")
                    .replace(".fsx", "");
                let dest = out_dir.join(format!("{}_generated.rs", safe_stem));
                if let Err(err) = ferrix_compiler::compile_file(&path, &dest) {
                    panic!("Failed to compile '{}': {}", path.display(), err);
                }
                let mod_name = to_snake_case(&safe_stem);
                mods.push_str(&format!(
                    "pub mod {} {{\n    // Ahead-of-time compiled from {}\n    include!(concat!(env!(\"OUT_DIR\"), \"/{}_generated.rs\"));\n}}\n",
                    mod_name,
                    path.display(),
                    safe_stem
                ));
            }
        }
    }
}
