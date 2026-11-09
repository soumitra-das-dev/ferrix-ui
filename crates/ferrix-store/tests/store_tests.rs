//! Unit and integration tests for ferrix-store (F4).

use ferrix_storage::KvStore;
use ferrix_store::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct AppState {
    user: User,
    count: i32,
    theme: String,
}

#[test]
fn test_store_basic_update_and_get() {
    let store = Store::new(AppState {
        user: User {
            id: 1,
            name: "Soumitra".to_string(),
        },
        count: 0,
        theme: "dark".to_string(),
    });

    assert_eq!(store.get_state().count, 0);

    store.update(|s| s.count += 1);
    assert_eq!(store.get_state().count, 1);
}

#[test]
fn test_fine_grained_selector_isolation() {
    let store = Store::new(AppState {
        user: User {
            id: 1,
            name: "Alice".to_string(),
        },
        count: 10,
        theme: "light".to_string(),
    });

    let count_signal = store.select(|s| s.count);
    let name_signal = store.select(|s| s.user.name.clone());

    assert_eq!(count_signal.get(), 10);
    assert_eq!(name_signal.get(), "Alice");

    // Updating count should update count_signal
    store.update(|s| s.count += 5);
    assert_eq!(count_signal.get(), 15);
    assert_eq!(name_signal.get(), "Alice");

    // Updating name should update name_signal
    store.update(|s| s.user.name = "Alice Wonder".to_string());
    assert_eq!(count_signal.get(), 15);
    assert_eq!(name_signal.get(), "Alice Wonder");
}

#[test]
fn test_store_persistence_middleware() {
    let storage = KvStore::open_in_memory();
    let initial_state = AppState {
        user: User {
            id: 42,
            name: "Bob".to_string(),
        },
        count: 100,
        theme: "dark".to_string(),
    };

    let store = Store::new(initial_state).with_persistence("app_state_key", storage.clone());

    store.update(|s| s.count = 999);

    // Verify written to storage
    let persisted: AppState = storage.get_json("app_state_key").unwrap().unwrap();
    assert_eq!(persisted.count, 999);

    // Rehydrate new store from storage
    let new_store = Store::new(AppState {
        user: User {
            id: 0,
            name: "Default".to_string(),
        },
        count: 0,
        theme: "light".to_string(),
    })
    .with_persistence("app_state_key", storage);

    assert_eq!(new_store.get_state().count, 999);
    assert_eq!(new_store.get_state().user.id, 42);
}
