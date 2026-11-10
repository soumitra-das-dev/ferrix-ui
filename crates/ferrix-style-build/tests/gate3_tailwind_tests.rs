use std::collections::HashSet;

use ferrix_layout::{AlignItems, DimensionValue, Display, FlexDirection, JustifyContent, Position};
use ferrix_renderer_protocol::{NodeId, Patch, PatchBatch};
use ferrix_style::{CascadeEngine, ReactiveStyleResolver};
use ferrix_style_build::tailwind::{scan_candidates, TailwindCompiler};

#[test]
fn test_gate3_candidate_scanner() {
    let fsx_source = r#"
        fn Card(props: CardProps) -> Element {
            let active_classes = cx("p-4 bg-white rounded-lg shadow-md", is_active && "border-blue-500");
            <Column class="flex flex-col w-full items-center">
                <Text className="text-xl font-bold text-slate-900 tracking-tight">
                    "Card Title"
                </Text>
                <Button class={active_classes} className="active:bg-blue-600 dark:bg-slate-800">
                    "Click Me"
                </Button>
            </Column>
        }
    "#;

    let safelist = &["m-2", "text-sm"];
    let candidates = scan_candidates(fsx_source, safelist);

    assert!(candidates.contains("flex"));
    assert!(candidates.contains("flex-col"));
    assert!(candidates.contains("w-full"));
    assert!(candidates.contains("items-center"));
    assert!(candidates.contains("text-xl"));
    assert!(candidates.contains("font-bold"));
    assert!(candidates.contains("text-slate-900"));
    assert!(candidates.contains("tracking-tight"));
    assert!(candidates.contains("p-4"));
    assert!(candidates.contains("bg-white"));
    assert!(candidates.contains("rounded-lg"));
    assert!(candidates.contains("shadow-md"));
    assert!(candidates.contains("border-blue-500"));
    assert!(candidates.contains("active:bg-blue-600"));
    assert!(candidates.contains("dark:bg-slate-800"));
    assert!(candidates.contains("m-2"));
    assert!(candidates.contains("text-sm"));

    println!(
        "Gate 3 Passed: Candidate scanner extracted {} valid tokens from .fsx",
        candidates.len()
    );
}

#[test]
fn test_gate3_unsupported_web_utilities_rejected() {
    let compiler = TailwindCompiler::new();
    let unsupported_classes = &[
        "float-left",
        "clear-both",
        "cursor-pointer",
        "grid-cols-4",
        "table-auto",
        "select-none",
    ];

    let result = compiler.compile(&[], unsupported_classes);
    assert!(result.is_err(), "Unsupported utilities must be rejected");

    let diags = result.unwrap_err();
    assert_eq!(diags.len(), 6);
    for d in diags {
        assert!(d.message.contains("not supported in native mobile layouts"));
    }

    println!("Gate 3 Passed: Strictly rejected unsupported web-only utilities.");
}

