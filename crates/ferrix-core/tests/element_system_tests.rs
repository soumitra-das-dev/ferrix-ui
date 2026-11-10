use ferrix_core::prelude::*;

#[test]
fn test_complex_element_tree_mount_and_reconciliation() {
    let alloc = NodeIdAllocator::with_start(100);

    let initial_tree = Column::new()
        .padding(20.0)
        .spacing(12.0)
        .background_color(0xFFEEEEEE)
        .child(Text::new("Ferrix Header").size(24.0).color(0xFF111111))
        .child(
            Row::new()
                .spacing(8.0)
                .child(Button::new("Cancel").enabled(true))
                .child(Button::new("Confirm").enabled(true)),
        )
        .build_with_id(NodeId::new(100), &alloc);

    assert_eq!(initial_tree.node_count(), 5); // Column, Text, Row, Button, Button
    assert_eq!(initial_tree.children.len(), 2);

    let mount_patches = generate_mount_patches(&initial_tree);
    assert!(!mount_patches.is_empty());

    // Verify all 4 view types are present in mount batch
    let types: Vec<_> = mount_patches
        .patches
        .iter()
        .filter_map(|p| match p {
            Patch::CreateNode { view_type, .. } => Some(*view_type),
            _ => None,
        })
        .collect();

    assert!(types.contains(&ViewType::Column));
    assert!(types.contains(&ViewType::Text));
    assert!(types.contains(&ViewType::Row));
    assert!(types.contains(&ViewType::Button));

    // Now test diffing: mutate text and disable one button
    alloc.reset(100);
    let updated_tree = Column::new()
        .padding(20.0)
        .spacing(12.0)
        .background_color(0xFFEEEEEE)
        .child(
            Text::new("Ferrix Header - Mutated")
                .size(24.0)
                .color(0xFF111111),
        )
        .child(
            Row::new()
                .spacing(8.0)
                .child(Button::new("Cancel").enabled(false))
                .child(Button::new("Confirm").enabled(true)),
        )
        .build_with_id(NodeId::new(100), &alloc);

    let diff_patches = diff_elements(&initial_tree, &updated_tree);

    // Diff should contain SetText for header and SetProperty for enabled(false)
    let text_patches: Vec<_> = diff_patches
        .patches
        .iter()
        .filter_map(|p| match p {
            Patch::SetText { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(text_patches, vec!["Ferrix Header - Mutated"]);

    let prop_patches: Vec<_> = diff_patches
        .patches
        .iter()
        .filter_map(|p| match p {
            Patch::SetProperty { property, .. } => Some(property),
            _ => None,
        })
        .collect();
    assert!(prop_patches
        .iter()
        .any(|p| matches!(p, Property::Enabled(false))));
}
