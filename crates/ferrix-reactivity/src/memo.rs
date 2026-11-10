//! Derived reactive computation (Memo).

use std::rc::Rc;

use crate::graph::{register_observer, with_observer, ObserverId};
use crate::signal::Signal;

/// Memoized reactive computation that re-evaluates only when upstream dependencies change.
#[derive(Clone)]
pub struct Memo<T> {
    inner_signal: Signal<T>,
    _observer_id: ObserverId,
}

impl<T: 'static> Memo<T> {
    /// Creates a new derived memo from a computation closure.
    pub fn new(compute: impl Fn() -> T + 'static) -> Self
    where
        T: Clone,
    {
        let observer_id = ObserverId::next();
        let compute = Rc::new(compute);

        // Initial computation while tracking dependencies
        let initial_val = with_observer(observer_id, || compute());
        let inner_signal = Signal::new(initial_val);

        // Register update callback for whenever upstream dependencies change
        let signal_clone = inner_signal.clone();
        let compute_clone = compute;
        register_observer(
            observer_id,
            Rc::new(move || {
                let updated = with_observer(observer_id, || compute_clone());
                signal_clone.set(updated);
            }),
        );

        Self {
            inner_signal,
            _observer_id: observer_id,
        }
    }

    /// Reads the memoized value, tracking dependencies.
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.inner_signal.get()
    }
}
