//! Launch application on an Android device or emulator with honest status reporting.

use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use crate::args::RunArgs;
use crate::crash::classify_crash;
use crate::device::resolve_device;

pub fn execute(args: &RunArgs) -> Result<(), String> {
    println!("🚀 Launching Ferrix application on Android device...\n");

    let android_dir = Path::new("android");
    if !android_dir.exists() {
        return Err("No android/ directory found. Run 'ferrix prebuild' first.".to_string());
    }

    // Auto-heal missing or incomplete gradle.properties
    let _ = crate::templates::ensure_gradle_properties(android_dir);

    // 0. Resolve target device and ABI
    let device = resolve_device(args.device.as_deref())?;
    let ndk_target = device.ndk_target();

    println!("  📱 Target Device: {} ({})", device.model, device.serial);
    println!("  🎯 Target ABI:    {}", device.primary_abi);

    // 1. Compile Rust native JNI library for device ABI
    println!("  🔨 Compiling Rust JNI library for '{}'...", ndk_target);
    let mut cargo_ndk = Command::new("cargo");
    cargo_ndk.args([
        "ndk",
        "--target",
        ndk_target,
        "-o",
        "android/app/src/main/jniLibs",
        "build",
    ]);
    if args.release {
        cargo_ndk.arg("--release");
    }
    let ndk_status = cargo_ndk
        .status()
        .map_err(|e| format!("Failed to run 'cargo ndk': {}", e))?;
    if !ndk_status.success() {
        return Err(format!(
            "Failed to compile native Rust library for target '{}' using 'cargo ndk'. Ensure Android NDK is installed and target is added via 'rustup target add {}'.",
            ndk_target, ndk_target
        ));
    }

    // 2. Verify .so exists in jniLibs
    let jni_dir = android_dir
        .join("app/src/main/jniLibs")
        .join(&device.primary_abi);
    if jni_dir.exists() {
        let has_so = std::fs::read_dir(&jni_dir)
            .ok()
            .and_then(|mut entries| {
                entries.find(|e| {
                    e.as_ref()
                        .ok()
                        .is_some_and(|f| f.path().extension().is_some_and(|ext| ext == "so"))
                })
            })
            .is_some();
        if !has_so {
            return Err(format!(
                "No native .so was packaged in {} after cargo ndk compilation.",
                jni_dir.display()
            ));
        }
    }

    // 3. Build APK via Gradle
    let (task, apk_subdir) = if args.release {
        ("assembleRelease", "release/app-release-unsigned.apk")
    } else {
        ("assembleDebug", "debug/app-debug.apk")
    };

    println!("  📦 Building APK via Gradle task '{}'...", task);

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
        .map_err(|e| format!("Failed to build APK: {}", e))?;

    if !status.success() {
        return Err(format!("Gradle build task '{}' failed.", task));
    }

    let apk_path = android_dir.join("app/build/outputs/apk").join(apk_subdir);
    if apk_path.exists() {
        println!("  📲 Installing APK via ADB to '{}'...", device.serial);
        let mut install_cmd = Command::new("adb");
        install_cmd
            .args(["-s", &device.serial, "install", "-r"])
            .arg(&apk_path);
        let install_status = install_cmd
            .status()
            .map_err(|e| format!("ADB install failed: {}", e))?;
        if !install_status.success() {
            return Err(format!("Failed to install APK on '{}'.", device.serial));
        }
    }

    // 4. Clear logcat
    let _ = Command::new("adb")
        .args(["-s", &device.serial, "logcat", "-c"])
        .status();

    // 5. Discover package name and launch via ADB
    let pkg_name = read_package_name().unwrap_or_else(|| "com.example.app".to_string());
    let activity = format!("{}/dev.ferrix.host.MainActivity", pkg_name);

    println!("  🚀 Starting activity '{}' via ADB...", activity);

    let launch_status = Command::new("adb")
        .args([
            "-s",
            &device.serial,
            "shell",
            "am",
            "start",
            "-n",
            &activity,
        ])
        .status()
        .map_err(|e| format!("Failed to start activity via ADB: {}", e))?;

    if !launch_status.success() {
        return Err("Failed to launch activity via ADB.".to_string());
    }

    // 6. Honest readiness confirmation: poll logcat for ferrix:ready or immediate crash
    println!("  ⏳ Awaiting runtime readiness marker ('ferrix:ready')...");
    let deadline = Instant::now() + Duration::from_secs(15);
    let mut recent_lines = Vec::new();

    while Instant::now() < deadline {
        if let Ok(output) = Command::new("adb")
            .args(["-s", &device.serial, "logcat", "-d", "-t", "80"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                recent_lines.push(line.to_string());
                if line.contains("ferrix:ready") {
                    println!("\n✅ Application launched and runtime initialized successfully!");
                    return Ok(());
                }
            }

            if let Some(crash) = classify_crash(&recent_lines) {
                return Err(format!(
                    "Application crashed immediately after launch:\n{}",
                    crash.display_formatted()
                ));
            }
        }
        thread::sleep(Duration::from_millis(200));
    }

    println!("\n⚠️ Activity scheduled, but runtime readiness marker was not observed within 15s.");
    Ok(())
}

pub fn read_package_name() -> Option<String> {
    if let Ok(config_str) = std::fs::read_to_string("ferrix.config.toml") {
        if let Ok(config) = ferrix_config::FerrixConfig::parse(&config_str) {
            return Some(config.app.id);
        }
    }
    None
}
