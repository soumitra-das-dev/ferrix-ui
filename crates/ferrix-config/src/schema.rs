//! Formal schema definitions and validation for ferrix.config.toml.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    #[error("Failed to parse TOML config: {0}")]
    ParseError(String),
    #[error(
        "Invalid package name '{0}': must contain at least two segments (e.g. 'com.example.app')"
    )]
    InvalidPackageName(String),
    #[error("Invalid version '{0}': must follow semver (e.g. '1.0.0')")]
    InvalidVersion(String),
    #[error("Invalid Android SDK version: min_sdk ({min}) cannot exceed target_sdk ({target})")]
    InvalidSdkRange { min: u32, target: u32 },
    #[error("Invalid URI scheme '{0}': must start with a lowercase letter and contain only alphanumeric chars or dashes")]
    InvalidScheme(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSection {
    pub name: String,
    pub slug: String,
    pub version: String,
    pub package_name: String,
    pub scheme: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidSection {
    #[serde(default = "default_min_sdk")]
    pub min_sdk: u32,
    #[serde(default = "default_target_sdk")]
    pub target_sdk: u32,
    #[serde(default = "default_version_code")]
    pub version_code: u32,
    #[serde(default)]
    pub permissions: Vec<String>,
}

fn default_min_sdk() -> u32 {
    26
}
fn default_target_sdk() -> u32 {
    34
}
fn default_version_code() -> u32 {
    1
}

impl Default for AndroidSection {
    fn default() -> Self {
        Self {
            min_sdk: default_min_sdk(),
            target_sdk: default_target_sdk(),
            version_code: default_version_code(),
            permissions: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IconSection {
    pub icon: Option<String>,
    pub adaptive_foreground: Option<String>,
    pub adaptive_background: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SplashSection {
    pub image: Option<String>,
    pub background_color: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FerrixConfig {
    pub app: AppSection,
    #[serde(default)]
    pub android: AndroidSection,
    #[serde(default)]
    pub icons: IconSection,
    #[serde(default)]
    pub splash: SplashSection,
}

impl FerrixConfig {
    /// Parses and validates a TOML configuration string.
    pub fn parse(toml_str: &str) -> Result<Self, ConfigError> {
        let config: FerrixConfig =
            toml::from_str(toml_str).map_err(|e| ConfigError::ParseError(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// Performs strict schema validation.
    pub fn validate(&self) -> Result<(), ConfigError> {
        // 1. Validate package name
        let parts: Vec<&str> = self.app.package_name.split('.').collect();
        if parts.len() < 2
            || parts
                .iter()
                .any(|p| p.is_empty() || !p.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
        {
            return Err(ConfigError::InvalidPackageName(
                self.app.package_name.clone(),
            ));
        }

        // 2. Validate version format
        let ver_parts: Vec<&str> = self.app.version.split('.').collect();
        if ver_parts.len() < 2 || ver_parts.iter().any(|p| p.parse::<u32>().is_err()) {
            return Err(ConfigError::InvalidVersion(self.app.version.clone()));
        }

        // 3. Validate Android SDK range
        if self.android.min_sdk > self.android.target_sdk {
            return Err(ConfigError::InvalidSdkRange {
                min: self.android.min_sdk,
                target: self.android.target_sdk,
            });
        }

        // 4. Validate URI scheme if specified
        if let Some(scheme) = &self.app.scheme {
            if scheme.is_empty()
                || !scheme.starts_with(|c: char| c.is_ascii_lowercase())
                || !scheme
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '+')
            {
                return Err(ConfigError::InvalidScheme(scheme.clone()));
            }
        }

        Ok(())
    }
}
