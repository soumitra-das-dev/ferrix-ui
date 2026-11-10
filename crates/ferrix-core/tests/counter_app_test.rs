use ferrix_core::prelude::*;

/// Hand-written functional Counter component using hooks and reactive state.
fn counter_component(cx: &mut ComponentContext, initial: i32) -> (Element, State<i32>) {
    let count = cx.use_state(initial);

    let tree = Column::new()
        .padding(16.0)
        .spacing(12.0)
        .child(Text::new("Counter App").size(24.0).color(0xFF0F172A))
        .child(
            Text::new(format!("Count: {}", count.get()))
                .size(20.0)
                .color(0xFF334155),
        )
        .child(
            Row::new()
                .spacing(8.0)
                .child(
                    Button::new("Increment")
                        .background_color(0xFF2563EB)
                        .text_color(0xFFFFFFFF),
                )
                .child(
                    Button::new("Decrement")
                        .background_color(0xFF64748B)
                        .text_color(0xFFFFFFFF),
                ),
        )
        .build_with_id(NodeId::new(1), cx.allocator());

    (tree, count)
}

#[test]
fn test_counter_app_reactive_execution() {
    let mut cx = ComponentContext::with_start_id(2);

    // Initial render
    cx.prepare_render();
    let (initial_tree, count) = counter_component(&mut cx, 0);
    let mount_patches = generate_mount_patches(&initial_tree);
    assert!(!mount_patches.is_empty());

    // User clicks increment: state mutates from 0 -> 1
    count.set(1);

    // Second render pass
    cx.prepare_render();
    let (next_tree, _) = counter_component(&mut cx, 0);
    let diff = diff_elements(&initial_tree, &next_tree);

    // Exactly 1 SetText patch emitted
    assert_eq!(diff.len(), 1);
    match &diff.patches[0] {
        Patch::SetText { node_id, text } => {
            assert_eq!(*node_id, NodeId::new(3));
            assert_eq!(text, "Count: 1");
        }
        other => panic!("Expected SetText patch, got {:?}", other),
    }

    // User clicks increment again: state mutates 1 -> 2
    count.set(2);
    cx.prepare_render();
    let (third_tree, _) = counter_component(&mut cx, 0);
    let diff2 = diff_elements(&next_tree, &third_tree);
    assert_eq!(diff2.len(), 1);
    assert_eq!(
        diff2.patches[0],
        Patch::SetText {
            node_id: NodeId::new(3),
            text: "Count: 2".to_string()
        }
    );
}
