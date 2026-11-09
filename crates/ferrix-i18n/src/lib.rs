//! # ferrix-i18n
//!
//! Internationalization runtime for Ferrix with ICU message formatting and reactive locale switching.

pub mod catalog;
pub mod hooks;
pub mod icu;

pub use catalog::I18nCatalog;
pub use hooks::{set_global_catalog, use_translation};
pub use icu::format_message;

pub mod prelude {
    pub use crate::catalog::I18nCatalog;
    pub use crate::hooks::use_translation;
}
