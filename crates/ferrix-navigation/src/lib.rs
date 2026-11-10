//! # ferrix-navigation
//!
//! Native navigation state machines, container drivers, and transition coordination.
//! Under Architecture A, Rust owns the navigation hierarchy and stack state;
//! Kotlin renders native container ViewGroups (FerrixNavContainer, FerrixScreenView).

pub mod options;
pub mod stack;
pub mod tabs;

pub use options::{PresentationStyle, ScreenOptions, TransitionAnimation};
pub use stack::{ScreenEntry, StackNavigator};
pub use tabs::{TabEntry, TabNavigator};

pub mod prelude {
    pub use crate::options::{PresentationStyle, ScreenOptions, TransitionAnimation};
    pub use crate::stack::{ScreenEntry, StackNavigator};
    pub use crate::tabs::{TabEntry, TabNavigator};
}
