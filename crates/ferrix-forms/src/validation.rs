//! Built-in form validation rules and helpers.

/// Validation rule function signature.
pub type Validator<T> = Box<dyn Fn(&T) -> Result<(), String>>;

/// Requires a string to be non-empty after trimming.
pub fn required(message: &'static str) -> Validator<String> {
    Box::new(move |val: &String| {
        if val.trim().is_empty() {
            Err(message.to_string())
        } else {
            Ok(())
        }
    })
}

/// Validates minimum string length.
pub fn min_length(min: usize, message: &'static str) -> Validator<String> {
    Box::new(move |val: &String| {
        if val.len() < min {
            Err(message.to_string())
        } else {
            Ok(())
        }
    })
}

/// Validates maximum string length.
pub fn max_length(max: usize, message: &'static str) -> Validator<String> {
    Box::new(move |val: &String| {
        if val.len() > max {
            Err(message.to_string())
        } else {
            Ok(())
        }
    })
}

/// Validates basic email formatting.
pub fn email(message: &'static str) -> Validator<String> {
    Box::new(move |val: &String| {
        let trimmed = val.trim();
        if let Some((local, domain)) = trimmed.split_once('@') {
            if !local.is_empty()
                && domain.contains('.')
                && !domain.starts_with('.')
                && !domain.ends_with('.')
            {
                return Ok(());
            }
        }
        Err(message.to_string())
    })
}
