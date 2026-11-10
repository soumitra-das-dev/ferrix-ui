//! Patch buffer inspector for monitoring JNI communication metrics.

use std::collections::HashMap;

use ferrix_renderer_protocol::{Patch, PatchBatch};

/// Summary metrics of a dispatched patch batch.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchSummary {
    pub total_patches: usize,
    pub set_text_count: usize,
    pub set_property_count: usize,
    pub create_node_count: usize,
    pub estimated_bytes: usize,
}

/// Tracks and inspects JNI patch batches emitted from Rust to Kotlin.
#[derive(Clone, Debug, Default)]
pub struct PatchInspector {
    total_batches: usize,
    total_patches: usize,
    patch_type_counts: HashMap<String, usize>,
    recent_batches: Vec<BatchSummary>,
}

impl PatchInspector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a dispatched PatchBatch.
    pub fn record_batch(&mut self, batch: &PatchBatch) {
        self.total_batches += 1;
        self.total_patches += batch.len();

        let mut set_text = 0;
        let mut set_prop = 0;
        let mut create_node = 0;

        for patch in &batch.patches {
            let key = match patch {
                Patch::CreateNode { .. } => {
                    create_node += 1;
                    "CreateNode"
                }
                Patch::DestroyNode { .. } => "DestroyNode",
                Patch::AppendChild { .. } => "AppendChild",
                Patch::RemoveChild { .. } => "RemoveChild",
                Patch::SetProperty { .. } => {
                    set_prop += 1;
                    "SetProperty"
                }
                Patch::SetText { .. } => {
                    set_text += 1;
                    "SetText"
                }
                Patch::SetVisibility { .. } => "SetVisibility",
                Patch::AttachEventListener { .. } => "AttachEventListener",
                Patch::DetachEventListener { .. } => "DetachEventListener",
                _ => "Other",
            };
            *self.patch_type_counts.entry(key.to_string()).or_insert(0) += 1;
        }

        let summary = BatchSummary {
            total_patches: batch.len(),
            set_text_count: set_text,
            set_property_count: set_prop,
            create_node_count: create_node,
            estimated_bytes: batch.len() * 32, // baseline estimated protocol header + payload
        };

        if self.recent_batches.len() >= 20 {
            self.recent_batches.remove(0);
        }
        self.recent_batches.push(summary);
    }

    /// Total number of batches recorded.
    pub fn total_batches(&self) -> usize {
        self.total_batches
    }

    /// Total number of individual patches recorded.
    pub fn total_patches(&self) -> usize {
        self.total_patches
    }

    /// Frequency distribution by patch variant.
    pub fn type_counts(&self) -> &HashMap<String, usize> {
        &self.patch_type_counts
    }

    /// Most recently recorded batch summaries.
    pub fn recent_batches(&self) -> &[BatchSummary] {
        &self.recent_batches
    }
}
