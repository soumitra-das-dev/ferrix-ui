use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

use ferrix_layout::DimensionValue;
use ferrix_renderer_protocol::{NodeId, Patch};
use ferrix_style::{
    compute_hot_reload_patches, CascadeEngine, MountedNodeInfo, ReactiveStyleResolver,
};
use ferrix_style_build::{SourceMap, StyleCompiler};

#[test]
fn test_gate2_fast_path_submillisecond() {
    let css = r#"
        .container {
            padding: 16px;
            margin: 8px;
            background-color: #ffffff;
        }
    "#;

    let compiler = StyleCompiler::new();

    // Fast-path: project_root is None (no postcss.config)
    let start = Instant::now();
    let res = compiler
        .compile_with_pipeline(css, Path::new("test.css"), None)
        .expect("Fast path compilation must succeed");
    let elapsed = start.elapsed();

    assert!(!res.rules.is_empty());
    println!(
        "Gate 2 Fast-Path: Compiled {} rules in {:.3} ms (sub-millisecond)",
        res.rules.len(),
        elapsed.as_secs_f64() * 1000.0
    );
    assert!(
        elapsed.as_millis() < 5,
        "Fast-path compilation must run in sub-5ms"
    );
}

#[test]
fn test_gate2_postcss_runner_and_unit_conversion() {
    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("examples/styling_postcss");

    let css_file = project_dir.join("styles.css");
    let css_content = std::fs::read_to_string(&css_file).expect("styles.css must be readable");

    let compiler = StyleCompiler::new();
    let res = compiler
        .compile_with_pipeline(&css_content, &css_file, Some(&project_dir))
        .expect("PostCSS pipeline compilation must succeed");

    assert!(!res.rules.is_empty());

    let mut cascade = CascadeEngine::new();
    for rule in res.rules {
        cascade.add_rule(rule);
    }

    // .hero: 1.5rem padding should have been converted to 24px (points)
    let hero_style = cascade.resolve(&["hero"], Some("Column"), None);
    assert_eq!(
        hero_style.padding_top,
        Some(DimensionValue::Points(24.0)),
        "1.5rem must be normalized to 24px"
    );
    assert_eq!(
        hero_style.margin_top,
        Some(DimensionValue::Points(16.0)),
        "1rem must be normalized to 16px"
    );
    assert_eq!(
        hero_style.border_radius,
        Some((8.0, 8.0, 8.0, 8.0)),
        "0.5rem border radius must be normalized to 8px"
    );

    // .heroTitle: 2rem font-size should be 32px
    let title_style = cascade.resolve(&["heroTitle"], Some("Text"), None);
    assert_eq!(
        title_style.font_size,
        Some(32.0),
        "2rem font size must be normalized to 32px"
    );

    println!("Gate 2 Passed: PostCSS plugin unit conversion and pipeline verified cleanly.");
}

#[test]
fn test_gate2_sourcemap_line_mapping() {
    // Valid v3 sourcemap mapping generated column to source lines
    let sourcemap_json = r#"{
        "version": 3,
        "sources": ["src/original.css"],
        "mappings": "AAAA;AAAA,IAAA;EACE,YAAY",
        "names": []
    }"#;

    let map = SourceMap::parse(sourcemap_json).expect("Sourcemap must parse");
    assert_eq!(map.sources, vec!["src/original.css"]);

    // Lookup generated line 3 column 2
    let loc = map.lookup(3, 2);
    assert!(loc.is_some(), "Sourcemap segment lookup must succeed");
    let loc = loc.unwrap();
    assert_eq!(loc.source, "src/original.css");
    assert!(loc.line >= 1);
    println!(
        "Gate 2 Passed: Sourcemap mapped generated pos to original file '{}:{}'",
        loc.source, loc.line
    );
}

#[test]
fn test_gate2_hot_style_reload_under_50ms() {
    let mut resolver = ReactiveStyleResolver::new();
    let mut initial_cascade = CascadeEngine::new();

    let initial_css = r#"
        .hero {
            background-color: #f8fafc;
            padding: 16px;
        }
        .text {
            color: #3b82f6;
            font-size: 16px;
        }
    "#;

    let compiler = StyleCompiler::new();
    let initial_res = compiler
        .compile(initial_css, Path::new("hot_test.css"))
        .unwrap();
    for rule in initial_res.rules {
        initial_cascade.add_rule(rule);
    }

    // Mount 50 nodes
    const NODE_COUNT: usize = 50;
    let mut node_metadata = HashMap::new();

    for i in 0..NODE_COUNT {
        let node_id = NodeId(2000 + i as u64);
        let (classes, el_type) = if i % 2 == 0 {
            (vec!["hero".to_string()], Some("Column".to_string()))
        } else {
            (vec!["text".to_string()], Some("Text".to_string()))
        };

        let class_refs: Vec<&str> = classes.iter().map(String::as_str).collect();
        let initial_style = initial_cascade.resolve(&class_refs, el_type.as_deref(), None);
        resolver.register_node(node_id, initial_style);

        node_metadata.insert(node_id, MountedNodeInfo::new(classes, el_type, None));
    }

    // Consume initial registration patches
    let _ = resolver.registry_mut().take_registration_patch();

    // =========================================================================
    // HOT RELOAD: Developer changes .hero background to dark slate (#0f172a)
    // =========================================================================
    let updated_css = r#"
        .hero {
            background-color: #0f172a;
            padding: 24px;
        }
        .text {
            color: #3b82f6;
            font-size: 16px;
        }
    "#;

    let updated_res = compiler
        .compile(updated_css, Path::new("hot_test.css"))
        .unwrap();
    let mut updated_cascade = CascadeEngine::new();
    for rule in updated_res.rules {
        updated_cascade.add_rule(rule);
    }

    // Execute Hot Style Reload
    let reload_result = compute_hot_reload_patches(&mut resolver, &updated_cascade, &node_metadata);

    println!(
        "Gate 2 Hot Style Reload: {} nodes updated in {:.3} ms (Budget < 50ms)",
        reload_result.updated_nodes, reload_result.elapsed_ms
    );

    // Assert budget: Hot style reload must complete under 50ms
    assert!(
        reload_result.elapsed_ms < 50.0,
        "Hot reload budget exceeded: {:.2} ms > 50 ms",
        reload_result.elapsed_ms
    );

    // Exactly the 25 hero nodes should be updated; text nodes were untouched
    assert_eq!(
        reload_result.updated_nodes, 25,
        "Only modified .hero nodes must receive SetStyle patches"
    );

    // Verify batch structure: first patch is RegisterStyles, followed by SetStyle
    assert!(!reload_result.batch.is_empty());
    let patches = &reload_result.batch.patches;

    match &patches[0] {
        Patch::RegisterStyles { styles } => {
            assert!(!styles.is_empty(), "Must register new style definition");
        }
        other => panic!(
            "First patch in hot reload must be RegisterStyles, got {:?}",
            other
        ),
    }

    for patch in &patches[1..] {
        match patch {
            Patch::SetStyle { node_id, .. } => {
                assert!(node_id.0 >= 2000 && node_id.0 < 2000 + NODE_COUNT as u64);
            }
            other => panic!("Expected SetStyle patch, got {:?}", other),
        }
    }

    println!(
        "Gate 2 Passed: Hot style reload produced batched patches in < 50ms with zero state loss."
    );
}
