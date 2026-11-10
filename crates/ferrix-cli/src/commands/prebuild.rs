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

    let config = ferrix_config::FerrixConfig::parse(&config_str)
        .map_err(|e| format!("Failed to parse ferrix.config.toml: {}", e))?;
    let slug = config.app.slug.clone();
    let pkg_name = slug.replace('-', "_");

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
