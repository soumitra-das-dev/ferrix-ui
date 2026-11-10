//! Synthetic event helpers for headless component testing.

use ferrix_renderer_protocol::NodeId;

use crate::renderer::TestRenderer;

/// Module namespace providing event simulation dispatchers.
pub mod fire_event {
    use super::*;

    /// Simulates a click / touch press event on a node.
    pub fn press(renderer: &mut TestRenderer, node_id: NodeId) {
        renderer.press(node_id);
    }

    /// Simulates a text change event on a TextInput node.
    pub fn change_text(renderer: &mut TestRenderer, node_id: NodeId, text: impl Into<String>) {
        renderer.change_text(node_id, text);
    }

    /// Simulates a boolean toggle on a Switch or Checkbox node.
    pub fn toggle(renderer: &mut TestRenderer, node_id: NodeId, value: bool) {
        renderer.toggle_value(node_id, value);
    }
}
