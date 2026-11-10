//! # ferrix-devtools
//!
//! Developer tools, frame rate monitor, patch buffer inspector, reactive signal graph debugger,
//! and background task scheduler for Ferrix.

pub mod background;
pub mod fps;
pub mod overlay;
pub mod patches;
pub mod signals;

pub use background::{BackgroundScheduler, BackgroundTask, TaskStatus};
pub use fps::FpsMonitor;
pub use overlay::build_devtools_overlay;
pub use patches::{BatchSummary, PatchInspector};
pub use signals::{SignalDebugInfo, SignalInspector};

/// Convenience prelude for devtools.
pub mod prelude {
    pub use super::background::{BackgroundScheduler, BackgroundTask, TaskStatus};
    pub use super::fps::FpsMonitor;
    pub use super::overlay::build_devtools_overlay;
    pub use super::patches::{BatchSummary, PatchInspector};
    pub use super::signals::{SignalDebugInfo, SignalInspector};
}
