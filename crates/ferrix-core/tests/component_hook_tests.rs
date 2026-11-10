use ferrix_core::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_component_context_use_state_and_use_memo() {
    let mut cx = ComponentContext::new();

    // First render pass
    cx.prepare_render();
    let count = cx.use_state(10);
    let doubled = cx.use_memo(|| count.get() * 2, count.get());
    assert_eq!(count.get(), 10);
    assert_eq!(doubled, 20);

    // Mutate state
    count.set(15);

    // Second render pass
    cx.prepare_render();
    let count = cx.use_state(0); // initial is ignored on subsequent renders
    let doubled = cx.use_memo(|| count.get() * 2, count.get());
    assert_eq!(count.get(), 15);
    assert_eq!(doubled, 30);
}

#[test]
fn test_component_context_use_effect_and_cleanup() {
    let mut cx = ComponentContext::new();
    let cleaned_up = Rc::new(RefCell::new(0));

    // First render pass
    cx.prepare_render();
    {
        let cleaned_up = cleaned_up.clone();
        cx.use_effect(
            move || {
                Some(Box::new(move || {
                    *cleaned_up.borrow_mut() += 1;
                }))
            },
            1, // dep
        );
    }
    assert_eq!(*cleaned_up.borrow(), 0);

    // Second render pass with same dep: effect does not re-run, no cleanup
    cx.prepare_render();
    {
        let cleaned_up = cleaned_up.clone();
        cx.use_effect(
            move || {
                Some(Box::new(move || {
                    *cleaned_up.borrow_mut() += 1;
                }))
            },
            1, // same dep
        );
    }
    assert_eq!(*cleaned_up.borrow(), 0);

    // Third render pass with changed dep: previous cleanup executes!
    cx.prepare_render();
    {
        let cleaned_up = cleaned_up.clone();
        cx.use_effect(
            move || {
                Some(Box::new(move || {
                    *cleaned_up.borrow_mut() += 1;
                }))
            },
            2, // changed dep!
        );
    }
    assert_eq!(*cleaned_up.borrow(), 1);
}

#[test]
fn test_component_context_use_ref() {
    let mut cx = ComponentContext::new();

    cx.prepare_render();
    let mutable_ref = cx.use_ref(100);
    assert_eq!(*mutable_ref.borrow(), 100);
    *mutable_ref.borrow_mut() += 50;

    cx.prepare_render();
    let same_ref = cx.use_ref(0);
    assert_eq!(*same_ref.borrow(), 150);
}
