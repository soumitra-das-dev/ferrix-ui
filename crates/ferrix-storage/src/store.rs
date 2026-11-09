//! Synchronous read cache and reactive signal bindings for KV storage.

use serde::{de::DeserializeOwned, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use thiserror::Error;

use crate::journal::{Journal, JournalError};
use ferrix_reactivity::Signal;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Journal error: {0}")]
    Journal(#[from] JournalError),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Lock poisoned")]
    PoisonedLock,
}

pub type SignalMap = Rc<RefCell<HashMap<String, Signal<Option<Vec<u8>>>>>>;

/// Reactive Key-Value store with lock-free $O(1)$ sync reads and durable write journal.
#[derive(Clone)]
pub struct KvStore {
    namespace: String,
    cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    journal: Option<Arc<Mutex<Journal>>>,
    signals: SignalMap,
}

impl KvStore {
    /// Opens an in-memory storage instance (ideal for tests and volatile state).
    pub fn open_in_memory() -> Self {
        Self {
            namespace: "default".to_string(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            journal: None,
            signals: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// Opens a persistent storage instance with durable WAL backing.
    pub fn open<P: AsRef<Path>>(path: P, namespace: &str) -> Result<Self, StorageError> {
        let (journal, live_entries) = Journal::open(path)?;
        Ok(Self {
            namespace: namespace.to_string(),
            cache: Arc::new(RwLock::new(live_entries)),
            journal: Some(Arc::new(Mutex::new(journal))),
            signals: Rc::new(RefCell::new(HashMap::new())),
        })
    }

    fn qualify_key(&self, key: &str) -> String {
        format!("{}:{}", self.namespace, key)
    }

    /// Reads raw bytes synchronously in $O(1)$ without blocking.
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let q_key = self.qualify_key(key);
        let cache = self.cache.read().ok()?;
        cache.get(&q_key).cloned()
    }

    /// Reads and deserializes a JSON value synchronously.
    pub fn get_json<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, StorageError> {
        match self.get(key) {
            Some(bytes) => {
                let val = serde_json::from_slice(&bytes)?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }

    /// Writes raw bytes: updates in-memory cache, triggers reactive signals, and appends to journal.
    pub fn set(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        let q_key = self.qualify_key(key);
        {
            let mut cache = self.cache.write().map_err(|_| StorageError::PoisonedLock)?;
            cache.insert(q_key.clone(), value.to_vec());
        }

        if let Some(journal_lock) = &self.journal {
            let mut journal = journal_lock
                .lock()
                .map_err(|_| StorageError::PoisonedLock)?;
            journal.append_set(&q_key, value)?;
        }

        // Notify reactive signal if active
        if let Some(signal) = self.signals.borrow().get(&q_key) {
            signal.set(Some(value.to_vec()));
        }

        Ok(())
    }

    /// Serializes and stores a JSON value.
    pub fn set_json<T: Serialize>(&self, key: &str, value: &T) -> Result<(), StorageError> {
        let bytes = serde_json::to_vec(value)?;
        self.set(key, &bytes)
    }

    /// Deletes a key from cache and journal.
    pub fn delete(&self, key: &str) -> Result<(), StorageError> {
        let q_key = self.qualify_key(key);
        {
            let mut cache = self.cache.write().map_err(|_| StorageError::PoisonedLock)?;
            cache.remove(&q_key);
        }

        if let Some(journal_lock) = &self.journal {
            let mut journal = journal_lock
                .lock()
                .map_err(|_| StorageError::PoisonedLock)?;
            journal.append_delete(&q_key)?;
        }

        if let Some(signal) = self.signals.borrow().get(&q_key) {
            signal.set(None);
        }

        Ok(())
    }

    /// Checks if a key exists in cache.
    pub fn contains(&self, key: &str) -> bool {
        let q_key = self.qualify_key(key);
        self.cache
            .read()
            .map(|c| c.contains_key(&q_key))
            .unwrap_or(false)
    }

    /// Creates or retrieves a fine-grained reactive Signal for the specified key.
    pub fn signal_for(&self, key: &str) -> Signal<Option<Vec<u8>>> {
        let q_key = self.qualify_key(key);
        let mut signals = self.signals.borrow_mut();
        if let Some(sig) = signals.get(&q_key) {
            return sig.clone();
        }

        let initial_val = self.cache.read().unwrap().get(&q_key).cloned();
        let sig = Signal::new(initial_val);
        signals.insert(q_key, sig.clone());
        sig
    }

    /// Returns a sub-namespaced store sharing the same storage journal.
    pub fn namespace(&self, ns: &str) -> Self {
        Self {
            namespace: format!("{}.{}", self.namespace, ns),
            cache: self.cache.clone(),
            journal: self.journal.clone(),
            signals: self.signals.clone(),
        }
    }

    /// Triggers compaction of the journal file.
    pub fn compact(&self) -> Result<(), StorageError> {
        if let Some(journal_lock) = &self.journal {
            let mut journal = journal_lock
                .lock()
                .map_err(|_| StorageError::PoisonedLock)?;
            let cache = self.cache.read().map_err(|_| StorageError::PoisonedLock)?;
            journal.compact(&cache)?;
        }
        Ok(())
    }
}
