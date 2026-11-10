//! Launch application on an Android device or emulator.

use std::path::Path;
use std::process::Command;

use crate::args::RunArgs;

pub fn execute(args: &RunArgs) -> Result<(), String> {
    println!("🚀 Launching Ferrix application on Android device...\n");

    let android_dir = Path::new("android");
    if !android_dir.exists() {
        return Err("No android/ directory found. Run 'ferrix prebuild' first.".to_string());
    }

    // 1. Build and install via Gradle
    let task = if args.release {
        "installRelease"
    } else {
        "installDebug"
    };

    println!("  📦 Installing APK via Gradle task '{}'...", task);

    let gradlew_path = if cfg!(windows) {
        android_dir.join("gradlew.bat")
    } else {
        android_dir.join("gradlew")
    };

    let mut gradle_cmd = if gradlew_path.exists() {
        Command::new(gradlew_path)
    } else {
        Command::new("gradle")
    };

    gradle_cmd.current_dir(android_dir).arg(task);
    let status = gradle_cmd
        .status()
        .map_err(|e| format!("Failed to install APK: {}", e))?;

    if !status.success() {
        return Err(format!("Gradle installation task '{}' failed.", task));
    }

    // 2. Discover package name and launch via ADB
    let pkg_name = read_package_name().unwrap_or_else(|| "com.example.app".to_string());
    let activity = format!("{}/.MainActivity", pkg_name);

    println!("  📱 Starting activity '{}' via ADB...", activity);

    let mut adb_cmd = Command::new("adb");
    if let Some(dev) = &args.device {
        adb_cmd.args(["-s", dev]);
    }
    adb_cmd.args(["shell", "am", "start", "-n", &activity]);

    let adb_status = adb_cmd.status();
    if adb_status.is_ok_and(|s| s.success()) {
        println!("\n✅ Application launched successfully on target device!");
        Ok(())
    } else {
        Err("Failed to start activity via ADB. Ensure device is connected.".to_string())
    }
}

fn read_package_name() -> Option<String> {
    if let Ok(config_str) = std::fs::read_to_string("ferrix.config.toml") {
        for line in config_str.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("package =") {
                let parts: Vec<&str> = trimmed.split('=').collect();
                if parts.len() >= 2 {
                    return Some(parts[1].trim().trim_matches('"').to_string());
                }
            }
        }
    }
    None
}
