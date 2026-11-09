//! # ferrix-reactivity
//!
//! Fine-grained reactive signals, dependency tracking, memos, effects, and batching runtime for Ferrix.

pub mod effect;
pub mod graph;
pub mod memo;
pub mod signal;

pub use effect::{CleanupFn, Effect};
pub use graph::{batch, track_signal, trigger_signal, with_observer, ObserverId, SignalId};
pub use memo::Memo;
pub use signal::Signal;

pub mod prelude {
    pub use crate::effect::{CleanupFn, Effect};
    pub use crate::graph::batch;
    pub use crate::memo::Memo;
    pub use crate::signal::Signal;
}
