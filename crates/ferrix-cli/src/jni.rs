//! JNI symbol name mangling according to the Java Native Interface specification.
//!
//! According to Oracle's JNI specification (Resolving Native Method Names):
//! - Every character '_' in Java identifiers is encoded as '_1'.
//! - Package and class separators ('.' or '/') are encoded as '_'.
//! - Unicode characters outside standard ASCII are encoded as '_0xxxx'.
//! - Overloaded method signatures begin with '__' followed by encoded parameter signatures
//!   ('_2' for ';', '_3' for '[').

/// Encodes an identifier component (package segment, class name, or method name)
/// according to JNI escaping rules.
pub fn encode_jni_component(ident: &str) -> String {
    let mut encoded = String::with_capacity(ident.len() * 2);
    for ch in ident.chars() {
        match ch {
            '_' => encoded.push_str("_1"),
            '.' | '/' => encoded.push('_'),
            c if c.is_ascii_alphanumeric() => encoded.push(c),
            c => {
                encoded.push_str(&format!("_0{:04x}", c as u32));
            }
        }
    }
    encoded
}

/// Mangles a full Java/Kotlin package, class, and method name into the corresponding
/// native C symbol exported by Rust or C.
///
/// # Example
/// ```rust
/// use ferrix_cli::jni::mangle_jni_symbol;
///
/// let sym = mangle_jni_symbol("com.example.my_app", "MainActivity", "nativeInit");
/// assert_eq!(sym, "Java_com_example_my_1app_MainActivity_nativeInit");
/// ```
pub fn mangle_jni_symbol(package: &str, class: &str, method: &str) -> String {
    let mut symbol = String::from("Java_");

    // Encode package segments
    for segment in package.split('.') {
        if !segment.is_empty() {
            symbol.push_str(&encode_jni_component(segment));
            symbol.push('_');
        }
    }

    symbol.push_str(&encode_jni_component(class));
    symbol.push('_');
    symbol.push_str(&encode_jni_component(method));

    symbol
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jni_symbol_mangling_standard() {
        let sym = mangle_jni_symbol("com.ferrix.runtime", "FerrixBridge", "nativeInit");
        assert_eq!(sym, "Java_com_ferrix_runtime_FerrixBridge_nativeInit");
    }

    #[test]
    fn test_jni_symbol_mangling_with_underscores() {
        // App ID com.example.my_app contains an underscore in the package segment
        let sym = mangle_jni_symbol("com.example.my_app", "MainActivity", "nativeInit");
        assert_eq!(sym, "Java_com_example_my_1app_MainActivity_nativeInit");
    }

    #[test]
    fn test_jni_symbol_mangling_method_with_underscores() {
        let sym = mangle_jni_symbol("com.example.app", "MainActivity", "on_event_received");
        assert_eq!(sym, "Java_com_example_app_MainActivity_on_1event_1received");
    }

    #[test]
    fn test_jni_symbol_multiple_underscore_segments() {
        let sym = mangle_jni_symbol("org.ferrix_ui.my_test_app", "Main_Activity", "do_work");
        assert_eq!(
            sym,
            "Java_org_ferrix_1ui_my_1test_1app_Main_1Activity_do_1work"
        );
    }
}
