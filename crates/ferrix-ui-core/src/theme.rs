//! Theming and internationalization (i18n / RTL) system for Ferrix.

use std::sync::atomic::{AtomicBool, Ordering};

/// Global RTL state managed by I18nManager.
static IS_RTL: AtomicBool = AtomicBool::new(false);
static ALLOW_RTL: AtomicBool = AtomicBool::new(true);

/// Manager for internationalization and layout direction mirroring React Native `I18nManager`.
pub struct I18nManager;

impl I18nManager {
    /// Returns true if the layout direction is currently right-to-left.
    pub fn is_rtl() -> bool {
        IS_RTL.load(Ordering::Relaxed)
    }

    /// Explicitly forces RTL layout direction (for testing or user locale preferences).
    pub fn force_rtl(force: bool) {
        IS_RTL.store(force, Ordering::Relaxed);
    }

    /// Allows or disallows RTL layout mirroring.
    pub fn allow_rtl(allow: bool) {
        ALLOW_RTL.store(allow, Ordering::Relaxed);
    }
}

/// Color scheme preference: Light or Dark.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorScheme {
    Light,
    Dark,
}

impl ColorScheme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

/// Material You / Dynamic system palette tokens.
pub struct DynamicColors;

impl DynamicColors {
    pub fn surface(dark: bool) -> u32 {
        if dark {
            0xFF121212
        } else {
            0xFFFFFFFF
        }
    }

    pub fn on_surface(dark: bool) -> u32 {
        if dark {
            0xFFE6E1E5
        } else {
            0xFF1C1B1F
        }
    }

    pub fn primary(dark: bool) -> u32 {
        if dark {
            0xFFD0BCFF
        } else {
            0xFF6750A4
        }
    }
}

/// Reactive hook returning current color scheme.
pub fn use_color_scheme() -> ColorScheme {
    // Default to Light scheme in testing/runtime until updated by platform configuration event
    ColorScheme::Light
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_manager_rtl_toggle() {
        assert_eq!(I18nManager::is_rtl(), false);
        I18nManager::force_rtl(true);
        assert_eq!(I18nManager::is_rtl(), true);
        I18nManager::force_rtl(false);
        assert_eq!(I18nManager::is_rtl(), false);
    }

    #[test]
    fn test_dynamic_colors() {
        assert_ne!(DynamicColors::surface(true), DynamicColors::surface(false));
    }
}
