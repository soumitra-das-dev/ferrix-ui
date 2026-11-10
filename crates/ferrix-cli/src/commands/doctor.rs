//! Toolchain diagnosis and remediation engine.

use std::env;
use std::path::Path;
use std::process::Command;

use crate::args::DoctorArgs;

pub fn execute(args: &DoctorArgs) -> Result<(), String> {
    println!("🩺 Running Ferrix Doctor diagnostics...\n");

    let mut all_ok = true;

    // 1. Check Rustc
    let rustc_ok = match Command::new("rustc").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("  ✅ Rust Compiler: {}", ver);
            true
        }
        _ => {
            println!("  ❌ Rust Compiler: Not found in PATH");
            println!("     Fix: Install Rust via https://rustup.rs");
            all_ok = false;
            false
        }
    };

    // 2. Check Cargo
    if rustc_ok {
        match Command::new("cargo").arg("--version").output() {
            Ok(output) if output.status.success() => {
                let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("  ✅ Cargo Package Manager: {}", ver);
            }
            _ => {
                println!("  ❌ Cargo: Not found");
                all_ok = false;
            }
        }
    }

    // 3. Check Rust Android Targets
    let installed_targets = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
        .unwrap_or_default();

    let has_aarch64 = installed_targets.contains("aarch64-linux-android");
    let has_x86_64 = installed_targets.contains("x86_64-linux-android");

    if has_aarch64 && has_x86_64 {
        println!(
            "  ✅ Rust Android Targets: aarch64-linux-android, x86_64-linux-android installed"
        );
    } else {
        println!(
            "  ⚠️ Rust Android Targets: Missing standard targets (aarch64: {}, x86_64: {})",
            has_aarch64, has_x86_64
        );
        if args.fix {
            println!("     Remediating: Adding missing Android targets...");
            let status = Command::new("rustup")
                .args([
                    "target",
                    "add",
                    "aarch64-linux-android",
                    "x86_64-linux-android",
                ])
                .status();
            if status.is_ok_and(|s| s.success()) {
                println!("     ✅ Successfully installed Android targets");
            } else {
                println!("     ❌ Failed to install targets automatically");
                all_ok = false;
            }
        } else {
            println!("     Fix: Run 'rustup target add aarch64-linux-android x86_64-linux-android' or rerun with '--fix'");
            all_ok = false;
        }
    }

    // 4. Check ANDROID_HOME / ANDROID_SDK_ROOT
    let sdk_path = env::var("ANDROID_HOME")
        .or_else(|_| env::var("ANDROID_SDK_ROOT"))
        .ok();

    match sdk_path {
        Some(p) if Path::new(&p).exists() => {
            println!("  ✅ Android SDK: Found at {}", p);

            // Check NDK
            let ndk_dir = Path::new(&p).join("ndk");
            let has_ndk = ndk_dir.exists()
                || env::var("ANDROID_NDK_ROOT").is_ok_and(|ndk| Path::new(&ndk).exists());

            if has_ndk {
                println!("  ✅ Android NDK: Installed and available");
            } else {
                println!("  ⚠️ Android NDK: Not found in SDK or ANDROID_NDK_ROOT");
                println!("     Fix: Install NDK via Android Studio SDK Manager or sdkmanager 'ndk;26.1.10909125'");
            }
        }
        Some(p) => {
            println!("  ❌ Android SDK: Path {} does not exist on disk", p);
            all_ok = false;
        }
        None => {
            println!("  ❌ Android SDK: Neither ANDROID_HOME nor ANDROID_SDK_ROOT is set");
            println!("     Fix: Export ANDROID_HOME=$HOME/Android/Sdk in your ~/.bashrc or env");
            all_ok = false;
        }
    }

    // 5. Check Java (JDK 17)
    match Command::new("javac").arg("-version").output() {
        Ok(output) if output.status.success() => {
            let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("  ✅ Java Development Kit (JDK): {}", ver);
        }
        _ => match Command::new("java").arg("-version").output() {
            Ok(output) => {
                let err = String::from_utf8_lossy(&output.stderr);
                let first_line = err.lines().next().unwrap_or("Java runtime");
                println!("  ✅ Java: {}", first_line);
            }
            _ => {
                println!("  ❌ Java (JDK 17): Not found in PATH");
                println!("     Fix: Install OpenJDK 17 (e.g. 'sudo apt install openjdk-17-jdk')");
                all_ok = false;
            }
        },
    }

    // 6. Check ADB
    match Command::new("adb").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let ver_line = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or("Android Debug Bridge")
                .to_string();
            println!("  ✅ Android Debug Bridge (ADB): {}", ver_line);

            // List connected devices
            if let Ok(dev_out) = Command::new("adb").arg("devices").output() {
                let dev_str = String::from_utf8_lossy(&dev_out.stdout);
                let devices: Vec<&str> = dev_str
                    .lines()
                    .skip(1)
                    .filter(|l| !l.trim().is_empty())
                    .collect();
                if devices.is_empty() {
                    println!("     ℹ️ No devices/emulators currently attached (run 'emulator' or plug in device)");
                } else {
                    println!(
                        "     📱 Connected targets: {} device(s) online",
                        devices.len()
                    );
                }
            }
        }
        _ => {
            println!("  ⚠️ ADB: Not found in PATH");
            println!("     Fix: Add $ANDROID_HOME/platform-tools to PATH");
        }
    }

    println!();
    if all_ok {
        println!("✨ Doctor found no critical issues. Your environment is ready for Ferrix!");
        Ok(())
    } else {
        Err("One or more toolchain dependencies need attention (see above).".to_string())
    }
}
