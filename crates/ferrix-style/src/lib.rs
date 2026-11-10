//! # ferrix-style
//!
//! Native Style IR, intern pool style registry, CSS cascade engine, specificity calculator,
//! and reactive style resolver for the Ferrix UI framework under Architecture A.

pub mod cascade;
pub mod color;
pub mod helpers;
pub mod hot_reload;
pub mod ir;
pub mod registry;
pub mod resolver;

pub use cascade::{CascadeEngine, CascadeRule, RuleSource, Specificity};
pub use color::Color;
pub use helpers::{cx, VariantConfig};
pub use hot_reload::{compute_hot_reload_patches, HotReloadResult, MountedNodeInfo};
pub use ir::StyleIr;
pub use registry::StyleRegistry;
pub use resolver::{
    ColorScheme, ElementState, EnvironmentState, ReactiveStyleResolver, WindowDimensions,
};
