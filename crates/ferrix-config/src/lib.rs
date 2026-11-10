//! # ferrix-config
//!
//! Build-time configuration validation, schema checking, and environment injection for Ferrix.

pub mod env;
pub mod generator;
pub mod schema;

pub use env::{filter_public_env, load_env_layers, parse_env_str};
pub use generator::GENERATED_FILE_HEADER;
pub use schema::{
    AndroidSection, AppSection, ConfigError, FerrixConfig, IconSection, SplashSection,
};

pub mod prelude {
    pub use crate::env::{filter_public_env, load_env_layers};
    pub use crate::generator::GENERATED_FILE_HEADER;
    pub use crate::schema::{ConfigError, FerrixConfig};
}
