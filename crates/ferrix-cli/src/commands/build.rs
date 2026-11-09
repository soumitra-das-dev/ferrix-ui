//! Production release packaging for APK and AAB.

use std::env;
use std::path::Path;
use std::process::Command;

use crate::args::BuildArgs;

pub fn execute(args: &BuildArgs) -> Result<(), String> {
    let mode = if args.release { "release" } else { "debug" };
    println!("🏗️ Building Ferrix Android app in {} mode...\n", mode);

    let android_dir = Path::new("android");
    if !android_dir.exists() {
        return Err("No android/ directory found. Run 'ferrix prebuild' first.".to_string());
    }

    // 1. Compile Rust native libraries using cargo-ndk
    let abi_target = match args.abi.as_str() {
        "x86_64" => "x86_64-linux-android",
        "armv7" => "armv7-linux-androideabi",
        _ => "aarch64-linux-android",
    };

    println!(
        "  🔨 Compiling Rust JNI library for target '{}'...",
        abi_target
    );

    let mut cargo_cmd = Command::new("cargo");
    cargo_cmd.args([
        "ndk",
        "--target",
        abi_target,
        "-o",
        "android/app/src/main/jniLibs",
        "build",
    ]);

    if args.release {
        cargo_cmd.arg("--release");
    }

    let ndk_status = cargo_cmd.status();
    let ndk_ok = ndk_status.is_ok_and(|s| s.success());

    if !ndk_ok {
        println!(
            "  ⚠️ 'cargo ndk' build skipped or not found. Falling back to host cargo build..."
        );
        let mut fallback = Command::new("cargo");
        fallback.arg("build");
        if args.release {
            fallback.arg("--release");
        }
        let _ = fallback.status();
    }

    // 2. Run Gradle build
    let gradle_task = if args.aab {
        if args.release {
            "bundleRelease"
        } else {
            "bundleDebug"
        }
    } else {
        if args.release {
            "assembleRelease"
        } else {
            "assembleDebug"
        }
    };

    println!("  📦 Running Gradle task '{}'...", gradle_task);

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

    gradle_cmd.current_dir(android_dir).arg(gradle_task);

    // Keystore signing environment injection
    if args.release {
        if let Ok(ks) = env::var("FERRIX_KEYSTORE_PATH") {
            println!(
                "  🔑 Using release keystore from FERRIX_KEYSTORE_PATH: {}",
                ks
            );
        } else {
            println!(
                "  ℹ️ No FERRIX_KEYSTORE_PATH set; APK/AAB signed with default debug keystore"
            );
        }
    }

    let status = gradle_cmd
        .status()
        .map_err(|e| format!("Failed to execute Gradle: {}", e))?;

    if status.success() {
        println!("\n✅ Build succeeded!");
        let output_type = if args.aab { "AAB" } else { "APK" };
        println!(
            "   Generated {} artifact located in android/app/build/outputs/",
            output_type
        );
        Ok(())
    } else {
        Err(format!("Gradle build task '{}' failed.", gradle_task))
    }
}
