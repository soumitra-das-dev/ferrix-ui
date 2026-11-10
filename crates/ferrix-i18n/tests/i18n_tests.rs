//! Unit and integration tests for ferrix-i18n (F5).

use ferrix_i18n::hooks::set_global_catalog;
use ferrix_i18n::prelude::*;
use std::collections::HashMap;

#[test]
fn test_catalog_translations_and_fallback() {
    let mut catalog = I18nCatalog::new("en");

    let mut en_dict = HashMap::new();
    en_dict.insert("welcome".to_string(), "Welcome, {name}!".to_string());
    en_dict.insert("fallback_key".to_string(), "English Only".to_string());
    catalog.add_locale("en", en_dict);

    let mut es_dict = HashMap::new();
    es_dict.insert("welcome".to_string(), "¡Bienvenido, {name}!".to_string());
    catalog.add_locale("es", es_dict);

    let mut params = HashMap::new();
    params.insert("name".to_string(), "Carlos".to_string());

    // Spanish translation
    assert_eq!(
        catalog.translate("es", "welcome", &params),
        "¡Bienvenido, Carlos!"
    );

    // English translation
    assert_eq!(
        catalog.translate("en", "welcome", &params),
        "Welcome, Carlos!"
    );

    // Fallback to English when Spanish missing
    assert_eq!(
        catalog.translate("es", "fallback_key", &HashMap::new()),
        "English Only"
    );

    // Missing key returns key literal
    assert_eq!(
        catalog.translate("es", "unknown_key", &HashMap::new()),
        "unknown_key"
    );
}

#[test]
fn test_icu_plurals_in_catalog() {
    let mut catalog = I18nCatalog::new("en");
    let mut dict = HashMap::new();
    dict.insert(
        "messages_count".to_string(),
        "{count, plural, zero {No new messages} one {1 new message} other {# new messages}}"
            .to_string(),
    );
    catalog.add_locale("en", dict);

    let mut p0 = HashMap::new();
    p0.insert("count".to_string(), "0".to_string());
    assert_eq!(
        catalog.translate("en", "messages_count", &p0),
        "No new messages"
    );

    let mut p1 = HashMap::new();
    p1.insert("count".to_string(), "1".to_string());
    assert_eq!(
        catalog.translate("en", "messages_count", &p1),
        "1 new message"
    );

    let mut p42 = HashMap::new();
    p42.insert("count".to_string(), "42".to_string());
    assert_eq!(
        catalog.translate("en", "messages_count", &p42),
        "42 new messages"
    );
}

#[test]
fn test_use_translation_hook_and_locale_switching() {
    let mut catalog = I18nCatalog::new("en");
    let mut en = HashMap::new();
    en.insert("farewell".to_string(), "Goodbye!".to_string());
    catalog.add_locale("en", en);

    let mut fr = HashMap::new();
    fr.insert("farewell".to_string(), "Au revoir !".to_string());
    catalog.add_locale("fr", fr);

    set_global_catalog(catalog);

    let (t, locale, set_locale) = use_translation();
    assert_eq!(locale.get(), "en");
    assert_eq!(t("farewell", &[]), "Goodbye!");

    // Switch locale
    set_locale("fr");
    assert_eq!(locale.get(), "fr");
    assert_eq!(t("farewell", &[]), "Au revoir !");
}
