//! Unit and integration tests for ferrix-forms (F4).

use ferrix_forms::prelude::*;

#[test]
fn test_field_validation_and_touch_states() {
    let email_field = Field::new(
        String::new(),
        vec![required("Email is required"), email("Invalid email format")],
    );

    assert_eq!(email_field.value.get(), "");
    assert!(!email_field.is_dirty.get());
    assert!(!email_field.is_touched.get());
    assert_eq!(email_field.error.get(), None);
    assert_eq!(email_field.version.get(), 0);

    // Initial manual validation fails
    assert!(!email_field.validate());
    assert_eq!(
        email_field.error.get(),
        Some("Email is required".to_string())
    );

    // Input invalid email
    email_field.set_touched(true);
    email_field.set_value("invalid-email".to_string());
    assert!(email_field.is_dirty.get());
    assert_eq!(email_field.version.get(), 1);
    assert_eq!(
        email_field.error.get(),
        Some("Invalid email format".to_string())
    );

    // Input valid email
    email_field.set_value("user@ferrix.dev".to_string());
    assert!(email_field.is_dirty.get());
    assert_eq!(email_field.version.get(), 2);
    assert_eq!(email_field.error.get(), None);

    // Reset restores initial state and bumps version
    email_field.reset();
    assert_eq!(email_field.value.get(), "");
    assert!(!email_field.is_dirty.get());
    assert_eq!(email_field.error.get(), None);
    assert_eq!(email_field.version.get(), 3);
}

#[test]
fn test_form_controller_lifecycle_and_submission() {
    let form = FormController::new();

    let username = Field::new(String::new(), vec![required("Username is required")]);
    let password = Field::new(
        String::new(),
        vec![
            required("Password is required"),
            min_length(6, "Password too short"),
        ],
    );

    form.register_field(&username);
    form.register_field(&password);

    // Attempting submit while invalid should fail and increment submit_count
    let mut submitted = false;
    let err = form.handle_submit(|| {
        submitted = true;
        Ok(())
    });

    assert!(err.is_err());
    assert!(!submitted);
    assert_eq!(form.submit_count.get(), 1);
    assert!(!form.is_valid.get());
    assert_eq!(
        username.error.get(),
        Some("Username is required".to_string())
    );
    assert_eq!(
        password.error.get(),
        Some("Password is required".to_string())
    );

    // Fill valid data
    username.set_value("soumitra".to_string());
    password.set_value("secure_password_123".to_string());

    let success = form.handle_submit(|| {
        submitted = true;
        Ok(())
    });

    assert!(success.is_ok());
    assert!(submitted);
    assert_eq!(form.submit_count.get(), 2);
    assert!(form.is_valid.get());
    assert_eq!(username.error.get(), None);
    assert_eq!(password.error.get(), None);
}
