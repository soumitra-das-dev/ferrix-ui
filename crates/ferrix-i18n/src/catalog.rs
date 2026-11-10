//! Multi-locale translation catalog and string resolution.

use crate::icu::format_message;
use std::collections::HashMap;

/// Translation repository storing localized message templates across multiple locales.
#[derive(Clone, Debug)]
pub struct I18nCatalog {
    locales: HashMap<String, HashMap<String, String>>,
    pub default_locale: String,
}

impl Default for I18nCatalog {
    fn default() -> Self {
        Self::new("en")
    }
}

impl I18nCatalog {
    /// Creates a new catalog with specified fallback locale.
    pub fn new(default_locale: &str) -> Self {
        Self {
            locales: HashMap::new(),
            default_locale: default_locale.to_string(),
        }
    }

    /// Adds a dictionary of string translations for a given locale.
    pub fn add_locale(&mut self, locale: &str, strings: HashMap<String, String>) {
        self.locales.insert(locale.to_lowercase(), strings);
    }

    /// Resolves and formats a key for the given locale with fallback to default_locale.
    pub fn translate(&self, locale: &str, key: &str, params: &HashMap<String, String>) -> String {
        let loc = locale.to_lowercase();
        let pattern = self
            .locales
            .get(&loc)
            .and_then(|dict| dict.get(key))
            .or_else(|| {
                self.locales
                    .get(&self.default_locale)
                    .and_then(|dict| dict.get(key))
            });

        match pattern {
            Some(pat) => format_message(pat, params),
            None => key.to_string(),
        }
    }
}
