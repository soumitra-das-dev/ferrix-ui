//! # ferrix-test
//!
//! Headless component testing framework, virtual tree inspector, and event simulation harness for Ferrix.
//! Enables testing Ferrix UI components on the host machine without requiring an Android emulator, JVM, or device.

pub mod event;
pub mod node;
pub mod query;
pub mod renderer;
pub mod snapshot;
pub mod timers;

pub use event::fire_event;
pub use node::TestNode;
pub use query::{QueryEngine, QueryError};
pub use renderer::TestRenderer;
pub use snapshot::format_snapshot;
pub use timers::FakeTimers;

use ferrix_core::prelude::{ComponentContext, Element};

/// Renders a functional component into a headless `TestRenderer` environment.
pub fn render<F: FnMut(&mut ComponentContext) -> Element + 'static>(f: F) -> TestRenderer {
    TestRenderer::new(f)
}

/// Mounts an existing Element tree into a headless `TestRenderer`.
pub fn render_element(element: Element) -> TestRenderer {
    TestRenderer::from_element(element)
}

/// Convenience prelude importing common testing utilities.
pub mod prelude {
    pub use super::event::fire_event;
    pub use super::node::TestNode;
    pub use super::query::QueryError;
    pub use super::renderer::TestRenderer;
    pub use super::timers::FakeTimers;
    pub use super::{render, render_element};
    pub use ferrix_core::prelude::*;
}
