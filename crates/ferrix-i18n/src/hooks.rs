//! Reactive translation hooks.

use crate::catalog::I18nCatalog;
use ferrix_reactivity::Signal;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

thread_local! {
    static CURRENT_CATALOG: RefCell<I18nCatalog> = RefCell::new(I18nCatalog::default());
    static LOCALE_SIGNAL: RefCell<Option<Signal<String>>> = const { RefCell::new(None) };
}

/// Sets the translation catalog for the current runtime thread.
pub fn set_global_catalog(catalog: I18nCatalog) {
    let def = catalog.default_locale.clone();
    CURRENT_CATALOG.with(|cell| {
        *cell.borrow_mut() = catalog;
    });
    LOCALE_SIGNAL.with(|cell| {
        let existing = cell.borrow().clone();
        if let Some(sig) = existing {
            sig.set(def);
        } else {
            *cell.borrow_mut() = Some(Signal::new(def));
        }
    });
}

fn get_locale_signal() -> Signal<String> {
    LOCALE_SIGNAL.with(|cell| {
        let mut borrowed = cell.borrow_mut();
        if borrowed.is_none() {
            let def = CURRENT_CATALOG.with(|c| c.borrow().default_locale.clone());
            *borrowed = Some(Signal::new(def));
        }
        borrowed.clone().unwrap()
    })
}

pub type TranslateFn = Rc<dyn Fn(&str, &[(&str, &str)]) -> String>;
pub type SetLocaleFn = Rc<dyn Fn(&str)>;
pub type TranslationHookResult = (TranslateFn, Signal<String>, SetLocaleFn);

/// Reactive hook providing translation helper, current locale signal, and locale setter.
pub fn use_translation() -> TranslationHookResult {
    let locale_signal = get_locale_signal();
    let loc_sig_clone = locale_signal.clone();

    let t = Rc::new(move |key: &str, params: &[(&str, &str)]| {
        let current_locale = loc_sig_clone.get();
        let mut map = HashMap::new();
        for (k, v) in params {
            map.insert(k.to_string(), v.to_string());
        }

        CURRENT_CATALOG.with(|c| c.borrow().translate(&current_locale, key, &map))
    });

    let loc_sig_for_setter = locale_signal.clone();
    let set_locale = Rc::new(move |new_locale: &str| {
        loc_sig_for_setter.set(new_locale.to_string());
    });

    (t, locale_signal, set_locale)
}
