use ferrix_core::prelude::*;
use ferrix_renderer_protocol::{Patch, Property, ViewType};

#[test]
fn test_wave2_primitives_element_construction() {
    let allocator = NodeIdAllocator::new();

    // 1. Layout helpers: Stack, ZStack, Spacer, Divider, Center, Grid
    let stack = Stack::new()
        .child(Text::new("Layer 1"))
        .child(Text::new("Layer 2"))
        .build(&allocator);
    assert_eq!(stack.view_type, ViewType::Stack);
    assert_eq!(stack.children.len(), 2);

    let zstack = ZStack::new().child(Text::new("Base")).build(&allocator);
    assert_eq!(zstack.view_type, ViewType::Stack);

    let spacer = Spacer::new().size(16.0).build(&allocator);
    assert_eq!(spacer.view_type, ViewType::Spacer);

    let divider = Divider::new()
        .color(0xFFE5E7EB)
        .thickness(2.0)
        .build(&allocator);
    assert_eq!(divider.view_type, ViewType::Divider);

    let center = Center::new()
        .child(Text::new("Centered Content"))
        .build(&allocator);
    assert_eq!(center.view_type, ViewType::Center);

    let grid = Grid::new().columns(3).spacing(8.0).build(&allocator);
    assert_eq!(grid.view_type, ViewType::Grid);

    // 2. Inputs: Checkbox, Slider
    let checkbox = Checkbox::new()
        .checked(true)
        .disabled(false)
        .on_value_change()
        .test_id("agree_terms")
        .build(&allocator);
    assert_eq!(checkbox.view_type, ViewType::Checkbox);
    assert!(checkbox
        .props
        .iter()
        .any(|p| matches!(p, Property::Checked(true))));

    let slider = Slider::new()
        .value(0.5)
        .range(0.0, 1.0, 0.01)
        .on_slider_change()
        .test_id("volume_slider")
        .build(&allocator);
    assert_eq!(slider.view_type, ViewType::Slider);
    assert!(slider
        .props
        .iter()
        .any(|p| matches!(p, Property::SliderValue(v) if (*v - 0.5).abs() < 1e-4)));

    // 3. Overlays & Display: ProgressBar, Badge, LinearGradient, Icon
    let progress_bar = ProgressBar::new()
        .progress(0.75)
        .color(0xFF10B981)
        .test_id("upload_progress")
        .build(&allocator);
    assert_eq!(progress_bar.view_type, ViewType::ProgressBar);

    let badge = Badge::new("3 Notifications")
        .background_color(0xFFEF4444)
        .text_color(0xFFFFFFFF)
        .build(&allocator);
    assert_eq!(badge.view_type, ViewType::Badge);

    let gradient = LinearGradient::new(0xFF3B82F6, 0xFF8B5CF6)
        .angle(45.0)
        .child(Text::new("Gradient Card"))
        .build(&allocator);
    assert_eq!(gradient.view_type, ViewType::LinearGradient);

    let icon = Icon::new("check-circle")
        .size(32.0)
        .color(0xFF10B981)
        .test_id("status_icon")
        .build(&allocator);
    assert_eq!(icon.view_type, ViewType::Icon);
}

#[test]
fn test_wave2_reconciliation_and_patch_diff() {
    let allocator = NodeIdAllocator::new();
    let root_id = allocator.allocate();

    // Previous state
    let prev = View::new()
        .child(Checkbox::new().checked(false))
        .child(Slider::new().value(0.2))
        .child(ProgressBar::new().progress(0.1))
        .build_with_id(root_id, &allocator);

    // Next state: toggled checkbox, slid slider, advanced progress
    let next = View::new()
        .child(Checkbox::new().checked(true))
        .child(Slider::new().value(0.8))
        .child(ProgressBar::new().progress(0.9))
        .build_with_id(root_id, &allocator);

    let diff_patches = diff_elements(&prev, &next);

    // Verify minimal patch generation: only changed properties are patched
    let has_checked_patch = diff_patches.patches.iter().any(|p| match p {
        Patch::SetProperty {
            property: Property::Checked(true),
            ..
        } => true,
        _ => false,
    });
    assert!(has_checked_patch, "Must emit SetProperty Checked(true)");

    let has_slider_patch = diff_patches.patches.iter().any(|p| match p {
        Patch::SetProperty {
            property: Property::SliderValue(v),
            ..
        } => (*v - 0.8).abs() < 1e-4,
        _ => false,
    });
    assert!(has_slider_patch, "Must emit SetProperty SliderValue(0.8)");

    let has_progress_patch = diff_patches.patches.iter().any(|p| match p {
        Patch::SetProperty {
            property: Property::Progress(p_val),
            ..
        } => (*p_val - 0.9).abs() < 1e-4,
        _ => false,
    });
    assert!(has_progress_patch, "Must emit SetProperty Progress(0.9)");
}
