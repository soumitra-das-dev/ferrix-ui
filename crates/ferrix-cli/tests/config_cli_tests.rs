use ferrix_cli::args::{
    AssetlinksArgs, ConfigAction, ConfigArgs, ConfigExplainArgs, ConfigPrintArgs, DeeplinksAction,
    DeeplinksArgs,
};
use ferrix_cli::commands::{config, deeplinks};
use ferrix_config::FerrixConfig;
use std::fs;
use tempfile::tempdir;

const VALID_TOML: &str = r##"
schema_version = 1

[app]
name = "Test App"
slug = "test-app"
id = "com.company.test_app"
version = "1.0.0"
orientation = "portrait"

[android]
target_sdk = 35

[build.variants.dev]
id_suffix = ".dev"
name_suffix = " (Dev)"
"##;

const INVALID_HYPHEN_TOML: &str = r##"
schema_version = 1

[app]
name = "Test App"
slug = "test-app"
id = "com.company.test-app"
version = "1.0.0"
"##;

#[test]
fn test_config_check_valid() {
    let temp = tempdir().expect("tempdir");
    let config_path = temp.path().join("ferrix.config.toml");
    fs::write(&config_path, VALID_TOML).unwrap();

    let args = ConfigArgs {
        action: ConfigAction::Check,
    };
    let res = config::execute_at(&args, &config_path);
    assert!(res.is_ok(), "Config check should succeed for valid config");
}

#[test]
fn test_config_check_rejects_hyphens_with_suggestion() {
    let temp = tempdir().expect("tempdir");
    let config_path = temp.path().join("ferrix.config.toml");
    fs::write(&config_path, INVALID_HYPHEN_TOML).unwrap();

    let args = ConfigArgs {
        action: ConfigAction::Check,
    };
    let res = config::execute_at(&args, &config_path);

    let err = res.expect_err("Should fail for hyphen in id");
    assert!(err.contains("contains hyphens"));
    assert!(err.contains("com.company.test_app"));
}

#[test]
fn test_config_print_and_explain() {
    let temp = tempdir().expect("tempdir");
    let config_path = temp.path().join("ferrix.config.toml");
    fs::write(&config_path, VALID_TOML).unwrap();

    // Print text
    let print_args = ConfigArgs {
        action: ConfigAction::Print(ConfigPrintArgs {
            variant: Some("dev".to_string()),
            json: false,
        }),
    };
    assert!(config::execute_at(&print_args, &config_path).is_ok());

    // Print json
    let print_json_args = ConfigArgs {
        action: ConfigAction::Print(ConfigPrintArgs {
            variant: None,
            json: true,
        }),
    };
    assert!(config::execute_at(&print_json_args, &config_path).is_ok());

    // Explain
    let explain_args = ConfigArgs {
        action: ConfigAction::Explain(ConfigExplainArgs {
            key: "app.id".to_string(),
        }),
    };
    assert!(config::execute_at(&explain_args, &config_path).is_ok());
}

#[test]
fn test_deeplinks_assetlinks() {
    let temp = tempdir().expect("tempdir");
    let config_path = temp.path().join("ferrix.config.toml");
    fs::write(&config_path, VALID_TOML).unwrap();

    let asset_args = DeeplinksArgs {
        action: DeeplinksAction::Assetlinks(AssetlinksArgs {
            fingerprints: vec!["AA:BB:CC:DD:...".to_string()],
        }),
    };
    assert!(deeplinks::execute_at(&asset_args, &config_path).is_ok());
}

#[test]
fn test_fixed_host_namespace_regression_with_underscores() {
    let config_str = r##"
schema_version = 1

[app]
name = "Underscore App"
slug = "underscore-app"
id = "com.example.sub_name_test.app_module"
version = "2.1.0"

[android]
target_sdk = 35
permissions = ["INTERNET"]
"##;
    let config = FerrixConfig::parse(config_str).expect("Should parse");
    let manifest = config.generate_android_manifest(None);

    // MainActivity namespace must remain dev.ferrix.host.MainActivity
    assert!(manifest.contains("android:name=\"dev.ferrix.host.MainActivity\""));
    assert_eq!(config.app.id, "com.example.sub_name_test.app_module");
}
