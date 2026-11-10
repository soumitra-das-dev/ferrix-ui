use ferrix_reactivity::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_signal_basic_get_set() {
    let count = Signal::new(0);
    assert_eq!(count.get(), 0);

    count.set(5);
    assert_eq!(count.get(), 5);

    count.update(|val| *val += 10);
    assert_eq!(count.get(), 15);
}

#[test]
fn test_memo_derived_computation() {
    let a = Signal::new(2);
    let b = Signal::new(3);

    let sum = Memo::new({
        let a = a.clone();
        let b = b.clone();
        move || a.get() + b.get()
    });

    assert_eq!(sum.get(), 5);

    a.set(10);
    assert_eq!(sum.get(), 13);

    b.set(20);
    assert_eq!(sum.get(), 30);
}

#[test]
fn test_batching_deduplication() {
    let a = Signal::new(1);
    let b = Signal::new(2);

    let run_count = Rc::new(RefCell::new(0));

    let _effect = Effect::new({
        let a = a.clone();
        let b = b.clone();
        let run_count = run_count.clone();
        move || {
            let _ = a.get() + b.get();
            *run_count.borrow_mut() += 1;
            None
        }
    });

    // Initial run
    assert_eq!(*run_count.borrow(), 1);

    // Without batching, updating both a and b triggers effect twice
    a.set(10);
    b.set(20);
    assert_eq!(*run_count.borrow(), 3);

    // With batching, updating both a and b triggers effect only ONCE
    batch(|| {
        a.set(100);
        b.set(200);
    });
    assert_eq!(*run_count.borrow(), 4);
}

#[test]
fn test_effect_cleanup_execution() {
    let trigger = Signal::new(0);
    let cleaned_up = Rc::new(RefCell::new(0));

    let effect = Effect::new({
        let trigger = trigger.clone();
        let cleaned_up = cleaned_up.clone();
        move || {
            let _ = trigger.get();
            let cleaned_up = cleaned_up.clone();
            Some(Box::new(move || {
                *cleaned_up.borrow_mut() += 1;
            }))
        }
    });

    // Initial run: no cleanup yet
    assert_eq!(*cleaned_up.borrow(), 0);

    // Trigger update: previous cleanup runs
    trigger.set(1);
    assert_eq!(*cleaned_up.borrow(), 1);

    trigger.set(2);
    assert_eq!(*cleaned_up.borrow(), 2);

    // Explicit dispose
    effect.dispose();
}
