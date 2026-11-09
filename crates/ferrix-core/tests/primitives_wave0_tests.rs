use ferrix_core::prelude::*;
use ferrix_layout::{DimensionValue, FlexDirection, LayoutEngine, LayoutStyle};

#[test]
fn test_view_and_text_primitive_styling_and_patch_generation() {
    let allocator = NodeIdAllocator::new();

    let style = Style::new()
        .background_color(Color::PRIMARY)
        .padding(16.0)
        .border_radius(8.0)
        .elevation(4.0);

    let text_style = Style::new()
        .color(Color::WHITE)
        .font_size(20.0)
        .font_weight(700);

    let view = View::new()
        .style(&style)
        .test_id("test_root_view")
        .accessibility_label("Root Container")
        .child(
            Text::new("Styled Heading")
                .style(&text_style)
                .test_id("heading_text")
                .number_of_lines(1),
        );

    let element = view.build(&allocator);

    assert_eq!(element.view_type, ViewType::Container);
    assert_eq!(element.children.len(), 1);

    let child = &element.children[0];
    assert_eq!(child.view_type, ViewType::Text);
    assert_eq!(child.text, Some("Styled Heading".to_string()));

    // Generate mount patches
    let patches = generate_mount_patches(&element);
    assert!(patches.len() >= 6); // CreateNode x2, AppendChild, SetText, SetProperty x...

    // Verify properties contain TestId, BackgroundColor, etc.
    let has_test_id = patches.patches.iter().any(|p| match p {
        Patch::SetProperty { property, .. } => {
            matches!(property, Property::TestId(id) if id == "test_root_view")
        }
        _ => false,
    });
    assert!(has_test_id, "Mount patches should include TestId");
}

#[test]
fn test_pressable_interactive_configuration() {
    let allocator = NodeIdAllocator::new();

    let pressable = Pressable::new()
        .test_id("primary_btn")
        .ripple_color(0x33000000)
        .hit_slop(8.0, 8.0, 8.0, 8.0)
        .accessibility_label("Submit Action")
        .on_press()
        .on_press_in()
        .on_press_out()
        .child(Text::new("Submit"));

    let element = pressable.build(&allocator);
    assert_eq!(element.view_type, ViewType::Pressable);
    assert!(element.event_listeners.contains(&EventType::Click));
    assert!(element.event_listeners.contains(&EventType::PressIn));
    assert!(element.event_listeners.contains(&EventType::PressOut));

    let patches = generate_mount_patches(&element);
    let has_hit_slop = patches.patches.iter().any(|p| match p {
        Patch::SetProperty { property, .. } => {
            matches!(property, Property::HitSlop { left, top, .. } if left == &8.0 && top == &8.0)
        }
        _ => false,
    });
    assert!(has_hit_slop, "Mount patches must contain HitSlop property");
}

#[test]
fn test_taffy_layout_integration_with_intrinsic_text_measure() {
    let mut layout_engine = LayoutEngine::new();

    let root_id = NodeId(1);
    let text_id = NodeId(2);

    let root_style = LayoutStyle {
        flex_direction: FlexDirection::Column,
        width: DimensionValue::Points(360.0),
        height: DimensionValue::Points(600.0),
        padding_left: DimensionValue::Points(16.0),
        padding_top: DimensionValue::Points(16.0),
        ..Default::default()
    };

    let text_style = LayoutStyle::default();
    let intrinsic_text_size = (180.0, 24.0);

    layout_engine
        .upsert_node(text_id, text_style, Some(intrinsic_text_size), vec![])
        .unwrap();
    layout_engine
        .upsert_node(root_id, root_style, None, vec![text_id])
        .unwrap();

    let layouts = layout_engine.compute_layout(root_id, 360.0, 600.0).unwrap();
    let text_layout = layouts.iter().find(|l| l.node_id == text_id).unwrap();

    assert_eq!(text_layout.x, 16.0);
    assert_eq!(text_layout.y, 16.0);
    assert_eq!(text_layout.width, 180.0);
    assert_eq!(text_layout.height, 24.0);
}

#[test]
fn test_single_prop_change_produces_single_patch() {
    let allocator = NodeIdAllocator::new();

    let old_view = View::new()
        .background_color(Color::WHITE)
        .padding(16.0)
        .build(&allocator);

    let mut new_view = old_view.clone();
    // Only background color changes
    new_view.props[0] = Property::BackgroundColor(Color::PRIMARY);

    let patches = diff_elements(&old_view, &new_view);
    assert_eq!(patches.len(), 1, "Exactly 1 patch for 1 property change");
    assert!(matches!(
        patches.patches[0],
        Patch::SetProperty {
            property: Property::BackgroundColor(Color::PRIMARY),
            ..
        }
    ));
}
