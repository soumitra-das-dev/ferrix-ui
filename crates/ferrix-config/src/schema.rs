//! Formal schema definitions and validation for ferrix.config.toml (Schema Version 1).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Reserved keywords in Java and Kotlin that cannot be used as Android package/applicationId segments.
pub const RESERVED_PACKAGE_KEYWORDS: &[&str] = &[
    "abstract",
    "assert",
    "boolean",
    "break",
    "byte",
    "case",
    "catch",
    "char",
    "class",
    "const",
    "continue",
    "default",
    "do",
    "double",
    "else",
    "enum",
    "extends",
    "final",
    "finally",
    "float",
    "for",
    "goto",
    "if",
    "implements",
    "import",
    "instanceof",
    "int",
    "interface",
    "long",
    "native",
    "new",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "short",
    "static",
    "strictfp",
    "super",
    "switch",
    "synchronized",
    "this",
    "throw",
    "throws",
    "transient",
    "try",
    "void",
    "volatile",
    "while",
    "val",
    "var",
    "fun",
    "when",
    "is",
    "in",
    "null",
    "true",
    "false",
    "object",
    "companion",
    "constructor",
    "init",
    "typealias",
    "typeof",
];

#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    #[error("Failed to parse TOML config: {0}")]
    ParseError(String),

    #[error("Invalid schema version {0}: expected 1. Run 'ferrix upgrade' to migrate.")]
    InvalidSchemaVersion(u32),

    #[error(
        "Invalid application ID '{0}': contains hyphens. Android package IDs cannot contain hyphens. Did you mean '{1}'?"
    )]
    PackageNameContainsHyphen(String, String),

    #[error(
        "Invalid application ID '{0}': must contain at least two dot-separated segments (e.g. 'com.example.app')"
    )]
    PackageNameTooFewSegments(String),
    #[error(
        "Invalid package name '{0}': must contain at least two segments (e.g. 'com.example.app')"
    )]
    InvalidPackageName(String),

    #[error(
        "Invalid application ID '{0}': segment '{1}' is a reserved Java/Kotlin keyword and cannot be used"
    )]
    PackageNameReservedKeyword(String, String),

    #[error(
        "Invalid application ID '{0}': segment '{1}' must start with an ASCII letter and contain only alphanumeric characters or underscores"
    )]
    PackageNameInvalidSegment(String, String),

    #[error("Invalid version '{0}': must follow semver 'major.minor.patch' (e.g. '1.4.2')")]
    InvalidVersion(String),

    #[error("Invalid Android SDK version: min_sdk ({min}) cannot exceed target_sdk ({target})")]
    InvalidSdkRange { min: u32, target: u32 },

    #[error("Invalid Android compile SDK version: target_sdk ({target}) cannot exceed compile_sdk ({compile})")]
    InvalidCompileSdk { target: u32, compile: u32 },

    #[error("Invalid URI scheme '{0}': must start with a lowercase letter and contain only alphanumeric chars, hyphens, or '+'")]
    InvalidScheme(String),

    #[error("Invalid color format '{0}': must be #RGB, #RRGGBB, or #AARRGGBB")]
    InvalidColor(String),

    #[error(
        "Invalid orientation '{0}': expected 'portrait', 'landscape', 'default', or 'per-screen'"
    )]
    InvalidOrientation(String),

    #[error("Invalid deep link host '{0}': host cannot be empty and must be a valid domain or IP")]
    InvalidDeepLinkHost(String),

    #[error("Invalid deep link path '{0}': must start with '/' (e.g. '/product/*')")]
    InvalidDeepLinkPath(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub package_name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_orientation")]
    pub orientation: String,
    #[serde(default = "default_edge_to_edge")]
    pub edge_to_edge: bool,
    #[serde(default)]
    pub scheme: Option<String>,
}

