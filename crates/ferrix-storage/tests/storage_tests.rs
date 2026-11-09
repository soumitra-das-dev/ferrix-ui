//! Unit and integration tests for ferrix-storage (F3).

use ferrix_storage::hooks::{get_default_store, set_default_store, use_storage};
use ferrix_storage::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use tempfile::tempdir;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct UserSettings {
    theme: String,
    notifications: bool,
    font_scale: f32,
}

#[test]
fn test_in_memory_kv_store_crud() {
    let store = KvStore::open_in_memory();

    assert_eq!(store.get("key1"), None);
    assert!(!store.contains("key1"));

    store.set("key1", b"hello ferrix").unwrap();
    assert_eq!(store.get("key1"), Some(b"hello ferrix".to_vec()));
    assert!(store.contains("key1"));

    store.delete("key1").unwrap();
    assert_eq!(store.get("key1"), None);
    assert!(!store.contains("key1"));
}

#[test]
fn test_json_serialization_and_namespaces() {
    let store = KvStore::open_in_memory();
    let auth_store = store.namespace("auth");
    let ui_store = store.namespace("ui");

    let settings = UserSettings {
        theme: "dark".to_string(),
        notifications: true,
        font_scale: 1.2,
    };

    ui_store.set_json("settings", &settings).unwrap();

    let loaded: UserSettings = ui_store.get_json("settings").unwrap().unwrap();
    assert_eq!(loaded, settings);

    // auth_store should not see ui_store's settings
    assert_eq!(
        auth_store.get_json::<UserSettings>("settings").unwrap(),
        None
    );
}

#[test]
fn test_persistent_wal_journal_and_crash_recovery() {
    let dir = tempdir().unwrap();
    let journal_path = dir.path().join("storage.wal");

    {
        let store = KvStore::open(&journal_path, "main").unwrap();
        store.set("k1", b"value1").unwrap();
        store.set("k2", b"value2").unwrap();
        store.set("k1", b"value1_updated").unwrap();
    }

    // Reopen from disk
    {
        let store2 = KvStore::open(&journal_path, "main").unwrap();
        assert_eq!(store2.get("k1"), Some(b"value1_updated".to_vec()));
        assert_eq!(store2.get("k2"), Some(b"value2".to_vec()));
    }

    // Simulate partial/torn record at EOF (crash during write)
    {
        let mut file = OpenOptions::new().append(true).open(&journal_path).unwrap();
        file.write_all(b"GARBAGE_TORN_RECORD_TRAILING").unwrap();
    }

    // Reopening should cleanly recover up to last valid CRC32 record
    {
        let store3 = KvStore::open(&journal_path, "main").unwrap();
        assert_eq!(store3.get("k1"), Some(b"value1_updated".to_vec()));
        assert_eq!(store3.get("k2"), Some(b"value2".to_vec()));
    }
}

#[test]
fn test_journal_compaction() {
    let dir = tempdir().unwrap();
    let journal_path = dir.path().join("compact_test.wal");

    let store = KvStore::open(&journal_path, "main").unwrap();
    for i in 0..100 {
        store.set("counter", format!("{}", i).as_bytes()).unwrap();
    }

    let pre_compact_size = std::fs::metadata(&journal_path).unwrap().len();

    store.compact().unwrap();

    let post_compact_size = std::fs::metadata(&journal_path).unwrap().len();
    assert!(post_compact_size < pre_compact_size);

    assert_eq!(store.get("counter"), Some(b"99".to_vec()));
}

#[test]
fn test_use_storage_reactive_hook() {
    let store = KvStore::open_in_memory();
    set_default_store(store);

    let (theme, set_theme) = use_storage("app_theme", "light".to_string());
    assert_eq!(theme.get(), "light");

    set_theme("dark".to_string());
    assert_eq!(theme.get(), "dark");

    // In-memory store was updated
    let store_val: String = get_default_store().get_json("app_theme").unwrap().unwrap();
    assert_eq!(store_val, "dark");
}
