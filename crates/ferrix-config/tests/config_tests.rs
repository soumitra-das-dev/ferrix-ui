//! Unit and integration tests for ferrix-config (F5).

use ferrix_config::env::{filter_public_env, load_env_layers, parse_env_str};
use ferrix_config::prelude::*;
use tempfile::tempdir;

#[test]
fn test_valid_config_parsing() {
    let toml = r##"
        [app]
        name = "Ferrix Demo"
        slug = "ferrix-demo"
        version = "1.2.0"
        package_name = "dev.ferrix.demo"
        scheme = "ferrixdemo"

        [android]
        min_sdk = 26
        target_sdk = 34
        permissions = ["android.permission.CAMERA"]

        [icons]
        icon = "assets/icon.png"

        [splash]
        background_color = "#000000"
    "##;

    let config = FerrixConfig::parse(toml).expect("Valid config should parse");
    assert_eq!(config.app.name, "Ferrix Demo");
    assert_eq!(config.app.package_name, "dev.ferrix.demo");
    assert_eq!(config.android.min_sdk, 26);
    assert_eq!(config.android.target_sdk, 34);
    assert_eq!(
        config.android.permissions,
        vec!["android.permission.CAMERA"]
    );
}

#[test]
fn test_invalid_package_name_rejected() {
    let toml = r#"
        [app]
        name = "App"
        slug = "app"
        version = "1.0.0"
        package_name = "invalid"
    "#;

    let err = FerrixConfig::parse(toml).unwrap_err();
    assert_eq!(err, ConfigError::InvalidPackageName("invalid".to_string()));
}

#[test]
fn test_invalid_sdk_range_rejected() {
    let toml = r#"
        [app]
        name = "App"
        slug = "app"
        version = "1.0.0"
        package_name = "com.test.app"

        [android]
        min_sdk = 35
        target_sdk = 34
    "#;

    let err = FerrixConfig::parse(toml).unwrap_err();
    assert_eq!(
        err,
        ConfigError::InvalidSdkRange {
            min: 35,
            target: 34
        }
    );
}

#[test]
fn test_env_parsing_and_public_filtering() {
    let env_content = r##"
        # Server secrets
        DATABASE_URL=postgres://user:pass@localhost:5432/db
        STRIPE_SECRET_KEY="sk_test_12345"

        # Client public variables
        FERRIX_PUBLIC_API_URL=https://api.ferrix.dev
        EXPO_PUBLIC_ANALYTICS_KEY='analytics_abc'
    "##;

    let parsed = parse_env_str(env_content);
    assert_eq!(
        parsed.get("DATABASE_URL").map(|s| s.as_str()),
        Some("postgres://user:pass@localhost:5432/db")
    );
    assert_eq!(
        parsed.get("STRIPE_SECRET_KEY").map(|s| s.as_str()),
        Some("sk_test_12345")
    );
    assert_eq!(
        parsed.get("FERRIX_PUBLIC_API_URL").map(|s| s.as_str()),
        Some("https://api.ferrix.dev")
    );

    let public_vars = filter_public_env(&parsed);
    assert_eq!(public_vars.len(), 2);
    assert!(public_vars.contains_key("FERRIX_PUBLIC_API_URL"));
    assert!(public_vars.contains_key("EXPO_PUBLIC_ANALYTICS_KEY"));
    assert!(!public_vars.contains_key("DATABASE_URL"));
    assert!(!public_vars.contains_key("STRIPE_SECRET_KEY"));
}

#[test]
fn test_env_layering_precedence() {
    let dir = tempdir().unwrap();
    let base = dir.path();

    std::fs::write(base.join(".env"), "FOO=base\nBAR=base\n").unwrap();
    std::fs::write(base.join(".env.production"), "FOO=prod\nBAZ=prod\n").unwrap();
    std::fs::write(base.join(".env.production.local"), "FOO=prod_local\n").unwrap();

    let merged = load_env_layers(base, Some("production"));
    assert_eq!(merged.get("FOO").map(|s| s.as_str()), Some("prod_local"));
    assert_eq!(merged.get("BAR").map(|s| s.as_str()), Some("base"));
    assert_eq!(merged.get("BAZ").map(|s| s.as_str()), Some("prod"));
}
