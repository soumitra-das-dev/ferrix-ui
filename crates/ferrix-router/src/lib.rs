//! # ferrix-router
//!
//! File-based, statically-typed routing and deep-linking system for Ferrix.
//! Architecture A compliant: Rust owns state, history, stack, and URL matching;
//! Kotlin renders native view containers.

pub mod components;
pub mod deeplink;
pub mod hooks;
pub mod manifest;
pub mod state;

pub use components::{
    link, redirect, LinkProps, StackAnimation, StackPresentation, StackScreenOptions,
};
pub use deeplink::{parse_deep_link, DeepLinkError};
pub use hooks::{
    get_current_router, set_current_router, use_can_go_back, use_focus_effect,
    use_global_search_params, use_is_focused, use_local_search_params, use_pathname, use_router,
    use_segments, RouterHandle,
};
pub use manifest::{RouteEntry, RouteManifest, RouteMatch, RouterError, SegmentPattern};
pub use state::{RouterSnapshot, RouterState, StackEntry};

pub mod prelude {
    pub use crate::components::{StackAnimation, StackPresentation, StackScreenOptions};
    pub use crate::hooks::{
        use_can_go_back, use_focus_effect, use_global_search_params, use_is_focused,
        use_local_search_params, use_pathname, use_router, use_segments, RouterHandle,
    };
    pub use crate::manifest::{RouteEntry, RouteManifest, RouteMatch, RouterError};
    pub use crate::state::{RouterSnapshot, RouterState};
}
