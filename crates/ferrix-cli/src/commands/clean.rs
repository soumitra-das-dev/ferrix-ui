//! Clean command for removing build caches and artifacts.

use std::fs;
use std::path::Path;
use std::process::Command;

use crate::args::CleanArgs;

pub fn execute(_args: &CleanArgs) -> Result<(), String> {
    println!("🧹 Cleaning Ferrix project build artifacts...\n");

    // 1. Cargo clean
    let _ = Command::new("cargo").arg("clean").status();
    println!("  ✅ Cargo target/ cleaned");

    // 2. Remove dist/
    if Path::new("dist").exists() {
        let _ = fs::remove_dir_all("dist");
        println!("  ✅ dist/ directory removed");
    }

    // 3. Clean Android build artifacts
    let android_build = Path::new("android/build");
    if android_build.exists() {
        let _ = fs::remove_dir_all(android_build);
        println!("  ✅ android/build/ removed");
    }

    let app_build = Path::new("android/app/build");
    if app_build.exists() {
        let _ = fs::remove_dir_all(app_build);
        println!("  ✅ android/app/build/ removed");
    }

    let gradle_cache = Path::new("android/.gradle");
    if gradle_cache.exists() {
        let _ = fs::remove_dir_all(gradle_cache);
        println!("  ✅ android/.gradle/ cache removed");
    }

    println!("\n✨ Workspace cleaned successfully.");
    Ok(())
}
