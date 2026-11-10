use ferrix_core::prelude::*;
use ferrix_renderer_protocol::{Patch, Property, ViewType};

#[test]
fn test_wave1_primitives_element_construction() {
    let allocator = NodeIdAllocator::new();

    // 1. Image & ImageBackground
    let img = Image::new("https://ferrix.dev/logo.png")
        .resize_mode(1)
        .blur_radius(2.0)
        .tint_color(0xFF00FF00)
        .test_id("logo_img")
        .build(&allocator);
    assert_eq!(img.view_type, ViewType::Image);

    let img_bg = ImageBackground::new("https://ferrix.dev/bg.png")
        .child(Text::new("Overlay content"))
        .build(&allocator);
    assert_eq!(img_bg.view_type, ViewType::Container);
    assert_eq!(img_bg.children.len(), 2); // background image + overlay text

    // 2. Controlled TextInput
    let text_input = TextInput::new()
        .controlled_text(1, "Initial Input")
        .placeholder("Type here...")
        .placeholder_text_color(0x88888888)
        .secure_text_entry(true)
        .keyboard_type(2)
        .multiline(true)
        .selection(0, 5)
        .test_id("controlled_input")
        .build(&allocator);
    assert_eq!(text_input.view_type, ViewType::TextInput);
    assert!(text_input
        .props
        .iter()
        .any(|p| matches!(p, Property::ControlledText { version: 1, .. })));
    assert!(text_input
        .props
        .iter()
        .any(|p| matches!(p, Property::Placeholder(_))));

    // 3. ScrollView
    let scroll = ScrollView::new()
        .horizontal(true)
        .paging_enabled(true)
        .shows_scroll_indicator(false)
        .child(Text::new("Scrollable item"))
        .build(&allocator);
    assert_eq!(scroll.view_type, ViewType::ScrollView);

    // 4. LazyList (FlatList)
    let lazy_list = LazyList::new()
        .item_count(10000)
        .item_height(72.0)
        .on_end_reached()
        .test_id("virtual_feed")
        .build(&allocator);
    assert_eq!(lazy_list.view_type, ViewType::RecyclerView);
    assert!(lazy_list
        .props
        .iter()
        .any(|p| matches!(p, Property::ItemCount(10000))));

    // 5. Switch & ActivityIndicator
    let switch = Switch::new()
        .value(true)
        .track_color(0xFF00FF00, 0xFF444444)
        .thumb_color(0xFFFFFFFF)
        .test_id("dark_mode_switch")
        .build(&allocator);
    assert_eq!(switch.view_type, ViewType::Switch);

    let indicator = ActivityIndicator::new()
        .color(0xFF3B82F6)
        .size(1)
        .animating(true)
        .test_id("spinner")
        .build(&allocator);
    assert_eq!(indicator.view_type, ViewType::ActivityIndicator);

    // 6. Modal, SafeAreaView, RefreshControl, Portal
    let modal = Modal::new()
        .visible(true)
        .animation_type(1)
        .transparent(true)
        .on_request_close()
        .child(Text::new("Modal Dialog"))
        .build(&allocator);
    assert_eq!(modal.view_type, ViewType::Modal);

    let safe_area = SafeAreaView::new()
        .child(Text::new("Safe Screen"))
        .build(&allocator);
    assert_eq!(safe_area.view_type, ViewType::SafeAreaView);

    let refresh = RefreshControl::new()
        .refreshing(false)
        .progress_background_color(0xFFFFFFFF)
        .on_refresh()
        .build(&allocator);
    assert_eq!(refresh.view_type, ViewType::RefreshControl);

    let portal = Portal::new()
        .child(Text::new("Toast Overlay"))
        .build(&allocator);
    assert_eq!(portal.view_type, ViewType::Portal);

    // 7. Compat Touchables
    let touchable_op = TouchableOpacity::new()
        .active_opacity(0.7)
        .child(Text::new("Touch Opacity Button"))
        .build(&allocator);
    assert_eq!(touchable_op.view_type, ViewType::Pressable);

    let insets = use_safe_area_insets();
    assert!(insets.top > 0.0);
}

#[test]
fn test_wave1_reconciliation_and_patch_diff() {
    let allocator = NodeIdAllocator::new();
    let root_id = allocator.allocate();

    // Initial Tree
    let prev = View::new()
        .child(
            TextInput::new()
                .controlled_text(1, "First query")
                .placeholder("Search..."),
        )
        .child(Switch::new().value(false))
        .build_with_id(root_id, &allocator);

    let mount_patches = generate_mount_patches(&prev);
    assert!(!mount_patches.is_empty());

    // Updated Tree: text edited, switch toggled
    let next = View::new()
        .child(
            TextInput::new()
                .controlled_text(2, "Second query")
                .placeholder("Search..."),
        )
        .child(Switch::new().value(true))
        .build_with_id(root_id, &allocator);

    let diff_patches = diff_elements(&prev, &next);

    // Verify minimal patch generation: only changed properties are patched
    let has_controlled_text_patch = diff_patches.patches.iter().any(|p| match p {
        Patch::SetProperty {
            property:
                Property::ControlledText {
                    version: 2,
                    ref text,
                },
            ..
        } => text == "Second query",
        _ => false,
    });
    assert!(
        has_controlled_text_patch,
        "Must generate SetProperty with updated controlled text"
    );

    let has_switch_value_patch = diff_patches.patches.iter().any(|p| match p {
        Patch::SetProperty {
            property: Property::SwitchValue(true),
            ..
        } => true,
        _ => false,
    });
    assert!(
        has_switch_value_patch,
        "Must generate SetProperty with updated switch value"
    );
}
