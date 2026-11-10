// Example headless component test using ferrix-test
use ferrix_renderer_protocol::Patch;
use ferrix_test::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn counter_app(cx: &mut ComponentContext, count: i32) -> Element {
    Column::new()
        .padding(16.0)
        .spacing(12.0)
        .child(Text::new("Counter App").accessibility_role("heading"))
        .child(Text::new(format!("Count: {}", count)).test_id("count-display"))
        .child(Button::new("Increment").test_id("increment-btn"))
        .build_with_id(NodeId::new(1), cx.allocator())
}

fn main() {
    println!("Running headless component test...");

    let count_state = Rc::new(RefCell::new(0));
    let count_render = count_state.clone();
    let count_click = count_state.clone();

    let mut renderer = render(move |cx| {
        counter_app(cx, *count_render.borrow())
    });

    let btn_id = renderer
        .get_by_test_id("increment-btn")
        .expect("Found button")
        .id();

    renderer.on_click(btn_id, move || {
        *count_click.borrow_mut() += 1;
    });

    // 1. Initial snapshot & assertions
    assert!(renderer.get_by_text("Count: 0").is_ok());
    println!("Initial snapshot:\n{}", renderer.to_snapshot());

    // 2. Fire click event
    fire_event::press(&mut renderer, btn_id);

    // 3. Verify text updated and single patch emitted
    assert!(renderer.get_by_text("Count: 1").is_ok());
    renderer.assert_single_patch(|p| {
        assert!(matches!(p, Patch::SetText { .. }));
    });

    println!("Component test passed successfully!");
}
