//! # ferrix-forms
//!
//! Controlled form and field management with monotonic cursor synchronization for Ferrix.

pub mod field;
pub mod form;
pub mod validation;

pub use field::Field;
pub use form::FormController;
pub use validation::{email, max_length, min_length, required, Validator};

pub mod prelude {
    pub use crate::field::Field;
    pub use crate::form::FormController;
    pub use crate::validation::{email, max_length, min_length, required};
}
