//! # ferrix-storage
//!
//! High-performance, crash-safe reactive Key-Value store for Ferrix.
//! Invariant: Synchronous $O(1)$ reads from in-memory cache, asynchronous
//! durable write WAL journal with CRC32 integrity verification.

pub mod hooks;
pub mod journal;
pub mod store;

pub use hooks::{get_default_store, set_default_store, use_storage};
pub use journal::{calculate_crc32, Journal, JournalError};
pub use store::{KvStore, StorageError};

pub mod prelude {
    pub use crate::hooks::use_storage;
    pub use crate::store::KvStore;
}
