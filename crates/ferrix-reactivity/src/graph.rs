//! Dependency tracking graph and runtime observer engine for fine-grained reactivity.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_OBSERVER_ID: AtomicU64 = AtomicU64::new(1);
static NEXT_SIGNAL_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalId(pub u64);

impl SignalId {
    pub fn next() -> Self {
        Self(NEXT_SIGNAL_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObserverId(pub u64);

impl ObserverId {
    pub fn next() -> Self {
        Self(NEXT_OBSERVER_ID.fetch_add(1, Ordering::Relaxed))
    }
}

thread_local! {
    static RUNTIME: RefCell<ReactiveRuntime> = RefCell::new(ReactiveRuntime::new());
}

pub struct ReactiveRuntime {
    observer_stack: Vec<ObserverId>,
    // SignalId -> Set of Observers subscribing to it
    subscribers: HashMap<SignalId, HashSet<ObserverId>>,
    // Observer callbacks
    observers: HashMap<ObserverId, Rc<dyn Fn()>>,
    // Batching state
    batch_depth: usize,
    dirty_observers: HashSet<ObserverId>,
}

impl ReactiveRuntime {
    fn new() -> Self {
        Self {
            observer_stack: Vec::new(),
            subscribers: HashMap::new(),
            observers: HashMap::new(),
            batch_depth: 0,
            dirty_observers: HashSet::new(),
        }
    }
}

/// Registers the currently active observer as a subscriber to the given signal.
pub fn track_signal(signal_id: SignalId) {
    RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        if let Some(&observer_id) = rt.observer_stack.last() {
            rt.subscribers
                .entry(signal_id)
                .or_default()
                .insert(observer_id);
        }
    });
}

/// Triggers notifications for all subscribers of the given signal.
pub fn trigger_signal(signal_id: SignalId) {
    let mut to_run = Vec::new();
    RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        if let Some(subs) = rt.subscribers.get(&signal_id).cloned() {
            for sub in subs {
                if rt.batch_depth > 0 {
                    rt.dirty_observers.insert(sub);
                } else if let Some(cb) = rt.observers.get(&sub).cloned() {
                    to_run.push(cb);
                }
            }
        }
    });

    for cb in to_run {
        cb();
    }
}

/// Executes a closure with an active observer on the stack.
pub fn with_observer<R>(observer_id: ObserverId, f: impl FnOnce() -> R) -> R {
    RUNTIME.with(|rt| {
        rt.borrow_mut().observer_stack.push(observer_id);
    });

    let result = f();

    RUNTIME.with(|rt| {
        rt.borrow_mut().observer_stack.pop();
    });

    result
}

/// Registers an observer callback.
pub fn register_observer(observer_id: ObserverId, callback: Rc<dyn Fn()>) {
    RUNTIME.with(|rt| {
        rt.borrow_mut().observers.insert(observer_id, callback);
    });
}

/// Unregisters an observer.
pub fn unregister_observer(observer_id: ObserverId) {
    RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        rt.observers.remove(&observer_id);
        for subs in rt.subscribers.values_mut() {
            subs.remove(&observer_id);
        }
    });
}

/// Executes a block with updates batched. Subscribers only re-run once after the batch completes.
pub fn batch<R>(f: impl FnOnce() -> R) -> R {
    RUNTIME.with(|rt| {
        rt.borrow_mut().batch_depth += 1;
    });

    let result = f();

    let mut to_run = Vec::new();
    RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        rt.batch_depth = rt.batch_depth.saturating_sub(1);
        if rt.batch_depth == 0 {
            let dirty: Vec<ObserverId> = rt.dirty_observers.drain().collect();
            for obs_id in dirty {
                if let Some(cb) = rt.observers.get(&obs_id) {
                    to_run.push(cb.clone());
                }
            }
        }
    });

    for cb in to_run {
        cb();
    }

    result
}
