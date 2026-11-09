//! Unit and integration tests for ferrix-fmt (F6).

use ferrix_fmt::format_source;

#[test]
fn test_formatter_idempotency_and_indentation() {
    let unformatted = r#"
import { Column, Text, Button } from "ferrix/ui";

// Header component
export function Header(): Element {
return (
<Column padding={20}>
<Text size={24}>Hello Ferrix</Text>
</Column>
);
}
"#;

    let formatted_pass1 = format_source(unformatted);
    let formatted_pass2 = format_source(&formatted_pass1);

    // Formatter must be strictly idempotent
    assert_eq!(formatted_pass1, formatted_pass2);

    // Check proper indentation was applied
    assert!(formatted_pass1.contains("    return ("));
    assert!(formatted_pass1.contains("        <Column padding={20}>"));
    assert!(formatted_pass1.contains("            <Text size={24}>Hello Ferrix</Text>"));
}

#[test]
fn test_preserves_comments_and_blank_lines() {
    let code = r#"// Global comment
import { Column } from "ferrix/ui";

/* Block comment */
export function App(): Element {
    // Inside function comment
    return (
        <Column>
            {/* JSX comment */}
        </Column>
    );
}
"#;

    let formatted = format_source(code);
    assert!(formatted.contains("// Global comment"));
    assert!(formatted.contains("/* Block comment */"));
    assert!(formatted.contains("// Inside function comment"));
    assert!(formatted.contains("{/* JSX comment */}"));
}
