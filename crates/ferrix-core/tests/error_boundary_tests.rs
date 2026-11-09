use ferrix_core::prelude::*;

#[test]
fn test_error_boundary_catches_panic_and_renders_fallback() {
    let mut cx = ComponentContext::with_start_id(2);

    // Component that deliberately panics during render
    let element = render_with_error_boundary(
        &mut cx,
        |_cx| {
            panic!("Critical render calculation error!");
        },
        |err: &ErrorInfo, cx: &mut ComponentContext| {
            Column::new()
                .child(Text::new("ErrorBoundary Caught Failure"))
                .child(Text::new(format!("Error message: {}", err.message)))
                .build_with_id(NodeId::new(1), cx.allocator())
        },
    );

    assert_eq!(element.children.len(), 2);
    assert_eq!(
        element.children[0].text,
        Some("ErrorBoundary Caught Failure".to_string())
    );
    assert!(element.children[1]
        .text
        .as_ref()
        .unwrap()
        .contains("Critical render calculation error!"));
}

#[test]
fn test_error_boundary_normal_render_passes_through() {
    let mut cx = ComponentContext::with_start_id(2);

    let element = render_with_error_boundary(
        &mut cx,
        |cx| {
            Column::new()
                .child(Text::new("Healthy Component"))
                .build_with_id(NodeId::new(1), cx.allocator())
        },
        |_err, cx| {
            Text::new("Fallback should not be called").build_with_id(NodeId::new(1), cx.allocator())
        },
    );

    assert_eq!(element.children.len(), 1);
    assert_eq!(
        element.children[0].text,
        Some("Healthy Component".to_string())
    );
}
