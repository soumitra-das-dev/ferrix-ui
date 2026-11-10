use ferrix_compiler::codegen::Codegen;
use ferrix_compiler::parser::Parser;

#[test]
fn test_compile_local_rust_module_import() {
    let source = r#"
import { fib, Counter, Status } from "./math.rs";
import { Column, Text } from "ferrix/ui";

export function MathView(): Element {
    const count = useState(5);
    return (
        <Column>
            <Text>Fib: {fib(count.value)}</Text>
        </Column>
    );
}
"#;

    let mut parser = Parser::new(source).expect("Failed to initialize parser");
    let ast = parser.parse_source_file().expect("Failed to parse source");
    let codegen = Codegen::new();
    let rust = codegen.generate(&ast);

    assert!(rust.contains("#[path = \"./math.rs\"]\npub mod math;"));
    assert!(rust.contains("pub use math::{fib, Counter, Status};"));
    assert!(rust.contains("fib(count.get())"));
}

#[test]
fn test_compile_crate_imports() {
    let source = r#"
import { Uuid } from "rust:uuid";
import { Regex } from "rust:regex";
import { json, Value } from "rust:serde_json";
import { Column, Text } from "ferrix/ui";

export function CrateView(): Element {
    const id = useState("init");
    return (
        <Column>
            <Text>{Uuid::new_v4().to_string()}</Text>
        </Column>
    );
}
"#;

    let mut parser = Parser::new(source).expect("Failed to initialize parser");
    let ast = parser.parse_source_file().expect("Failed to parse source");
    let codegen = Codegen::new();
    let rust = codegen.generate(&ast);

    assert!(rust.contains("pub use uuid::{Uuid};"));
    assert!(rust.contains("pub use regex::{Regex};"));
    assert!(rust.contains("pub use serde_json::{json, Value};"));
    assert!(rust.contains("Uuid::new_v4().to_string()"));
}

#[test]
fn test_compile_inline_rust_block() {
    let source = r#"
rust {
    pub fn calculate_score(base: i32, multiplier: i32) -> i32 {
        base * multiplier
    }

    pub struct ScoreConfig {
        pub factor: f64,
    }
}

import { Column, Text } from "ferrix/ui";

export function ScoreView(): Element {
    const score = useState(10);
    return (
        <Column>
            <Text>Score: {calculate_score(score.value, 3)}</Text>
        </Column>
    );
}
"#;

    let mut parser = Parser::new(source).expect("Failed to initialize parser");
    let ast = parser.parse_source_file().expect("Failed to parse source");
    let codegen = Codegen::new();
    let rust = codegen.generate(&ast);

    assert!(rust.contains("pub fn calculate_score(base: i32, multiplier: i32) -> i32"));
    assert!(rust.contains("pub struct ScoreConfig"));
    assert!(rust.contains("calculate_score(score.get(), 3)"));
}
