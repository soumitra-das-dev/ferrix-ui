//! Fine-grained reactive signal type.

use std::cell::RefCell;
use std::rc::Rc;

use crate::graph::{track_signal, trigger_signal, SignalId};

/// Fine-grained reactive state container.
#[derive(Clone, Debug)]
pub struct Signal<T> {
    id: SignalId,
    value: Rc<RefCell<T>>,
}

impl<T> Signal<T> {
    /// Creates a new reactive signal initialized to `initial`.
    pub fn new(initial: T) -> Self {
        Self {
            id: SignalId::next(),
            value: Rc::new(RefCell::new(initial)),
        }
    }

    /// Unique identifier for this signal in the reactive graph.
    pub fn id(&self) -> SignalId {
        self.id
    }

    /// Reads the current value, registering the active observer as a subscriber.
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        track_signal(self.id);
        self.value.borrow().clone()
    }

    /// Accesses the current value by reference without cloning, registering the active observer.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        track_signal(self.id);
        f(&self.value.borrow())
    }

    /// Sets a new value and notifies all subscribed observers.
    pub fn set(&self, new_val: T) {
        *self.value.borrow_mut() = new_val;
        trigger_signal(self.id);
    }

    /// Updates the value in-place via a mutable closure and notifies subscribers.
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        f(&mut self.value.borrow_mut());
        trigger_signal(self.id);
    }
}
