use ferrix_renderer_protocol::Patch;
use ferrix_test::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_queries_find_elements_by_text_testid_and_role() {
    let renderer = render(|cx| {
        Column::new()
            .padding(16.0)
            .child(
                Text::new("Welcome to Ferrix")
                    .accessibility_role("header")
                    .content_description("App Header"),
            )
            .child(
                Button::new("Submit")
                    .test_id("submit-btn")
                    .content_description("Submit Action"),
            )
            .build_with_id(NodeId::new(1), cx.allocator())
    });

    // 1. Text query
    let title_node = renderer
        .get_by_text("Welcome to Ferrix")
        .expect("Should find title");
    assert_eq!(title_node.text(), Some("Welcome to Ferrix"));

    // 2. TestId query
    let btn_node = renderer
        .get_by_test_id("submit-btn")
        .expect("Should find submit button");
    assert_eq!(btn_node.test_id(), Some("submit-btn"));

    // 3. Role query
    let header_node = renderer.get_by_role("header").expect("Should find header");
    assert_eq!(header_node.id(), title_node.id());

    // 4. Label query
    let action_node = renderer
        .get_by_label("Submit Action")
        .expect("Should find by label");
    assert_eq!(action_node.id(), btn_node.id());
}

#[test]
fn test_query_errors_on_missing_or_duplicate_elements() {
    let renderer = render(|cx| {
        Column::new()
            .child(Text::new("Duplicate"))
            .child(Text::new("Duplicate"))
            .build_with_id(NodeId::new(1), cx.allocator())
    });

    // Missing text
    match renderer.get_by_text("NonExistent") {
        Err(QueryError::TextNotFound(msg)) => assert_eq!(msg, "NonExistent"),
        other => panic!("Expected TextNotFound, got {:?}", other),
    }

    // Duplicate text
    match renderer.get_by_text("Duplicate") {
        Err(QueryError::MultipleWithText(msg, count)) => {
            assert_eq!(msg, "Duplicate");
            assert_eq!(count, 2);
        }
        other => panic!("Expected MultipleWithText, got {:?}", other),
    }
}

#[test]
fn test_interactive_counter_event_simulation_and_single_patch_contract() {
    let count_signal = Rc::new(RefCell::new(0));
    let count_for_render = count_signal.clone();
    let count_for_click = count_signal.clone();

    let mut renderer = render(move |cx| {
        let current_count = *count_for_render.borrow();
        Column::new()
            .padding(20.0)
            .child(Text::new("Counter Title"))
            .child(Text::new(format!("Count: {}", current_count)))
            .child(Button::new("Increment").test_id("inc-btn"))
            .build_with_id(NodeId::new(1), cx.allocator())
    });

    let inc_button_id = renderer
        .get_by_test_id("inc-btn")
        .expect("Found inc button")
        .id();

    // Register button click handler
    renderer.on_click(inc_button_id, move || {
        *count_for_click.borrow_mut() += 1;
    });

    // Assert initial state
    assert!(renderer.get_by_text("Count: 0").is_ok());

    // Fire synthetic click
    fire_event::press(&mut renderer, inc_button_id);

    // Verify state updated in tree
    assert!(renderer.get_by_text("Count: 1").is_ok());

    // Architecture A invariant: Exactly 1 SetText patch produced
    renderer.assert_single_patch(|patch| match patch {
        Patch::SetText { node_id, text } => {
            assert_eq!(*node_id, NodeId::new(3));
            assert_eq!(text, "Count: 1");
        }
        other => panic!("Expected SetText patch, got {:?}", other),
    });
}

#[test]
fn test_text_input_event_simulation() {
    let input_value = Rc::new(RefCell::new(String::from("initial")));
    let input_for_render = input_value.clone();
    let input_for_change = input_value.clone();

    let mut renderer = render(move |cx| {
        Column::new()
            .child(
                TextInput::new()
                    .test_id("username-input")
                    .value(input_for_render.borrow().clone()),
            )
            .build_with_id(NodeId::new(1), cx.allocator())
    });

    let input_id = renderer
        .get_by_test_id("username-input")
        .expect("Found input")
        .id();

    renderer.on_text_change(input_id, move |new_text| {
        *input_for_change.borrow_mut() = new_text;
    });

    fire_event::change_text(&mut renderer, input_id, "ferrix_user");

    assert_eq!(*input_value.borrow(), "ferrix_user");
}

#[test]
fn test_snapshot_formatting() {
    let renderer = render(|cx| {
        Column::new()
            .child(
                Text::new("Snapshot Test")
                    .accessibility_role("heading")
                    .test_id("test-header"),
            )
            .build_with_id(NodeId::new(1), cx.allocator())
    });

    let snapshot = renderer.to_snapshot();
    assert!(snapshot.contains("<Column>"));
    assert!(snapshot.contains("testID=\"test-header\""));
    assert!(snapshot.contains("accessibilityRole=\"heading\""));
    assert!(snapshot.contains("Snapshot Test"));
    assert!(snapshot.contains("</Text>"));
    assert!(snapshot.contains("</Column>"));
}

#[test]
fn test_fake_timers_advance_and_interval() {
    let timers = FakeTimers::new();
    let timeout_fired = Rc::new(RefCell::new(false));
    let timeout_clone = timeout_fired.clone();

    timers.set_timeout(500, move || {
        *timeout_clone.borrow_mut() = true;
    });

    let interval_counter = Rc::new(RefCell::new(0));
    let interval_clone = interval_counter.clone();

    timers.set_interval(200, move || {
        *interval_clone.borrow_mut() += 1;
    });

    assert_eq!(timers.now_ms(), 0);
    assert_eq!(*timeout_fired.borrow(), false);
    assert_eq!(*interval_counter.borrow(), 0);

    // Advance 300ms: timeout not yet, interval fired once (at 200ms)
    timers.advance_timers_by_time(300);
    assert_eq!(timers.now_ms(), 300);
    assert_eq!(*timeout_fired.borrow(), false);
    assert_eq!(*interval_counter.borrow(), 1);

    // Advance another 250ms (total 550ms): timeout fired (at 500ms), interval fired twice (at 400ms)
    timers.advance_timers_by_time(250);
    assert_eq!(timers.now_ms(), 550);
    assert_eq!(*timeout_fired.borrow(), true);
    assert_eq!(*interval_counter.borrow(), 2);
}
