//! Form controller coordinating field validations and submissions.

use crate::field::Field;
use ferrix_reactivity::Signal;
use std::cell::RefCell;
use std::rc::Rc;

type FieldValidator = Box<dyn Fn() -> bool>;

/// Form controller coordinating validation, submit states, and submission lifecycle.
#[derive(Clone)]
pub struct FormController {
    field_validators: Rc<RefCell<Vec<FieldValidator>>>,
    pub is_valid: Signal<bool>,
    pub is_submitting: Signal<bool>,
    pub submit_count: Signal<u32>,
}

impl Default for FormController {
    fn default() -> Self {
        Self::new()
    }
}

impl FormController {
    /// Creates a new FormController.
    pub fn new() -> Self {
        Self {
            field_validators: Rc::new(RefCell::new(Vec::new())),
            is_valid: Signal::new(true),
            is_submitting: Signal::new(false),
            submit_count: Signal::new(0),
        }
    }

    /// Registers a field's validation routine with the form.
    pub fn register_field<T: Clone + PartialEq + 'static>(&self, field: &Field<T>) {
        let field_clone = field.clone();
        self.field_validators.borrow_mut().push(Box::new(move || {
            field_clone.set_touched(true);
            field_clone.validate()
        }));
    }

    /// Validates all registered fields. Returns true if all fields are valid.
    pub fn validate_all(&self) -> bool {
        let mut all_valid = true;
        for validator in self.field_validators.borrow().iter() {
            if !validator() {
                all_valid = false;
            }
        }
        self.is_valid.set(all_valid);
        all_valid
    }

    /// Executes submission lifecycle: validates all fields, increments submit count,
    /// sets is_submitting, and invokes the submission handler.
    pub fn handle_submit<F: FnOnce() -> Result<(), String>>(
        &self,
        on_submit: F,
    ) -> Result<(), String> {
        self.submit_count.update(|c| *c += 1);
        let valid = self.validate_all();
        if !valid {
            return Err("Form validation failed".to_string());
        }

        self.is_submitting.set(true);
        let result = on_submit();
        self.is_submitting.set(false);
        result
    }
}
