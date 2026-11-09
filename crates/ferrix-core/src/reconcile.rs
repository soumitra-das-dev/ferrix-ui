//! UI tree reconciliation, diffing engine, and mount patch generation.

use crate::Element;
use ferrix_renderer_protocol::{Patch, PatchBatch};

/// Generates the initial batch of patches required to mount a complete Element tree.
pub fn generate_mount_patches(root: &Element) -> PatchBatch {
    let mut batch = PatchBatch::new();
    mount_subtree(root, &mut batch);
    batch
}

fn mount_subtree(node: &Element, batch: &mut PatchBatch) {
    // 1. Create the node
    batch.push(Patch::CreateNode {
        node_id: node.id,
        view_type: node.view_type,
    });

    // 2. Apply properties
    for prop in &node.props {
        batch.push(Patch::SetProperty {
            node_id: node.id,
            property: prop.clone(),
        });
    }

    // 3. Set initial text content
    if let Some(text) = &node.text {
        batch.push(Patch::SetText {
            node_id: node.id,
            text: text.clone(),
        });
    }

    // 4. Set visibility if non-default
    if node.visibility != ferrix_renderer_protocol::Visibility::Visible {
        batch.push(Patch::SetVisibility {
            node_id: node.id,
            visibility: node.visibility,
        });
    }

    // 5. Attach event listeners
    for event in &node.event_listeners {
        batch.push(Patch::AttachEventListener {
            node_id: node.id,
            event_type: *event,
        });
    }

    // 6. Recursively mount and append children
    for child in &node.children {
        mount_subtree(child, batch);
        batch.push(Patch::AppendChild {
            parent_id: node.id,
            child_id: child.id,
        });
    }
}

/// Diffs an existing Element tree against a new Element tree and produces the minimal PatchBatch.
pub fn diff_elements(old: &Element, new: &Element) -> PatchBatch {
    let mut batch = PatchBatch::new();
    diff_subtrees(old, new, &mut batch);
    batch
}

fn diff_subtrees(old: &Element, new: &Element, batch: &mut PatchBatch) {
    // If the node type or identity changed completely, replace the node
    if old.view_type != new.view_type || old.id != new.id || old.key != new.key {
        batch.push(Patch::DestroyNode { node_id: old.id });
        mount_subtree(new, batch);
        return;
    }

    // 1. Text diff
    if old.text != new.text {
        if let Some(text) = &new.text {
            batch.push(Patch::SetText {
                node_id: new.id,
                text: text.clone(),
            });
        }
    }

    // 2. Visibility diff
    if old.visibility != new.visibility {
        batch.push(Patch::SetVisibility {
            node_id: new.id,
            visibility: new.visibility,
        });
    }

    // 3. Properties diff
    for new_prop in &new.props {
        if !old.props.contains(new_prop) {
            batch.push(Patch::SetProperty {
                node_id: new.id,
                property: new_prop.clone(),
            });
        }
    }

    // 4. Event listeners diff
    for new_event in &new.event_listeners {
        if !old.event_listeners.contains(new_event) {
            batch.push(Patch::AttachEventListener {
                node_id: new.id,
                event_type: *new_event,
            });
        }
    }
    for old_event in &old.event_listeners {
        if !new.event_listeners.contains(old_event) {
            batch.push(Patch::DetachEventListener {
                node_id: new.id,
                event_type: *old_event,
            });
        }
    }

    // 5. Children diffing
    diff_children(old, new, batch);
}

fn diff_children(old: &Element, new: &Element, batch: &mut PatchBatch) {
    let old_len = old.children.len();
    let new_len = new.children.len();
    let common_len = old_len.min(new_len);

    // Diff existing paired children
    for i in 0..common_len {
        diff_subtrees(&old.children[i], &new.children[i], batch);
    }

    // Handle removed children
    if old_len > new_len {
        for old_child in &old.children[new_len..old_len] {
            batch.push(Patch::RemoveChild {
                parent_id: old.id,
                child_id: old_child.id,
            });
            batch.push(Patch::DestroyNode {
                node_id: old_child.id,
            });
        }
    }

    // Handle appended new children
    if new_len > old_len {
        for new_child in &new.children[old_len..new_len] {
            mount_subtree(new_child, batch);
            batch.push(Patch::AppendChild {
                parent_id: old.id,
                child_id: new_child.id,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_id::NodeIdAllocator;
    use crate::primitives::*;
    use ferrix_renderer_protocol::{NodeId, Property, ViewType};

    #[test]
    fn test_mount_patches_generation() {
        let alloc = NodeIdAllocator::with_start(10);
        let tree = Column::new()
            .padding(16.0)
            .child(Text::new("Hello").size(18.0))
            .child(Button::new("Click"))
            .build_with_id(NodeId::new(10), &alloc);

        let batch = generate_mount_patches(&tree);
        assert!(!batch.is_empty());

        // Check root creation
        assert_eq!(
            batch.patches[0],
            Patch::CreateNode {
                node_id: NodeId::new(10),
                view_type: ViewType::Column,
            }
        );
        // Padding prop
        assert_eq!(
            batch.patches[1],
            Patch::SetProperty {
                node_id: NodeId::new(10),
                property: Property::Padding {
                    left: 16.0,
                    top: 16.0,
                    right: 16.0,
                    bottom: 16.0,
                },
            }
        );
    }

    #[test]
    fn test_diff_text_mutation() {
        let alloc = NodeIdAllocator::with_start(10);
        let old_tree = Text::new("Initial").build_with_id(NodeId::new(10), &alloc);
        let new_tree = Text::new("Updated").build_with_id(NodeId::new(10), &alloc);

        let batch = diff_elements(&old_tree, &new_tree);
        assert_eq!(batch.len(), 1);
        assert_eq!(
            batch.patches[0],
            Patch::SetText {
                node_id: NodeId::new(10),
                text: "Updated".to_string(),
            }
        );
    }

    #[test]
    fn test_diff_children_addition_and_removal() {
        let alloc = NodeIdAllocator::with_start(10);
        let id = NodeId::new(10);

        let old_tree = Column::new()
            .child(Text::new("Child 1"))
            .child(Text::new("Child 2"))
            .build_with_id(id, &alloc);

        alloc.reset(11);
        let new_tree = Column::new()
            .child(Text::new("Child 1"))
            .child(Text::new("Child 2"))
            .child(Text::new("Child 3"))
            .build_with_id(id, &alloc);

        let batch = diff_elements(&old_tree, &new_tree);
        // Expect Child 3 created, text set, and appended
        assert!(batch.patches.iter().any(|p| matches!(
            p,
            Patch::SetText { text, .. } if text == "Child 3"
        )));
        assert!(batch.patches.iter().any(|p| matches!(
            p,
            Patch::AppendChild { parent_id, .. } if *parent_id == id
        )));
    }
}
