use ferrix_compiler::{compile_fsx_to_rust, Parser};

const GOLDEN_COUNTER_FSX: &str = r#"
import { Column, Text, Button } from "ferrix/ui";

type CounterProps = { initial: i32 };

export function Counter(props: CounterProps): Element {
    const count = useState(props.initial);

    return (
        <Column padding={20} spacing={12}>
            <Text size={24}>Ferrix Counter</Text>
            <Text>Count: {count.value}</Text>
            <Button onClick={() => count.set(count.value + 1)}>
                Increment
            </Button>
        </Column>
    );
}
"#;

#[test]
fn test_golden_counter_ast_parsing() {
    let mut parser = Parser::new(GOLDEN_COUNTER_FSX).expect("Failed to create parser");
    let ast = parser
        .parse_source_file()
        .expect("Failed to parse source file");

    assert_eq!(ast.imports.len(), 1);
    assert_eq!(ast.imports[0].items, vec!["Column", "Text", "Button"]);
    assert_eq!(ast.imports[0].source, "ferrix/ui");

    assert_eq!(ast.types.len(), 1);
    assert_eq!(ast.types[0].name, "CounterProps");
    assert_eq!(ast.types[0].fields.len(), 1);
    assert_eq!(ast.types[0].fields[0].name, "initial");

    assert_eq!(ast.components.len(), 1);
    let comp = &ast.components[0];
    assert_eq!(comp.name, "Counter");
    assert_eq!(comp.statements.len(), 1);
    assert_eq!(comp.return_elem.tag_name, "Column");
    assert_eq!(comp.return_elem.children.len(), 3);
}

#[test]
fn test_golden_counter_code_generation() {
    let rust_code =
        compile_fsx_to_rust(GOLDEN_COUNTER_FSX, "component.fsx").expect("Compilation failed");

    println!("GENERATED RUST:\n{}", rust_code);

    assert!(rust_code.contains("pub struct CounterProps {"));
    assert!(rust_code.contains("pub initial: i32,"));
    assert!(rust_code
        .contains("pub fn Counter(cx: &mut ComponentContext, props: CounterProps) -> Element {"));
    assert!(rust_code.contains("let count = cx.use_state(props.initial);"));
    assert!(rust_code.contains("Column::new()"));
    assert!(rust_code.contains(".padding(20.0)"));
    assert!(rust_code.contains(".spacing(12.0)"));
    assert!(rust_code.contains("Text::new(\"Ferrix Counter\")"));
    assert!(rust_code.contains(".size(24.0)"));
    assert!(rust_code.contains("Text::new(format!(\"Count: {}\", count.get()))"));
    assert!(rust_code.contains("Button::new(\"Increment\")"));
}

#[test]
fn test_hook_ordering_diagnostic() {
    let invalid_fsx = r#"
export function Invalid(props: Props): Element {
    let x = 10;
    const count = useState(x);
    return <Text>Invalid</Text>;
}
"#;

    let res = compile_fsx_to_rust(invalid_fsx, "invalid.fsx");
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("error[FSX0002]"));
    assert!(err.contains("called after non-hook statements"));
}

#[test]
fn test_wave0_primitives_fsx_compilation() {
    let view_fsx = include_str!("../../../examples/primitives/view.fsx");
    let view_rust = compile_fsx_to_rust(view_fsx, "view.fsx").expect("Failed to compile view.fsx");
    assert!(view_rust.contains("pub fn ViewExample"));
    assert!(view_rust.contains("View::new()"));

    let text_fsx = include_str!("../../../examples/primitives/text.fsx");
    let text_rust = compile_fsx_to_rust(text_fsx, "text.fsx").expect("Failed to compile text.fsx");
    assert!(text_rust.contains("pub fn TextExample"));
    assert!(text_rust.contains("Text::new"));

    let pressable_fsx = include_str!("../../../examples/primitives/pressable.fsx");
    let pressable_rust = compile_fsx_to_rust(pressable_fsx, "pressable.fsx")
        .expect("Failed to compile pressable.fsx");
    assert!(pressable_rust.contains("pub fn PressableExample"));
    assert!(pressable_rust.contains("Pressable::new()"));
}
