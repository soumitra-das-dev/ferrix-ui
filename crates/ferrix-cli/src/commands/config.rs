//! CLI command implementation for `ferrix config`.

use crate::args::{ConfigAction, ConfigArgs, ConfigExplainArgs, ConfigPrintArgs};
use ferrix_config::FerrixConfig;
use std::fs;
use std::path::Path;

pub fn execute(args: &ConfigArgs) -> Result<(), String> {
    execute_at(args, Path::new("ferrix.config.toml"))
}

pub fn execute_at(args: &ConfigArgs, config_path: &Path) -> Result<(), String> {
    match &args.action {
        ConfigAction::Check => execute_check(config_path),
        ConfigAction::Print(print_args) => execute_print(print_args, config_path),
        ConfigAction::Explain(explain_args) => execute_explain(explain_args),
    }
}

fn execute_check(config_path: &Path) -> Result<(), String> {
    if !config_path.exists() {
        return Err(format!(
            "Configuration file '{}' not found.",
            config_path.display()
        ));
    }

    let contents = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    println!("🔍 Validating ferrix.config.toml...");
    match FerrixConfig::parse(&contents) {
        Ok(config) => {
            println!("  ✅ Schema version: {}", config.schema_version);
            println!("  ✅ Application ID: '{}'", config.app.id);
            println!("  ✅ Application Name: '{}'", config.app.name);
            println!("  ✅ Version: '{}'", config.app.version);
            println!("  ✅ Orientation: '{}'", config.app.orientation);
            println!("  ✅ Target SDK: {}", config.android.target_sdk);
            if !config.deep_links.schemes.is_empty() {
                println!("  ✅ Deep link schemes: {:?}", config.deep_links.schemes);
            }
            if !config.deep_links.hosts.is_empty() {
                println!(
                    "  ✅ App link hosts: {} host(s) configured",
                    config.deep_links.hosts.len()
                );
            }
            if !config.build.variants.is_empty() {
                println!(
                    "  ✅ Build variants: {:?}",
                    config.build.variants.keys().collect::<Vec<_>>()
                );
            }
            println!("\n✨ 'ferrix.config.toml' is valid and adheres strictly to schema v1.");
            Ok(())
        }
        Err(err) => Err(format!("Validation failed:\n  ❌ {}", err)),
    }
}

fn execute_print(args: &ConfigPrintArgs, config_path: &Path) -> Result<(), String> {
    if !config_path.exists() {
        return Err(format!(
            "Configuration file '{}' not found.",
            config_path.display()
        ));
    }

    let contents = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config =
        FerrixConfig::parse(&contents).map_err(|e| format!("Configuration error: {}", e))?;

    if args.json {
        let json_val = serde_json::to_string_pretty(&config)
            .map_err(|e| format!("Failed to serialize to JSON: {}", e))?;
        println!("{}", json_val);
    } else {
        println!("# Resolved Ferrix Configuration");
        println!(
            "Application ID: {}",
            config.resolved_app_id(args.variant.as_deref())
        );
        println!(
            "Application Name: {}",
            config.resolved_app_name(args.variant.as_deref())
        );
        println!("Version: {}", config.app.version);
        println!("Target SDK: {}", config.android.target_sdk);
        if let Some(variant) = &args.variant {
            println!("Active Variant: {}", variant);
        }
        println!("\nFull TOML representation:");
        let toml_val = toml::to_string_pretty(&config)
            .map_err(|e| format!("Failed to serialize to TOML: {}", e))?;
        println!("{}", toml_val);
    }

    Ok(())
}

fn execute_explain(args: &ConfigExplainArgs) -> Result<(), String> {
    let key = args.key.as_str();
    println!("📖 Configuration key: '{}'", key);
    match key {
        "app.id" => {
            println!("  Description: Android applicationId for Google Play Store.");
            println!("  Rules: ≥ 2 dot-separated segments, ASCII letters/digits/underscores only. NO HYPHENS.");
            println!("  Example: 'com.example.app'");
        }
        "app.name" => {
            println!(
                "  Description: Human-readable launcher label displayed on Android home screen."
            );
            println!("  Example: 'My Cool App'");
        }
        "app.version" => {
            println!("  Description: Semantic version string (major.minor.patch).");
            println!("  Derivation: Determines versionCode via `major * 1_000_000 + minor * 1_000 + patch`.");
            println!("  Example: '1.4.2' -> versionCode 1004002");
        }
        "app.orientation" => {
            println!("  Description: Screen orientation locking for MainActivity.");
            println!("  Options: 'portrait', 'landscape', 'default', 'per-screen'");
            println!("  Default: 'portrait'");
        }
        "android.target_sdk" => {
            println!("  Description: Android API level targeted by the application.");
            println!("  Default: 35 (Required for modern Google Play Store submissions).");
        }
        "splash.background" => {
            println!("  Description: Hex color code (#RGB, #RRGGBB, #AARRGGBB) for Android 12+ SplashScreen background.");
            println!("  Default: '#0B1F3A'");
        }
        "deep_links" => {
            println!("  Description: Deep link schemes and Android App Links (universal links).");
            println!("  Properties: 'schemes' (custom schemes e.g. myapp://), 'hosts' (verified HTTPS hosts).");
        }
        "build.variants" => {
            println!("  Description: Build flavors such as dev, staging, prod.");
            println!("  Properties: 'id_suffix', 'name_suffix', 'icon_badge', 'env_file'.");
        }
        _ => {
            println!(
                "  Key '{}' is recognized under Ferrix Schema Version 1.",
                key
            );
            println!("  Refer to 'docs/config.md' for comprehensive reference.");
        }
    }
    Ok(())
}