fn default_orientation() -> String {
    "portrait".to_string()
}
fn default_edge_to_edge() -> bool {
    true
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "Ferrix App".to_string(),
            slug: "ferrix-app".to_string(),
            id: "com.example.app".to_string(),
            package_name: "com.example.app".to_string(),
            version: "0.1.0".to_string(),
            description: None,
            orientation: default_orientation(),
            edge_to_edge: default_edge_to_edge(),
            scheme: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidFeature {
    pub name: String,
    #[serde(default)]
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidConfig {
    #[serde(default = "default_min_sdk")]
    pub min_sdk: u32,
    #[serde(default = "default_target_sdk")]
    pub target_sdk: u32,
    #[serde(default = "default_compile_sdk")]
    pub compile_sdk: u32,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub features: Vec<AndroidFeature>,
    #[serde(default)]
    pub allow_backup: bool,
    #[serde(default)]
    pub cleartext_domains: Vec<String>,
}

fn default_min_sdk() -> u32 {
    26
}
fn default_target_sdk() -> u32 {
    35
}
fn default_compile_sdk() -> u32 {
    35
}

impl Default for AndroidConfig {
    fn default() -> Self {
        Self {
            min_sdk: default_min_sdk(),
            target_sdk: default_target_sdk(),
            compile_sdk: default_compile_sdk(),
            permissions: vec!["INTERNET".to_string()],
            features: Vec::new(),
            allow_backup: false,
            cleartext_domains: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IconConfig {
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub adaptive_foreground: Option<String>,
    #[serde(default = "default_adaptive_bg")]
    pub adaptive_background: String,
    #[serde(default)]
    pub monochrome: Option<String>,
    #[serde(default = "default_true")]
    pub round: bool,
}

fn default_adaptive_bg() -> String {
    "#0B1F3A".to_string()
}
fn default_true() -> bool {
    true
}

impl Default for IconConfig {
    fn default() -> Self {
        Self {
            source: None,
            adaptive_foreground: None,
            adaptive_background: default_adaptive_bg(),
            monochrome: None,
            round: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplashConfig {
    #[serde(default = "default_splash_bg")]
    pub background: String,
    #[serde(default = "default_splash_dark_bg")]
    pub dark_background: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub branding: Option<String>,
    #[serde(default = "default_true")]
    pub keep_until_ready: bool,
    #[serde(default = "default_exit_anim")]
    pub exit_animation: String,
}

fn default_splash_bg() -> String {
    "#0B1F3A".to_string()
}
fn default_splash_dark_bg() -> String {
    "#000000".to_string()
}
fn default_exit_anim() -> String {
    "fade".to_string()
}

impl Default for SplashConfig {
    fn default() -> Self {
        Self {
            background: default_splash_bg(),
            dark_background: default_splash_dark_bg(),
            icon: None,
            branding: None,
            keep_until_ready: true,
            exit_animation: default_exit_anim(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeepLinkPattern {
    Exact(String),
    Structured {
        #[serde(default)]
        prefix: Option<String>,
        #[serde(default)]
        pattern: Option<String>,
        #[serde(default)]
        path: Option<String>,
    },
}

impl DeepLinkPattern {
    pub fn to_attribute(&self) -> (&'static str, String) {
        match self {
            DeepLinkPattern::Exact(s) => {
                if s.contains('*') {
                    ("android:pathPattern", s.clone())
                } else {
                    ("android:path", s.clone())
                }
            }
            DeepLinkPattern::Structured {
                prefix,
                pattern,
                path,
            } => {
                if let Some(pfx) = prefix {
                    ("android:pathPrefix", pfx.clone())
                } else if let Some(pat) = pattern {
                    ("android:pathPattern", pat.clone())
                } else if let Some(pth) = path {
                    ("android:path", pth.clone())
                } else {
                    ("android:path", "/".to_string())
                }
            }
        }
    }

    pub fn value(&self) -> String {
        self.to_attribute().1
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepLinkHost {
    pub host: String,
    #[serde(default)]
    pub auto_verify: bool,
    #[serde(default)]
    pub paths: Vec<DeepLinkPattern>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeepLinksConfig {
    #[serde(default)]
    pub schemes: Vec<String>,
    #[serde(default)]
    pub hosts: Vec<DeepLinkHost>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VariantConfig {
    #[serde(default)]
    pub id_suffix: Option<String>,
    #[serde(default)]
    pub name_suffix: Option<String>,
    #[serde(default)]
    pub icon_badge: Option<String>,
    #[serde(default)]
    pub env_file: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default)]
    pub variants: HashMap<String, VariantConfig>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocaleConfig {
    #[serde(default = "default_locale_list")]
    pub supported: Vec<String>,
    #[serde(default = "default_locale_en")]
    pub default: String,
}

fn default_locale_list() -> Vec<String> {
    vec!["en".to_string()]
}
fn default_locale_en() -> String {
    "en".to_string()
}

impl Default for LocaleConfig {
    fn default() -> Self {
        Self {
            supported: default_locale_list(),
            default: default_locale_en(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SigningConfig {
    pub keystore_env: Option<String>,
    pub alias_env: Option<String>,
}

// Backwards-compatible aliases
pub type AppSection = AppConfig;
pub type AndroidSection = AndroidConfig;
pub type IconSection = IconConfig;
pub type SplashSection = SplashConfig;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FerrixConfig {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    pub app: AppConfig,
    #[serde(default)]
    pub android: AndroidConfig,
    #[serde(default, alias = "icons")]
    pub icon: IconConfig,
    #[serde(default)]
    pub splash: SplashConfig,
    #[serde(default)]
    pub deep_links: DeepLinksConfig,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub locale: LocaleConfig,
    #[serde(default)]
    pub signing: SigningConfig,
    #[serde(default)]
    pub plugins: HashMap<String, serde_json::Value>,
}

fn default_schema_version() -> u32 {
    1
}

impl FerrixConfig {
    /// Parses and strictly validates a `ferrix.config.toml` file.
    pub fn parse(toml_str: &str) -> Result<Self, ConfigError> {
        let mut config: FerrixConfig =
            toml::from_str(toml_str).map_err(|e| ConfigError::ParseError(e.to_string()))?;
        if config.app.id.is_empty() && !config.app.package_name.is_empty() {
            config.app.id = config.app.package_name.clone();
        } else if config.app.package_name.is_empty() && !config.app.id.is_empty() {
            config.app.package_name = config.app.id.clone();
        }
        config.validate()?;
        Ok(config)
    }

    /// Performs strict schema, syntax, and Android ecosystem validation.
    pub fn validate(&self) -> Result<(), ConfigError> {
        // 1. Schema version
        if self.schema_version != 1 {
            return Err(ConfigError::InvalidSchemaVersion(self.schema_version));
        }

        // 2. Application ID validation
        Self::validate_application_id(&self.app.id)?;

        // 3. Semver format validation
        Self::validate_semver(&self.app.version)?;

        // 4. Android SDK ranges
        if self.android.min_sdk > self.android.target_sdk {
            return Err(ConfigError::InvalidSdkRange {
                min: self.android.min_sdk,
                target: self.android.target_sdk,
            });
        }
        if self.android.target_sdk > self.android.compile_sdk {
            return Err(ConfigError::InvalidCompileSdk {
                target: self.android.target_sdk,
                compile: self.android.compile_sdk,
            });
        }

        // 5. Orientation validation
        match self.app.orientation.as_str() {
            "portrait" | "landscape" | "default" | "per-screen" => {}
            other => return Err(ConfigError::InvalidOrientation(other.to_string())),
        }

        // 6. Color validation
        Self::validate_color(&self.icon.adaptive_background)?;
        Self::validate_color(&self.splash.background)?;
        Self::validate_color(&self.splash.dark_background)?;

        // 7. Deep link schemes and hosts validation
        for scheme in &self.deep_links.schemes {
            if scheme.is_empty()
                || !scheme.starts_with(|c: char| c.is_ascii_lowercase())
                || !scheme
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '+')
            {
                return Err(ConfigError::InvalidScheme(scheme.clone()));
            }
        }

        for host_cfg in &self.deep_links.hosts {
            if host_cfg.host.is_empty() || host_cfg.host.contains(' ') {
                return Err(ConfigError::InvalidDeepLinkHost(host_cfg.host.clone()));
            }
            for path in &host_cfg.paths {
                let p = path.value();
                if !p.starts_with('/') {
                    return Err(ConfigError::InvalidDeepLinkPath(p));
                }
            }
        }

        // 8. Variants validation
        for (name, variant) in &self.build.variants {
            if let Some(suffix) = &variant.id_suffix {
                let resulting_id = format!("{}{}", self.app.id, suffix);
                Self::validate_application_id(&resulting_id).map_err(|_| {
                    ConfigError::PackageNameInvalidSegment(
                        resulting_id.clone(),
                        format!("variant '{}' resulting ID is invalid", name),
                    )
                })?;
            }
        }

        Ok(())
    }

    /// Validates Android Application ID according to strict Play Store and Gradle conventions:
    /// - At least two dot-separated segments
    /// - Each segment starts with an ASCII letter and contains only alphanumeric characters or underscores
    /// - No hyphens allowed (suggests replacing hyphens with underscores)
    /// - No Java/Kotlin reserved keywords as segments
    pub fn validate_application_id(id: &str) -> Result<(), ConfigError> {
        if id.contains('-') {
            let suggested = id.replace('-', "_");
            return Err(ConfigError::PackageNameContainsHyphen(
                id.to_string(),
                suggested,
            ));
        }

        let segments: Vec<&str> = id.split('.').collect();
        if segments.len() < 2 {
            return Err(ConfigError::PackageNameTooFewSegments(id.to_string()));
        }

        for segment in segments {
            if segment.is_empty() {
                return Err(ConfigError::PackageNameInvalidSegment(
                    id.to_string(),
                    "empty segment".to_string(),
                ));
            }

            if RESERVED_PACKAGE_KEYWORDS.contains(&segment) {
                return Err(ConfigError::PackageNameReservedKeyword(
                    id.to_string(),
                    segment.to_string(),
                ));
            }

            let first_char = segment.chars().next().unwrap();
            if !first_char.is_ascii_alphabetic() {
                return Err(ConfigError::PackageNameInvalidSegment(
                    id.to_string(),
                    segment.to_string(),
                ));
            }

            if !segment
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
            {
                return Err(ConfigError::PackageNameInvalidSegment(
                    id.to_string(),
                    segment.to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Derives the monotonic integer `versionCode` from semver string:
    /// `major * 1_000_000 + minor * 1_000 + patch`.
    pub fn derive_version_code(version: &str) -> Result<u32, ConfigError> {
        let (major, minor, patch) = Self::parse_semver(version)?;
        let code = major * 1_000_000 + minor * 1_000 + patch;
        Ok(code)
    }

    fn parse_semver(version: &str) -> Result<(u32, u32, u32), ConfigError> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() < 2 || parts.len() > 3 {
            return Err(ConfigError::InvalidVersion(version.to_string()));
        }

        let major = parts[0]
            .parse::<u32>()
            .map_err(|_| ConfigError::InvalidVersion(version.to_string()))?;
        let minor = parts[1]
            .parse::<u32>()
            .map_err(|_| ConfigError::InvalidVersion(version.to_string()))?;
        let patch = if parts.len() == 3 {
            parts[2]
                .parse::<u32>()
                .map_err(|_| ConfigError::InvalidVersion(version.to_string()))?
        } else {
            0
        };

        Ok((major, minor, patch))
    }

    pub fn validate_semver(version: &str) -> Result<(), ConfigError> {
        Self::parse_semver(version).map(|_| ())
    }

    pub fn validate_color(color: &str) -> Result<(), ConfigError> {
        if color.is_empty() {
            return Ok(());
        }
        if !color.starts_with('#') {
            return Ok(()); // Treat as asset path if not hex color
        }
        let hex = &color[1..];
        if (hex.len() == 3 || hex.len() == 6 || hex.len() == 8)
            && hex.chars().all(|c| c.is_ascii_hexdigit())
        {
            Ok(())
        } else {
            Err(ConfigError::InvalidColor(color.to_string()))
        }
    }

    /// Generates the standard JSON Schema document for `ferrix.config.toml`.
    pub fn json_schema() -> serde_json::Value {
        serde_json::json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": "FerrixAppConfiguration",
            "type": "object",
            "required": ["app"],
            "properties": {
                "schema_version": { "type": "integer", "default": 1 },
                "app": {
                    "type": "object",
                    "required": ["name", "slug", "id", "version"],
                    "properties": {
                        "name": { "type": "string", "description": "Launcher application name" },
                        "slug": { "type": "string", "description": "Filesystem and URL-safe identifier" },
                        "id": { "type": "string", "description": "Android application ID (e.g. com.example.app)" },
                        "version": { "type": "string", "description": "Semver version string (e.g. 1.0.0)" },
                        "description": { "type": "string" },
                        "orientation": { "type": "string", "enum": ["portrait", "landscape", "default", "per-screen"] },
                        "edge_to_edge": { "type": "boolean", "default": true }
                    }
                },
                "android": {
                    "type": "object",
                    "properties": {
                        "min_sdk": { "type": "integer", "default": 26 },
                        "target_sdk": { "type": "integer", "default": 35 },
                        "compile_sdk": { "type": "integer", "default": 35 },
                        "permissions": { "type": "array", "items": { "type": "string" } },
                        "allow_backup": { "type": "boolean", "default": false },
                        "cleartext_domains": { "type": "array", "items": { "type": "string" } }
                    }
                },
                "icon": {
                    "type": "object",
                    "properties": {
                        "source": { "type": "string" },
                        "adaptive_foreground": { "type": "string" },
                        "adaptive_background": { "type": "string" },
                        "monochrome": { "type": "string" },
                        "round": { "type": "boolean", "default": true }
                    }
                },
                "splash": {
                    "type": "object",
                    "properties": {
                        "background": { "type": "string", "default": "#0B1F3A" },
                        "dark_background": { "type": "string", "default": "#000000" },
                        "icon": { "type": "string" },
                        "branding": { "type": "string" },
                        "keep_until_ready": { "type": "boolean", "default": true },
                        "exit_animation": { "type": "string", "enum": ["none", "fade", "slide"] }
                    }
                },
                "deep_links": {
                    "type": "object",
                    "properties": {
                        "schemes": { "type": "array", "items": { "type": "string" } },
                        "hosts": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "required": ["host"],
                                "properties": {
                                    "host": { "type": "string" },
                                    "auto_verify": { "type": "boolean" },
                                    "paths": { "type": "array", "items": { "type": "string" } }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_comprehensive_config() {
        let toml_str = r##"
schema_version = 1

[app]
name = "My App"
slug = "my-app"
id = "com.name.sub_name"
version = "1.4.2"
description = "A production mobile app"
orientation = "portrait"
edge_to_edge = true

[android]
min_sdk = 26
target_sdk = 35
compile_sdk = 35
permissions = ["INTERNET", "CAMERA"]
features = [{ name = "android.hardware.camera", required = false }]
allow_backup = false
cleartext_domains = []

[icon]
source = "assets/icon.png"
adaptive_foreground = "assets/icon-fg.png"
adaptive_background = "#0B1F3A"
monochrome = "assets/icon-mono.png"
round = true

[splash]
background = "#0B1F3A"
dark_background = "#000000"
icon = "assets/splash-icon.png"
branding = "assets/branding.png"
keep_until_ready = true
exit_animation = "fade"

[deep_links]
schemes = ["myapp"]

[[deep_links.hosts]]
host = "example.com"
auto_verify = true
paths = ["/product/*", { prefix = "/u/" }]

[build.variants.dev]
id_suffix = ".dev"
name_suffix = " (Dev)"
icon_badge = "DEV"
env_file = ".env.dev"

[build.variants.prod]
env_file = ".env.prod"

[locale]
supported = ["en", "bn", "hi"]
default = "en"

[signing]
keystore_env = "FERRIX_KEYSTORE"
alias_env = "FERRIX_KEY_ALIAS"
"##;

        let config = FerrixConfig::parse(toml_str).expect("Valid config should parse cleanly");
        assert_eq!(config.app.id, "com.name.sub_name");
        assert_eq!(config.android.target_sdk, 35);
        assert_eq!(config.deep_links.hosts.len(), 1);
        assert_eq!(config.deep_links.hosts[0].paths.len(), 2);
    }

    #[test]
    fn test_reject_application_id_with_hyphen() {
        let err = FerrixConfig::validate_application_id("com.name.sub-name").unwrap_err();
        match err {
            ConfigError::PackageNameContainsHyphen(bad, suggested) => {
                assert_eq!(bad, "com.name.sub-name");
                assert_eq!(suggested, "com.name.sub_name");
            }
            _ => panic!("Expected PackageNameContainsHyphen, got {:?}", err),
        }
    }

    #[test]
    fn test_reject_reserved_keyword_in_application_id() {
        let err = FerrixConfig::validate_application_id("com.example.class").unwrap_err();
        match err {
            ConfigError::PackageNameReservedKeyword(bad, kw) => {
                assert_eq!(bad, "com.example.class");
                assert_eq!(kw, "class");
            }
            _ => panic!("Expected PackageNameReservedKeyword, got {:?}", err),
        }
    }

    #[test]
    fn test_reject_single_segment_application_id() {
        let err = FerrixConfig::validate_application_id("myapp").unwrap_err();
        assert_eq!(
            err,
            ConfigError::PackageNameTooFewSegments("myapp".to_string())
        );
    }

    #[test]
    fn test_version_code_derivation() {
        assert_eq!(
            FerrixConfig::derive_version_code("1.0.0").unwrap(),
            1_000_000
        );
        assert_eq!(
            FerrixConfig::derive_version_code("1.4.2").unwrap(),
            1_004_002
        );
        assert_eq!(
            FerrixConfig::derive_version_code("2.10.5").unwrap(),
            2_010_005
        );
    }

    #[test]
    fn test_color_validation() {
        assert!(FerrixConfig::validate_color("#FFF").is_ok());
        assert!(FerrixConfig::validate_color("#0B1F3A").is_ok());
        assert!(FerrixConfig::validate_color("#FF0B1F3A").is_ok());
        assert!(FerrixConfig::validate_color("assets/bg.png").is_ok());
        assert!(FerrixConfig::validate_color("#INVALID").is_err());
    }

    #[test]
    fn test_json_schema_generation() {
        let schema = FerrixConfig::json_schema();
        assert_eq!(schema["title"], "FerrixAppConfiguration");
        assert!(schema["properties"]["app"].is_object());
    }
}
