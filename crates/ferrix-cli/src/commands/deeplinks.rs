//! CLI command implementation for `ferrix deeplinks`.

use crate::args::{
    AssetlinksArgs, DeeplinkTestArgs, DeeplinkVerifyArgs, DeeplinksAction, DeeplinksArgs,
};
use crate::device;
use ferrix_config::FerrixConfig;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn execute(args: &DeeplinksArgs) -> Result<(), String> {
    execute_at(args, Path::new("ferrix.config.toml"))
}

pub fn execute_at(args: &DeeplinksArgs, config_path: &Path) -> Result<(), String> {
    match &args.action {
        DeeplinksAction::Assetlinks(asset_args) => execute_assetlinks(asset_args, config_path),
        DeeplinksAction::Test(test_args) => execute_test(test_args),
        DeeplinksAction::Verify(verify_args) => execute_verify(verify_args, config_path),
    }
}

fn read_config(config_path: &Path) -> Result<FerrixConfig, String> {
    if !config_path.exists() {
        return Err(format!(
            "Configuration file '{}' not found.",
            config_path.display()
        ));
    }
    let contents = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    FerrixConfig::parse(&contents).map_err(|e| format!("Configuration error: {}", e))
}

fn execute_assetlinks(args: &AssetlinksArgs, config_path: &Path) -> Result<(), String> {
    let config = read_config(config_path)?;
    let default_fingerprint = "14:6D:E9:FD:8E:5B:0F:7B:8E:0A:78:E1:9F:8B:2E:3A:4C:5D:6E:7F:8A:9B:0C:1D:2E:3F:4A:5B:6C:7D:8E:9F";
    let fps: Vec<&str> = if args.fingerprints.is_empty() {
        vec![default_fingerprint]
    } else {
        args.fingerprints.iter().map(|s| s.as_str()).collect()
    };

    let json = config.generate_assetlinks_json(&fps);
    println!(
        "🌐 Android Digital Asset Links (to host at https://<host>/.well-known/assetlinks.json):\n"
    );
    println!("{}", json);
    Ok(())
}

fn execute_test(args: &DeeplinkTestArgs) -> Result<(), String> {
    let dev = device::resolve_device(None)?;
    println!(
        "📲 Testing deep link '{}' on device '{}'...",
        args.url, dev.serial
    );

    let status = Command::new("adb")
        .args([
            "-s",
            &dev.serial,
            "shell",
            "am",
            "start",
            "-a",
            "android.intent.action.VIEW",
            "-d",
            &args.url,
        ])
        .status()
        .map_err(|e| format!("Failed to execute adb am start: {}", e))?;

    if status.success() {
        println!("  ✅ Deep link intent dispatched successfully via ADB.");
        Ok(())
    } else {
        Err("Failed to dispatch deep link intent via ADB.".to_string())
    }
}

fn execute_verify(args: &DeeplinkVerifyArgs, config_path: &Path) -> Result<(), String> {
    let config = read_config(config_path)?;
    let pkg = args.package.clone().unwrap_or(config.app.id);
    let dev = device::resolve_device(None)?;

    println!(
        "🔍 Checking domain verification state for '{}' on '{}'...",
        pkg, dev.serial
    );
    let output = Command::new("adb")
        .args(["-s", &dev.serial, "shell", "pm", "get-app-links", &pkg])
        .output()
        .map_err(|e| format!("Failed to execute adb pm get-app-links: {}", e))?;

    let out_str = String::from_utf8_lossy(&output.stdout);
    if output.status.success() {
        println!("{}", out_str);
        Ok(())
    } else {
        let err_str = String::from_utf8_lossy(&output.stderr);
        Err(format!("pm get-app-links failed: {}", err_str))
    }
}
