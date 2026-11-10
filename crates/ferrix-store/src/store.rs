//! Fine-grained reactive Store engine with selector subscriptions and persistence.

use ferrix_reactivity::Signal;
use ferrix_storage::KvStore;
use serde::{de::DeserializeOwned, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

type Subscriber<S> = Box<dyn Fn(&S)>;

/// Fine-grained global state container.
#[derive(Clone)]
pub struct Store<S: 'static> {
    state: Rc<RefCell<S>>,
    subscribers: Rc<RefCell<Vec<Subscriber<S>>>>,
}

impl<S: 'static> Store<S> {
    /// Creates a new reactive store with initial state.
    pub fn new(initial: S) -> Self {
        Self {
            state: Rc::new(RefCell::new(initial)),
            subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Reads a clone of the current complete state.
    pub fn get_state(&self) -> S
    where
        S: Clone,
    {
        self.state.borrow().clone()
    }

    /// Mutates the state and notifies only dependent slice subscribers whose selected value changed.
    pub fn update<F: FnOnce(&mut S)>(&self, updater: F) {
        updater(&mut self.state.borrow_mut());
        let current_state = self.state.borrow();
        for sub in self.subscribers.borrow().iter() {
            sub(&current_state);
        }
    }

    /// Creates a fine-grained reactive `Signal<T>` selecting a slice of state.
    /// Only triggers when `selector(&state)` produces a value distinct from the previous value.
    pub fn select<T, F>(&self, selector: F) -> Signal<T>
    where
        T: Clone + PartialEq + 'static,
        F: Fn(&S) -> T + 'static,
    {
        let initial_val = selector(&self.state.borrow());
        let signal = Signal::new(initial_val.clone());

        let sig_clone = signal.clone();
        let prev_val = Rc::new(RefCell::new(initial_val));

        self.subscribers
            .borrow_mut()
            .push(Box::new(move |new_state: &S| {
                let next_val = selector(new_state);
                if next_val != *prev_val.borrow() {
                    *prev_val.borrow_mut() = next_val.clone();
                    sig_clone.set(next_val);
                }
            }));

        signal
    }

    /// Attaches persistence middleware that rehydrates from and persists to `ferrix-storage`.
    pub fn with_persistence(self, storage_key: &'static str, storage: KvStore) -> Self
    where
        S: Clone + Serialize + DeserializeOwned + 'static,
    {
        // Rehydrate if exists
        if let Ok(Some(restored)) = storage.get_json::<S>(storage_key) {
            *self.state.borrow_mut() = restored;
        }

        // Persist on updates
        let key = storage_key.to_string();
        let store_clone = storage;
        self.subscribers
            .borrow_mut()
            .push(Box::new(move |new_state: &S| {
                let _ = store_clone.set_json(&key, new_state);
            }));

        self
    }
}
