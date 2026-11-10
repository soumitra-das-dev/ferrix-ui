//! Environment variable parsing, layering, and client exposure filtering.

use std::collections::HashMap;
use std::path::Path;

/// Parses a `.env` string into key-value pairs.
pub fn parse_env_str(content: &str) -> HashMap<String, String> {
    let mut vars = HashMap::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some((k, v)) = trimmed.split_once('=') {
            let key = k.trim().to_string();
            let mut val = v.trim().to_string();

            // Strip enclosing quotes if present
            let is_quoted = (val.starts_with('"') && val.ends_with('"'))
                || (val.starts_with('\'') && val.ends_with('\''));
            if is_quoted && val.len() >= 2 {
                val = val[1..val.len() - 1].to_string();
            }

            vars.insert(key, val);
        }
    }

    vars
}

/// Loads and layers `.env` files according to precedence:
/// `.env.<variant>.local` > `.env.<variant>` > `.env.local` > `.env`
pub fn load_env_layers(base_dir: &Path, variant: Option<&str>) -> HashMap<String, String> {
    let mut merged = HashMap::new();

    let mut candidate_files = vec![base_dir.join(".env"), base_dir.join(".env.local")];

    if let Some(v) = variant {
        candidate_files.push(base_dir.join(format!(".env.{}", v)));
        candidate_files.push(base_dir.join(format!(".env.{}.local", v)));
    }

    for file_path in candidate_files {
        if file_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                let parsed = parse_env_str(&content);
                merged.extend(parsed);
            }
        }
    }

    merged
}

/// Filters environment variables for client binary injection.
/// Only keys prefixed with `FERRIX_PUBLIC_` or `EXPO_PUBLIC_` are retained.
pub fn filter_public_env(all_vars: &HashMap<String, String>) -> HashMap<String, String> {
    all_vars
        .iter()
        .filter(|(k, _)| k.starts_with("FERRIX_PUBLIC_") || k.starts_with("EXPO_PUBLIC_"))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}