#[test]
fn test_gate3_300_plus_tailwind_utility_corpus() {
    // Generate >= 300 distinct Tailwind utility classes covering all domains
    let mut corpus: Vec<String> = Vec::new();

    // 1. Layout & Flexbox (40 classes)
    let layout_classes = [
        "flex",
        "hidden",
        "flex-row",
        "flex-col",
        "flex-row-reverse",
        "flex-col-reverse",
        "flex-wrap",
        "flex-nowrap",
        "flex-wrap-reverse",
        "flex-1",
        "flex-auto",
        "flex-initial",
        "flex-none",
        "grow",
        "grow-0",
        "flex-grow",
        "flex-grow-0",
        "shrink",
        "shrink-0",
        "flex-shrink",
        "flex-shrink-0",
        "items-start",
        "items-center",
        "items-end",
        "items-stretch",
        "items-baseline",
        "justify-start",
        "justify-center",
        "justify-end",
        "justify-between",
        "justify-around",
        "justify-evenly",
        "content-start",
        "content-center",
        "content-end",
        "content-between",
        "content-around",
        "content-stretch",
        "self-auto",
        "self-start",
        "self-center",
        "self-end",
        "self-stretch",
        "absolute",
        "relative",
    ];
    for &c in &layout_classes {
        corpus.push(c.to_string());
    }

    // 2. Insets (10 classes)
    let inset_classes = [
        "inset-0", "top-0", "top-2", "top-4", "bottom-0", "bottom-4", "left-0", "left-4",
        "right-0", "right-4",
    ];
    for &c in &inset_classes {
        corpus.push(c.to_string());
    }

    // 3. Spacing: p, px, py, pt, pb, pl, pr, m, mx, my, mt, mb, ml, mr, gap (90 classes)
    let scales = [
        "0", "1", "2", "3", "4", "5", "6", "8", "10", "12", "16", "20", "24", "32", "48", "64",
    ];
    for scale in scales {
        corpus.push(format!("p-{}", scale));
        corpus.push(format!("px-{}", scale));
        corpus.push(format!("py-{}", scale));
        corpus.push(format!("m-{}", scale));
        corpus.push(format!("mx-{}", scale));
        corpus.push(format!("gap-{}", scale));
    }

    // 4. Sizing: w, h, min-w, max-w, min-h, max-h (50 classes)
    let size_scales = [
        "full", "auto", "1/2", "1/3", "2/3", "1/4", "3/4", "4", "8", "12", "16", "24", "32", "48",
        "64",
    ];
    for s in size_scales {
        corpus.push(format!("w-{}", s));
        corpus.push(format!("h-{}", s));
    }
    corpus.push("min-w-0".to_string());
    corpus.push("min-w-full".to_string());
    corpus.push("max-w-full".to_string());
    corpus.push("min-h-0".to_string());
    corpus.push("min-h-full".to_string());
    corpus.push("max-h-full".to_string());

    // 5. Typography (40 classes)
    let typo_classes = [
        "text-xs",
        "text-sm",
        "text-base",
        "text-lg",
        "text-xl",
        "text-2xl",
        "text-3xl",
        "text-4xl",
        "text-5xl",
        "font-thin",
        "font-extralight",
        "font-light",
        "font-normal",
        "font-medium",
        "font-semibold",
        "font-bold",
        "font-extrabold",
        "font-black",
        "italic",
        "not-italic",
        "text-left",
        "text-center",
        "text-right",
        "underline",
        "line-through",
        "no-underline",
        "leading-none",
        "leading-tight",
        "leading-snug",
        "leading-normal",
        "leading-relaxed",
        "leading-loose",
        "tracking-tighter",
        "tracking-tight",
        "tracking-normal",
        "tracking-wide",
        "tracking-wider",
        "tracking-widest",
    ];
    for &c in &typo_classes {
        corpus.push(c.to_string());
    }

    // 6. Colors across shades (50 classes)
    let colors = [
        "slate", "gray", "red", "amber", "green", "emerald", "blue", "indigo", "purple", "pink",
    ];
    let shades = [
        "50", "100", "200", "400", "500", "600", "700", "800", "900", "950",
    ];
    for c in colors {
        for s in shades {
            corpus.push(format!("bg-{}-{}", c, s));
            corpus.push(format!("text-{}-{}", c, s));
        }
    }
    corpus.push("bg-white".to_string());
    corpus.push("bg-black".to_string());
    corpus.push("bg-transparent".to_string());
    corpus.push("text-white".to_string());
    corpus.push("text-black".to_string());

    // 7. Borders, Radii, Shadows, Opacity (35 classes)
    let border_classes = [
        "border",
        "border-0",
        "border-2",
        "border-4",
        "border-8",
        "rounded-none",
        "rounded-sm",
        "rounded",
        "rounded-md",
        "rounded-lg",
        "rounded-xl",
        "rounded-2xl",
        "rounded-3xl",
        "rounded-full",
        "shadow-none",
        "shadow-sm",
        "shadow",
        "shadow-md",
        "shadow-lg",
        "shadow-xl",
        "shadow-2xl",
        "elevation-1",
        "elevation-2",
        "elevation-4",
        "elevation-8",
        "elevation-16",
        "opacity-0",
        "opacity-25",
        "opacity-50",
        "opacity-75",
        "opacity-100",
    ];
    for &c in &border_classes {
        corpus.push(c.to_string());
    }

    // 8. Arbitrary Values (15 classes)
    let arbitrary_classes = [
        "w-[120px]",
        "w-[50%]",
        "h-[75px]",
        "min-w-[100px]",
        "max-w-[400px]",
        "p-[14px]",
        "m-[10px]",
        "gap-[18px]",
        "top-[25px]",
        "bg-[#3b82f6]",
        "text-[#ef4444]",
        "border-[#10b981]",
        "rounded-[15px]",
    ];
    for &c in &arbitrary_classes {
        corpus.push(c.to_string());
    }

    // 9. Variants (20 classes)
    let variant_classes = [
        "active:bg-blue-600",
        "active:opacity-75",
        "dark:bg-slate-900",
        "dark:text-white",
        "dark:border-slate-700",
        "sm:p-6",
        "sm:flex-row",
        "md:p-8",
        "md:flex-row",
        "lg:max-w-full",
        "focus:border-blue-500",
        "disabled:opacity-50",
        "checked:bg-blue-600",
    ];
    for &c in &variant_classes {
        corpus.push(c.to_string());
    }

    let corpus_refs: Vec<&str> = corpus.iter().map(String::as_str).collect();
    let distinct_count = corpus_refs.iter().collect::<HashSet<_>>().len();

    println!(
        "Total distinct Tailwind utility classes in corpus: {}",
        distinct_count
    );
    assert!(
        distinct_count >= 300,
        "Corpus must have at least 300 utility classes, found {}",
        distinct_count
    );

    // Compile entire corpus ahead-of-time
    let compiler = TailwindCompiler::new();
    let compiled = compiler
        .compile(&[], &corpus_refs)
        .expect("All corpus utilities must compile cleanly without diagnostics");

    assert!(
        compiled.rules.len() >= distinct_count,
        "All utilities must generate cascade rules"
    );

    // =========================================================================
    // Verify Mobile Preflight Invariant: Universal * rule
    // =========================================================================
    let preflight_rule = compiled
        .rules
        .iter()
        .find(|r| r.selector == "*")
        .expect("Mobile preflight * rule must exist");
    assert_eq!(preflight_rule.style.display, Some(Display::Flex));
    assert_eq!(
        preflight_rule.style.flex_direction,
        Some(FlexDirection::Column)
    );
    assert_eq!(preflight_rule.style.position, Some(Position::Relative));

    // =========================================================================
    // Verify Cascade Resolution and Zero Runtime Parsing Contract
    // =========================================================================
    let mut cascade = CascadeEngine::new();
    for rule in compiled.rules {
        cascade.add_rule(rule);
    }

    // Resolve an element composed of multiple Tailwind classes
    let classes = &[
        "flex",
        "flex-row",
        "items-center",
        "justify-between",
        "p-4",
        "bg-blue-500",
        "text-white",
        "rounded-lg",
        "shadow-md",
        "w-[120px]",
        "active:bg-blue-600",
        "dark:bg-slate-900",
    ];
    let resolved = cascade.resolve(classes, Some("Row"), None);

    assert_eq!(resolved.flex_direction, Some(FlexDirection::Row));
    assert_eq!(resolved.align_items, Some(AlignItems::Center));
    assert_eq!(resolved.justify_content, Some(JustifyContent::SpaceBetween));
    assert_eq!(resolved.padding_top, Some(DimensionValue::Points(16.0)));
    assert_eq!(resolved.width, Some(DimensionValue::Points(120.0)));
    assert_eq!(resolved.border_radius, Some((8.0, 8.0, 8.0, 8.0)));
    assert_eq!(resolved.elevation, Some(4.0)); // shadow-md
    assert_eq!(resolved.background_color, Some(0xFF3B82F6)); // bg-blue-500
    assert_eq!(resolved.color, Some(0xFFFFFFFF)); // text-white
    assert!(resolved.active.is_some(), "active variant must be resolved");
    assert!(resolved.dark.is_some(), "dark variant must be resolved");

    // Register with ReactiveStyleResolver and verify binary JNI patch output
    let mut resolver = ReactiveStyleResolver::new();
    let node_id = NodeId(500);
    let (style_id, reg_patch_opt) = resolver.register_node(node_id, resolved);

    assert_ne!(style_id, 0);
    assert!(reg_patch_opt.is_some());

    let mut batch = PatchBatch::new();
    batch.push(reg_patch_opt.unwrap());
    batch.push(Patch::SetStyle { node_id, style_id });

    let encoded_binary = batch.encode();
    assert!(!encoded_binary.is_empty());

    // Verify binary buffer round-trip with zero string parsing
    let decoded_batch = PatchBatch::decode(&encoded_binary).unwrap();
    assert_eq!(decoded_batch.len(), 2);

    println!(
        "Gate 3 Passed: {} Tailwind utility classes compiled AOT. Emitted {} bytes binary patch buffer with ZERO runtime CSS parsing.",
        distinct_count,
        encoded_binary.len()
    );
}
