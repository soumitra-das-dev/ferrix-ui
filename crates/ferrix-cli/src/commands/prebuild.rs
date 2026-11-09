//! Materialize or synchronize the native Android Studio project.

use std::fs;
use std::path::Path;

use crate::args::PrebuildArgs;
use crate::templates::scaffold_android_project;

pub fn execute(args: &PrebuildArgs) -> Result<(), String> {
    println!("⚙️ Materializing native Android project...\n");

    let config_path = Path::new("ferrix.config.toml");
    if !config_path.exists() {
        return Err("No ferrix.config.toml found in current directory.".to_string());
    }

    let config_str = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read ferrix.config.toml: {}", e))?;

    let mut slug = "ferrix-app".to_string();
    let mut pkg_name = "ferrix_app".to_string();

    for line in config_str.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("slug =") {
            if let Some(val) = trimmed.split('=').nth(1) {
                slug = val.trim().trim_matches('"').to_string();
            }
        } else if trimmed.starts_with("package =") {
            if let Some(val) = trimmed.split('=').nth(1) {
                let full_pkg = val.trim().trim_matches('"');
                if let Some(last) = full_pkg.split('.').next_back() {
                    pkg_name = last.to_string();
                }
            }
        }
    }

    let android_dir = Path::new("android");
    if args.clean && android_dir.exists() {
        println!("  🧹 Cleaning existing android/ directory...");
        let _ = fs::remove_dir_all(android_dir);
    }

    scaffold_android_project(Path::new("."), &slug, &pkg_name)
        .map_err(|e| format!("Failed to scaffold Android project: {}", e))?;

    println!("  ✅ Materialized android/ directory with Gradle wrapper and AndroidManifest.xml");
    println!("  📱 You can now open 'android/' directly in Android Studio!");
    Ok(())
}
