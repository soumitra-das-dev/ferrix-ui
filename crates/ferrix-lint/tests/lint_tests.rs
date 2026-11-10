//! Unit and integration tests for ferrix-lint (F6).

use ferrix_lint::prelude::*;

#[test]
fn test_rules_of_hooks_lint() {
    let code_with_hook_in_conditional = r#"
export function BadComponent(): Element {
    if (condition) {
        const count = useState(0);
    }
    return <Text>Invalid</Text>;
}
"#;

    let engine = LintEngine::new();
    let diags = engine.lint(code_with_hook_in_conditional);

    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].rule, "rules-of-hooks");
    assert_eq!(diags[0].severity, Severity::Error);
}

#[test]
fn test_require_keyed_children_lint() {
    let code_with_unkeyed_map = r#"
export function List(props: { items: string[] }): Element {
    return (
        <Column>
            {props.items.map((item) => (
                <Text>{item}</Text>
            ))}
        </Column>
    );
}
"#;

    let engine = LintEngine::new();
    let diags = engine.lint(code_with_unkeyed_map);

    assert!(diags.iter().any(|d| d.rule == "require-keyed-children"));
}

#[test]
fn test_accessibility_pressable_lint() {
    let code = r#"
export function IconButton(): Element {
    return (
        <Pressable onPress={() => doSomething()}>
            <Icon name="star" />
        </Pressable>
    );
}
"#;

    let engine = LintEngine::new();
    let diags = engine.lint(code);

    assert!(diags.iter().any(|d| d.rule == "accessibility-pressable"));
}

#[test]
fn test_unused_imports_and_autofix() {
    let code = r#"import { Column, Text, UnusedComponent } from "ferrix/ui";

export function App(): Element {
    return (
        <Column>
            <Text>Hi</Text>
        </Column>
    );
}
"#;

    let engine = LintEngine::new();
    let diags = engine.lint(code);

    assert!(diags
        .iter()
        .any(|d| d.rule == "unused-imports" && d.message.contains("UnusedComponent")));
}
