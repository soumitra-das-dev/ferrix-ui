//! Reactive side-effects with automated dependency tracking and cleanup.

use std::cell::RefCell;
use std::rc::Rc;

use crate::graph::{register_observer, unregister_observer, with_observer, ObserverId};

pub type CleanupFn = Box<dyn FnOnce()>;

/// A side-effect that automatically tracks accessed signals and re-executes when dependencies mutate.
pub struct Effect {
    observer_id: ObserverId,
    disposed: RefCell<bool>,
}

impl Effect {
    /// Creates and immediately runs a reactive effect with optional cleanup callback.
    pub fn new<F>(effect: F) -> Self
    where
        F: FnMut() -> Option<CleanupFn> + 'static,
    {
        let observer_id = ObserverId::next();
        let cleanup_slot: Rc<RefCell<Option<CleanupFn>>> = Rc::new(RefCell::new(None));

        let effect_cell = Rc::new(RefCell::new(effect));
        let cleanup_slot_clone = cleanup_slot.clone();

        let run_effect = Rc::new(move || {
            // Run previous cleanup if any
            if let Some(cleanup) = cleanup_slot_clone.borrow_mut().take() {
                cleanup();
            }

            // Run effect while tracking dependencies
            let new_cleanup = with_observer(observer_id, || {
                let mut ef = effect_cell.borrow_mut();
                ef()
            });

            *cleanup_slot_clone.borrow_mut() = new_cleanup;
        });

        // Register observer callback
        register_observer(observer_id, run_effect.clone());

        // Initial execution
        run_effect();

        Self {
            observer_id,
            disposed: RefCell::new(false),
        }
    }

    /// Disposes the effect, running any registered cleanup and unregistering from the reactive graph.
    pub fn dispose(&self) {
        if !*self.disposed.borrow() {
            *self.disposed.borrow_mut() = true;
            unregister_observer(self.observer_id);
        }
    }
}

impl Drop for Effect {
    fn drop(&mut self) {
        self.dispose();
    }
}
