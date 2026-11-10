//! # ferrix-store
//!
//! Fine-grained reactive state container modeled on Redux/Zustand for Ferrix.
//! Invariant: Selector subscriptions update only dependent nodes; never re-render the whole tree.

pub mod store;

pub use store::Store;

pub mod prelude {
    pub use crate::store::Store;
}
