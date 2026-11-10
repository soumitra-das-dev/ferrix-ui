use std::path::Path;

use ferrix_layout::{DimensionValue, FlexDirection};
use ferrix_style_build::{DiagnosticSeverity, StyleCompiler};

#[test]
fn test_parse_global_css_and_shorthands() {
    let css = r#"
        .card {
            background-color: #1E293B;
            padding: 16px 20px;
            margin: 10px;
            border-radius: 8px;
            flex-direction: row;
        }
    "#;

    let compiler = StyleCompiler::new();
    let res = compiler
        .compile(css, Path::new("global.css"))
        .expect("should compile valid css");

    assert_eq!(res.rules.len(), 1);
    let rule = &res.rules[0];
    assert_eq!(rule.selector, ".card");
    assert_eq!(rule.style.background_color, Some(0xFF1E293B));
    assert_eq!(rule.style.flex_direction, Some(FlexDirection::Row));

    // Padding expanded: top/bottom = 16, left/right = 20
    assert_eq!(rule.style.padding_top, Some(DimensionValue::Points(16.0)));
    assert_eq!(
        rule.style.padding_bottom,
        Some(DimensionValue::Points(16.0))
    );
    assert_eq!(rule.style.padding_left, Some(DimensionValue::Points(20.0)));
    assert_eq!(rule.style.padding_right, Some(DimensionValue::Points(20.0)));

    // Margin expanded: all 10
    assert_eq!(rule.style.margin_top, Some(DimensionValue::Points(10.0)));
    assert_eq!(rule.style.margin_left, Some(DimensionValue::Points(10.0)));

    // Border radius: all 8
    assert_eq!(rule.style.border_radius, Some((8.0, 8.0, 8.0, 8.0)));
}

#[test]
fn test_parse_media_queries_and_pseudo_classes() {
    let css = r#"
        .btn {
            background-color: #2563EB;
            color: #FFFFFF;
        }

        .btn:active {
            opacity: 0.8;
        }

        @media (prefers-color-scheme: dark) {
            .btn {
                background-color: #1D4ED8;
            }
        }

        @media (min-width: 640px) {
            .btn {
                padding: 24px;
            }
        }
    "#;

    let compiler = StyleCompiler::new();
    let res = compiler
        .compile(css, Path::new("button.css"))
        .expect("should compile");

    // Rule 0: base .btn
    assert_eq!(res.rules[0].selector, ".btn");
    assert_eq!(res.rules[0].style.background_color, Some(0xFF2563EB));

    // Rule 1: .btn:active
    assert_eq!(res.rules[1].selector, ".btn");
    let active_variant = res.rules[1].style.active.as_ref().expect("has active");
    assert_eq!(active_variant.opacity, Some(0.8));

    // Rule 2: dark mode @media
    assert_eq!(res.rules[2].selector, ".btn");
    let dark_variant = res.rules[2].style.dark.as_ref().expect("has dark");
    assert_eq!(dark_variant.background_color, Some(0xFF1D4ED8));

    // Rule 3: sm breakpoint @media
    assert_eq!(res.rules[3].selector, ".btn");
    let sm_variant = res.rules[3].style.sm.as_ref().expect("has sm");
    assert_eq!(sm_variant.padding_top, Some(DimensionValue::Points(24.0)));
}

#[test]
fn test_css_variables_in_root() {
    let css = r#"
        :root {
            --primary: #2563EB;
            --background: #FFFFFF;
        }

        .box {
            background-color: var(--primary);
        }
    "#;

    let compiler = StyleCompiler::new();
    let res = compiler
        .compile(css, Path::new("theme.css"))
        .expect("should compile");

    assert_eq!(res.variables.get("--primary"), Some(&0xFF2563EB));
    assert_eq!(res.variables.get("--background"), Some(&0xFFFFFFFF));

    assert_eq!(res.rules.len(), 1);
    assert_eq!(
        res.rules[0].style.variables.get("--primary"),
        Some(&"var(--primary)".to_string())
    );
}

#[test]
fn test_css_modules_deterministic_scoping() {
    let css = r#"
        .header {
            font-size: 20px;
            font-weight: bold;
        }

        .content {
            padding: 12px;
        }
    "#;

    let compiler = StyleCompiler::new();
    let res = compiler
        .compile(css, Path::new("card.module.css"))
        .expect("should compile module");

    // Verify class map exists
    assert!(res.class_map.contains_key("header"));
    assert!(res.class_map.contains_key("content"));

    let scoped_header = res.class_map.get("header").unwrap();
    assert!(scoped_header.starts_with("card_header_"));

    let scoped_content = res.class_map.get("content").unwrap();
    assert!(scoped_content.starts_with("card_content_"));

    // Verify rules use scoped selector
    assert_eq!(res.rules[0].selector, format!(".{}", scoped_header));
    assert_eq!(res.rules[1].selector, format!(".{}", scoped_content));
}

#[test]
fn test_unsupported_properties_rejected_with_diagnostics() {
    let css = r#"
        .unsupported {
            float: left;
            cursor: pointer;
        }
    "#;

    let compiler = StyleCompiler::new();
    let err = compiler
        .compile(css, Path::new("test.css"))
        .expect_err("should reject unsupported properties");

    assert_eq!(err.len(), 2);
    assert_eq!(err[0].severity, DiagnosticSeverity::Error);
    assert!(err[0].message.contains("float"));
    assert_eq!(err[1].severity, DiagnosticSeverity::Error);
    assert!(err[1].message.contains("cursor"));
}

#[test]
fn test_unsupported_pseudo_elements_rejected_with_diagnostics() {
    let css = r#"
        .card::before {
            content: "";
        }
    "#;

    let compiler = StyleCompiler::new();
    let err = compiler
        .compile(css, Path::new("test.css"))
        .expect_err("should reject pseudo elements");

    assert_eq!(err.len(), 1);
    assert!(err[0].message.contains("::before"));
}
