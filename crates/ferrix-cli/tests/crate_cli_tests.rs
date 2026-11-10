use ferrix_cli::commands::crate_cmd::{check_pitfall, ensure_deny_toml_in, parse_dependencies};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_check_pitfall_openssl() {
    let warning = check_pitfall("openssl");
    assert!(warning.is_some());
    assert!(warning.unwrap().contains("rustls"));
}

#[test]
fn test_check_pitfall_safe_crate() {
    let warning = check_pitfall("uuid");
    assert!(warning.is_none());
}

#[test]
fn test_parse_dependencies() {
    let toml_sample = r#"
[package]
name = "my-app"
version = "0.1.0"

[dependencies]
uuid = "1.10"
serde = { version = "1.0", features = ["derive"] }
regex = "1.10"
"#;
    let deps = parse_dependencies(toml_sample).expect("parse dependencies");
    assert_eq!(deps.len(), 3);
    let names: Vec<String> = deps.into_iter().map(|(n, _)| n).collect();
    assert!(names.contains(&"uuid".to_string()));
    assert!(names.contains(&"serde".to_string()));
    assert!(names.contains(&"regex".to_string()));
}

#[test]
fn test_ensure_deny_toml_created() {
    let temp = tempdir().expect("tempdir");
    let deny_path = temp.path().join("deny.toml");
    assert!(!deny_path.exists());

    ensure_deny_toml_in(temp.path()).expect("ensure deny toml");
    assert!(deny_path.exists());

    let content = fs::read_to_string(&deny_path).unwrap();
    assert!(content.contains("[licenses]"));
    assert!(content.contains("MIT"));
}
