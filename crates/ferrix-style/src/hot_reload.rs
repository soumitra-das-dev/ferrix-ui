//! Hot Style Reloading engine.
//!
//! Enables instantaneous style updates during development without recompiling the Rust binary
//! or restarting the Android Activity/View tree. All component reactive signals, ephemeral state,
//! text inputs, and scroll positions are 100% preserved.

use std::collections::HashMap;
use std::time::Instant;

use ferrix_renderer_protocol::{NodeId, PatchBatch};

use crate::cascade::CascadeEngine;
use crate::ir::StyleIr;
use crate::resolver::ReactiveStyleResolver;

/// Information about a mounted node used to recompute styles during hot reload.
#[derive(Clone, Debug, Default)]
pub struct MountedNodeInfo {
    pub classes: Vec<String>,
    pub element_type: Option<String>,
    pub inline: Option<StyleIr>,
}

impl MountedNodeInfo {
    pub fn new(
        classes: Vec<String>,
        element_type: Option<String>,
        inline: Option<StyleIr>,
    ) -> Self {
        Self {
            classes,
            element_type,
            inline,
        }
    }
}

/// Result of a hot style reload operation.
#[derive(Clone, Debug)]
pub struct HotReloadResult {
    pub batch: PatchBatch,
    pub updated_nodes: usize,
    pub elapsed_ms: f64,
}

/// Computes hot style reload patches when stylesheet rules are modified during development.
pub fn compute_hot_reload_patches(
    resolver: &mut ReactiveStyleResolver,
    cascade: &CascadeEngine,
    node_metadata: &HashMap<NodeId, MountedNodeInfo>,
) -> HotReloadResult {
    let start = Instant::now();
    let mut batch = PatchBatch::new();
    let mut updated_nodes = 0;

    for (&node_id, info) in node_metadata {
        let class_refs: Vec<&str> = info.classes.iter().map(String::as_str).collect();
        let el_type = info.element_type.as_deref();
        let new_style = cascade.resolve(&class_refs, el_type, info.inline.as_ref());

        // Re-register or update the node style
        let (_style_id, set_patch_opt) = resolver.update_node_style(node_id, new_style);
        if let Some(set_patch) = set_patch_opt {
            batch.push(set_patch);
            updated_nodes += 1;
        }
    }

    // Prepend any new style registration commands
    if let Some(reg_patch) = resolver.registry_mut().take_registration_patch() {
        batch.patches.insert(0, reg_patch);
    }

    let elapsed = start.elapsed();
    let elapsed_ms = elapsed.as_secs_f64() * 1000.0;

    HotReloadResult {
        batch,
        updated_nodes,
        elapsed_ms,
    }
}
