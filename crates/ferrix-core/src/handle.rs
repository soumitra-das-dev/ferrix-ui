//! Imperative element handles and typed command dispatch for Ferrix components.

use ferrix_renderer_protocol::{Command, NodeId};
use std::cell::Cell;
use std::rc::Rc;

/// Persistent handle to a mounted UI element allowing imperative operations
/// via typed command buffers without direct View access.
#[derive(Clone, Debug, Default)]
pub struct ElementHandle {
    node_id: Rc<Cell<Option<NodeId>>>,
}

impl ElementHandle {
    pub fn new() -> Self {
        Self {
            node_id: Rc::new(Cell::new(None)),
        }
    }

    pub fn bind(&self, id: NodeId) {
        self.node_id.set(Some(id));
    }

    pub fn unbind(&self) {
        self.node_id.set(None);
    }

    pub fn current_node_id(&self) -> Option<NodeId> {
        self.node_id.get()
    }

    /// Emits a command to request focus on the target node.
    pub fn focus(&self) -> Option<Command> {
        self.current_node_id()
            .map(|node_id| Command::FocusNode { node_id })
    }

    /// Emits a command to clear focus from the target node.
    pub fn blur(&self) -> Option<Command> {
        self.current_node_id()
            .map(|node_id| Command::BlurNode { node_id })
    }

    /// Emits a command to scroll the target node.
    pub fn scroll_to(&self, x: f32, y: f32, animated: bool) -> Option<Command> {
        self.current_node_id().map(|node_id| Command::ScrollTo {
            node_id,
            x,
            y,
            animated,
        })
    }

    /// Emits a command to measure the target node's bounding frame on screen.
    pub fn measure(&self) -> Option<Command> {
        self.current_node_id()
            .map(|node_id| Command::MeasureNode { node_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_handle_binding_and_commands() {
        let handle = ElementHandle::new();
        assert_eq!(handle.focus(), None);

        handle.bind(NodeId(42));
        assert_eq!(
            handle.focus(),
            Some(Command::FocusNode {
                node_id: NodeId(42)
            })
        );
        assert_eq!(
            handle.scroll_to(0.0, 100.0, true),
            Some(Command::ScrollTo {
                node_id: NodeId(42),
                x: 0.0,
                y: 100.0,
                animated: true
            })
        );

        handle.unbind();
        assert_eq!(handle.focus(), None);
    }
}
