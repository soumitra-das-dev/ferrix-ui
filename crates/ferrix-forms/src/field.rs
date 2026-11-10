//! Reactive form field state with monotonic cursor synchronization.

use crate::validation::Validator;
use ferrix_reactivity::Signal;
use std::cell::RefCell;
use std::rc::Rc;

/// Controlled reactive form field.
#[derive(Clone)]
pub struct Field<T: Clone + PartialEq + 'static> {
    pub value: Signal<T>,
    pub error: Signal<Option<String>>,
    pub is_dirty: Signal<bool>,
    pub is_touched: Signal<bool>,
    pub version: Signal<u64>,
    initial_value: T,
    validators: Rc<RefCell<Vec<Validator<T>>>>,
}

impl<T: Clone + PartialEq + 'static> Field<T> {
    /// Creates a new reactive field with initial value and optional validators.
    pub fn new(initial: T, validators: Vec<Validator<T>>) -> Self {
        Self {
            value: Signal::new(initial.clone()),
            error: Signal::new(None),
            is_dirty: Signal::new(false),
            is_touched: Signal::new(false),
            version: Signal::new(0),
            initial_value: initial,
            validators: Rc::new(RefCell::new(validators)),
        }
    }

    /// Updates field value from user input: increments version counter, marks dirty, and runs validators if touched.
    pub fn set_value(&self, new_val: T) {
        let is_dirty = new_val != self.initial_value;
        self.is_dirty.set(is_dirty);
        self.value.set(new_val);
        self.version.update(|v| *v += 1);

        if self.is_touched.get() {
            self.validate();
        }
    }

    /// Sets field touch state (e.g. on blur). Triggers validation if touched.
    pub fn set_touched(&self, touched: bool) {
        self.is_touched.set(touched);
        if touched {
            self.validate();
        }
    }

    /// Runs all validators against current value and updates the error signal.
    pub fn validate(&self) -> bool {
        let current = self.value.get();
        for val_fn in self.validators.borrow().iter() {
            if let Err(msg) = val_fn(&current) {
                self.error.set(Some(msg));
                return false;
            }
        }
        self.error.set(None);
        true
    }

    /// Resets field to initial value and clears errors and dirty flags.
    pub fn reset(&self) {
        self.value.set(self.initial_value.clone());
        self.is_dirty.set(false);
        self.is_touched.set(false);
        self.error.set(None);
        self.version.update(|v| *v += 1);
    }
}
