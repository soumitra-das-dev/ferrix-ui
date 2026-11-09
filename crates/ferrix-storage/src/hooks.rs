//! Reactive hooks for Key-Value storage binding.

use serde::{de::DeserializeOwned, Serialize};
use std::cell::RefCell;
use std::sync::Arc;

use crate::store::KvStore;
use ferrix_reactivity::Signal;

thread_local! {
    static DEFAULT_STORE: RefCell<Option<KvStore>> = const { RefCell::new(None) };
}

/// Sets the thread-local default `KvStore` for reactive hooks.
pub fn set_default_store(store: KvStore) {
    DEFAULT_STORE.with(|cell| {
        *cell.borrow_mut() = Some(store);
    });
}

/// Retrieves the thread-local default `KvStore`. Defaults to in-memory store if unset.
pub fn get_default_store() -> KvStore {
    DEFAULT_STORE.with(|cell| {
        let mut borrowed = cell.borrow_mut();
        if borrowed.is_none() {
            *borrowed = Some(KvStore::open_in_memory());
        }
        borrowed.clone().unwrap()
    })
}

/// Reactive hook binding a persistent storage key to a fine-grained `Signal<T>`.
///
/// Returns `(signal, setter_fn)`:
/// - Any updates via `setter_fn` persist to storage and trigger reactive UI observers.
pub fn use_storage<T>(key: &str, default: T) -> (Signal<T>, Arc<dyn Fn(T)>)
where
    T: Clone + Serialize + DeserializeOwned + 'static,
{
    let store = get_default_store();
    let initial_val = store
        .get_json::<T>(key)
        .ok()
        .flatten()
        .unwrap_or_else(|| default.clone());

    let signal = Signal::new(initial_val);
    let sig_clone = signal.clone();
    let key_str = key.to_string();
    let store_clone = store.clone();

    let setter = Arc::new(move |new_val: T| {
        let _ = store_clone.set_json(&key_str, &new_val);
        sig_clone.set(new_val);
    });

    (signal, setter)
}
