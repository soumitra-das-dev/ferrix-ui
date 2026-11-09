//! # ferrix-core
//!
//! Core component model, element tree, builders, and reconciliation engine for Ferrix.

pub mod component;
pub mod element;
pub mod node_id;
pub mod primitives;
pub mod reconcile;

pub use component::{ComponentContext, State};
pub use element::{Element, ElementDef};
pub use node_id::NodeIdAllocator;
pub use primitives::{Button, Column, Row, Text};
pub use reconcile::{diff_elements, generate_mount_patches};

pub mod prelude {
    pub use crate::component::{ComponentContext, State};
    pub use crate::element::{Element, ElementDef};
    pub use crate::node_id::NodeIdAllocator;
    pub use crate::primitives::{Button, Column, Row, Text};
    pub use crate::reconcile::{diff_elements, generate_mount_patches};
    pub use ferrix_reactivity::prelude::*;
    pub use ferrix_renderer_protocol::{
        EventType, NodeId, Patch, PatchBatch, Property, ViewType, Visibility,
    };
}
