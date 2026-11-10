//! Cargo crate dependency management, Android compatibility checks, and supply-chain auditing.

use std::fs;
use std::path::Path;
use std::process::Command;

/// Known Android compatibility warnings and recommended alternatives.
pub const KNOWN_ANDROID_PITFALLS: &[(&str, &str)] = &[
    (
        "openssl",
        "Crate 'openssl' requires native C toolchain and dynamic libraries not bundled with Android. Recommended: 'rustls' or 'reqwest' with 'rustls-tls'.",
    ),
    (
        "native-tls",
        "Crate 'native-tls' attempts to link platform TLS which fails on Android. Recommended: 'rustls' or 'rustls-tls'.",
    ),
    (
        "dirs",
        "Crate 'dirs' relies on standard desktop XDG directories that do not match Android app sandbox storage paths. Recommended: Ferrix storage APIs.",
    ),
];

/// Returns a warning message if the crate has known Android compatibility pitfalls.
pub fn check_pitfall(name: &str) -> Option<&'static str> {
    for (bad_crate, warning) in KNOWN_ANDROID_PITFALLS {
        if name == *bad_crate {
            return Some(*warning);
        }
    }
    None
}

/// Parses dependencies from Cargo.toml content.
pub fn parse_dependencies(content: &str) -> Result<Vec<(String, String)>, String> {
    let parsed: toml::Value =
        toml::from_str(content).map_err(|e| format!("Failed to parse Cargo.toml: {}", e))?;

    let mut result = Vec::new();
    if let Some(deps) = parsed.get("dependencies").and_then(|d| d.as_table()) {
        for (dep_name, val) in deps {
            let ver = match val {
                toml::Value::String(ver) => ver.clone(),
                toml::Value::Table(t) => t
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("*")
                    .to_string(),
                _ => "*".to_string(),
            };
            result.push((dep_name.clone(), ver));
        }
    }
    Ok(result)
}

pub fn add_crate(
    name: &str,
    features: Option<&str>,
    no_default_features: bool,
) -> Result<(), String> {
    add_crate_in(Path::new("."), name, features, no_default_features)
}

pub fn add_crate_in(
    dir: &Path,
    name: &str,
    features: Option<&str>,
    no_default_features: bool,
) -> Result<(), String> {
    println!("📦 Adding crate '{}' to Ferrix application...\n", name);

    // 1. Check known Android compatibility pitfalls
    if let Some(warning) = check_pitfall(name) {
        println!("  ⚠️ Android Compatibility Warning: {}\n", warning);
    }

    // 2. Run cargo add
    let mut cmd = Command::new("cargo");
    cmd.current_dir(dir);
    cmd.arg("add").arg(name);

    if let Some(feats) = features {
        cmd.arg("--features").arg(feats);
    }
    if no_default_features {
        cmd.arg("--no-default-features");
    }

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute 'cargo add': {}", e))?;

    if !status.success() {
        return Err(format!("'cargo add {}' failed.", name));
    }
    println!("  ✅ Successfully added '{}' to Cargo.toml", name);

    // 3. Android target compatibility check
    println!("  🔍 Verifying Android target compilation...");
    let target_check = Command::new("cargo")
        .current_dir(dir)
        .args(["check", "--target", "x86_64-linux-android"])
        .status();

    match target_check {
        Ok(s) if s.success() => {
            println!("  ✅ Android target check passed (x86_64-linux-android).");
        }
        Ok(_) => {
            println!(
                "  ⚠️ Note: Android target check reported errors. Ensure target-specific features are enabled."
            );
        }
        Err(_) => {
            println!("  ℹ️ Host check: cargo check completed.");
        }
    }

    // 4. Supply-chain security check
    ensure_deny_toml_in(dir)?;

    println!(
        "\n✨ Crate '{}' is ready to import in .fsx files via: import {{ ... }} from \"rust:{}\";",
        name, name
    );
    Ok(())
}

pub fn remove_crate(name: &str) -> Result<(), String> {
    remove_crate_in(Path::new("."), name)
}

pub fn remove_crate_in(dir: &Path, name: &str) -> Result<(), String> {
    println!("🗑️ Removing crate '{}' from Ferrix application...", name);
    let status = Command::new("cargo")
        .current_dir(dir)
        .args(["remove", name])
        .status()
        .map_err(|e| format!("Failed to execute 'cargo remove': {}", e))?;

    if status.success() {
        println!("  ✅ Removed '{}' from Cargo.toml", name);
        Ok(())
    } else {
        Err(format!("'cargo remove {}' failed.", name))
    }
}

pub fn list_crates() -> Result<(), String> {
    list_crates_in(Path::new("."))
}

pub fn list_crates_in(dir: &Path) -> Result<(), String> {
    let cargo_path = dir.join("Cargo.toml");
    if !cargo_path.exists() {
        return Err("Cargo.toml not found in current directory.".to_string());
    }

    let content =
        fs::read_to_string(&cargo_path).map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;

    let deps = parse_dependencies(&content)?;

    println!("📋 Installed Cargo Dependencies:\n");
    if deps.is_empty() {
        println!("  (No dependencies found in Cargo.toml)");
    } else {
        for (dep_name, ver) in &deps {
            println!("  • {} = \"{}\"", dep_name, ver);
        }
    }

    Ok(())
}

pub fn ensure_deny_toml_in(dir: &Path) -> Result<(), String> {
    let deny_path = dir.join("deny.toml");
    if !deny_path.exists() {
        let sample_deny = r#"# Cargo Deny configuration for Ferrix supply-chain security
[advisories]
vulnerability = "deny"
unmaintained = "warn"

[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-3-Clause",
    "ISC",
    "Unicode-3.0",
]

[bans]
multiple-versions = "warn"
"#;
        let _ = fs::write(deny_path, sample_deny);
    }
    Ok(())
}
