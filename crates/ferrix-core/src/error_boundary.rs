//! Runtime error boundary and failure recovery for component execution.

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::component::ComponentContext;
use crate::element::Element;

/// Information describing a runtime component failure or panic.
#[derive(Clone, Debug, PartialEq)]
pub struct ErrorInfo {
    /// Failure message extracted from the panic payload or error result.
    pub message: String,
    /// Name of the component where failure occurred, if identifiable.
    pub component_name: Option<String>,
}

impl ErrorInfo {
    /// Creates a new ErrorInfo with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            component_name: None,
        }
    }

    /// Sets the component name.
    pub fn with_component(mut self, name: impl Into<String>) -> Self {
        self.component_name = Some(name.into());
        self
    }
}

/// Executes a component render pass within an unwind-safe barrier.
/// If `try_render` panics, catches the unwind, constructs `ErrorInfo`, and renders `fallback`.
pub fn render_with_error_boundary<F, Fallback>(
    cx: &mut ComponentContext,
    try_render: F,
    fallback: Fallback,
) -> Element
where
    F: FnOnce(&mut ComponentContext) -> Element + std::panic::UnwindSafe,
    Fallback: FnOnce(&ErrorInfo, &mut ComponentContext) -> Element,
{
    let result = catch_unwind(AssertUnwindSafe(|| try_render(cx)));

    match result {
        Ok(element) => element,
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown component execution panic".to_string()
            };

            let info = ErrorInfo {
                message: msg,
                component_name: None,
            };

            fallback(&info, cx)
        }
    }
}
