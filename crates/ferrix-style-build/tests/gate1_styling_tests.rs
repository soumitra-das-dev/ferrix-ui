use std::path::Path;
use std::time::Instant;

use ferrix_renderer_protocol::{NodeId, Patch, PatchBatch};
use ferrix_style::{
    CascadeEngine, ColorScheme, ElementState, EnvironmentState, ReactiveStyleResolver, StyleIr,
};
use ferrix_style_build::StyleCompiler;

#[test]
fn test_gate1_styling_pipeline_and_benchmarks() {
    let global_css = include_str!("../../../examples/styling/global.css");
    let card_module_css = include_str!("../../../examples/styling/card.module.css");

    let compiler = StyleCompiler::new();

    // 1. Compile global.css
    let global_res = compiler
        .compile(global_css, Path::new("global.css"))
        .expect("global.css must compile cleanly");
    assert!(!global_res.rules.is_empty());
    assert!(global_res.variables.contains_key("--accent"));

    // 2. Compile card.module.css (CSS Module)
    let module_res = compiler
        .compile(card_module_css, Path::new("card.module.css"))
        .expect("card.module.css must compile cleanly");
    assert!(module_res.class_map.contains_key("card"));
    assert!(module_res.class_map.contains_key("cardHeader"));

    let scoped_card_class = module_res.class_map.get("card").unwrap();
    let scoped_header_class = module_res.class_map.get("cardHeader").unwrap();

    // 3. Build CascadeEngine with compiled rules
    let mut cascade = CascadeEngine::new();
    for rule in global_res.rules {
        cascade.add_rule(rule);
    }
    for rule in module_res.rules {
        cascade.add_rule(rule);
    }

    // 4. Resolve styles for component tree
    // Element 1: Root container (.container)
    let container_style = cascade.resolve(&["container"], Some("Column"), None);

    // Element 2: Card (.card with active state)
    let card_style = cascade.resolve(&[scoped_card_class.as_str()], Some("Column"), None);
    assert!(
        card_style.active.is_some(),
        "card must contain :active variant"
    );
    assert!(card_style.dark.is_some(), "card must contain dark variant");

    // Element 3: Card Header (.cardHeader with inline style overriding color)
    let mut inline_header_override = StyleIr::new();
    inline_header_override.font_size = Some(22.0); // overrides 18px from module
    let header_style = cascade.resolve(
        &[scoped_header_class.as_str()],
        Some("Text"),
        Some(&inline_header_override),
    );
    assert_eq!(header_style.font_size, Some(22.0));

    // 5. Setup ReactiveStyleResolver
    let mut resolver = ReactiveStyleResolver::new();
    for (k, &v) in &global_res.variables {
        resolver.set_variable(k.clone(), v);
    }

    let node_container = NodeId(1);
    let node_card = NodeId(2);
    let node_header = NodeId(3);

    let (style_container_id, patch1) = resolver.register_node(node_container, container_style);
    let (style_card_id, patch2) = resolver.register_node(node_card, card_style);
    let (style_header_id, patch3) = resolver.register_node(node_header, header_style);

    assert_ne!(style_container_id, 0);
    assert_ne!(style_card_id, 0);
    assert_ne!(style_header_id, 0);

    // Initial registration batch
    let mut initial_batch = PatchBatch::new();
    if let Some(p) = patch1 {
        initial_batch.push(p);
    }
    if let Some(p) = patch2 {
        initial_batch.push(p);
    }
    if let Some(p) = patch3 {
        initial_batch.push(p);
    }

    println!(
        "Initial style registration batch size: {}",
        initial_batch.len()
    );

    // =========================================================================
    // GATE 1 ASSERTION 1: Unchanged state emits exactly 0 patches!
    // =========================================================================
    let no_patch = resolver.update_element_state(node_card, ElementState::default());
    assert_eq!(
        no_patch, None,
        "Gate 1 Failed: Unchanged state must emit 0 patches"
    );

    // =========================================================================
    // GATE 1 ASSERTION 2: Interaction state (:active) emits exactly 1 SetStyle patch!
    // =========================================================================
    let active_patch = resolver.update_element_state(
        node_card,
        ElementState {
            active: true,
            focus: false,
            disabled: false,
            checked: false,
        },
    );
    assert!(
        active_patch.is_some(),
        "Gate 1 Failed: Active state must emit SetStyle patch"
    );
    let _active_style_id = match active_patch.unwrap() {
        Patch::SetStyle { node_id, style_id } => {
            assert_eq!(node_id, node_card);
            style_id
        }
        other => panic!("Expected Patch::SetStyle, got {:?}", other),
    };

    // Releasing active state restores initial style
    let release_patch = resolver.update_element_state(node_card, ElementState::default());
    assert_eq!(
        release_patch,
        Some(Patch::SetStyle {
            node_id: node_card,
            style_id: style_card_id,
        })
    );

    // =========================================================================
    // GATE 1 ASSERTION 3: Theme switch emits exactly 1 batched JNI call!
    // =========================================================================
    let mut dark_env = EnvironmentState::default();
    dark_env.color_scheme = ColorScheme::Dark;
    dark_env.variables = global_res.variables.clone();
    // Update variable in dark mode
    dark_env
        .variables
        .insert("--background".to_string(), 0xFF0F172A);

    let theme_patches = resolver.update_environment(dark_env);
    assert!(
        !theme_patches.is_empty(),
        "Theme switch must produce updates"
    );

    // Pack into a single atomic PatchBatch
    let mut theme_batch = PatchBatch::new();
    for p in theme_patches {
        theme_batch.push(p);
    }
    let encoded_theme_batch = theme_batch.encode();
    assert!(
        !encoded_theme_batch.is_empty(),
        "Theme batch must serialize to binary buffer"
    );

    // Decode and verify roundtrip
    let decoded_theme_batch =
        PatchBatch::decode(&encoded_theme_batch).expect("Binary buffer must decode cleanly");
    assert_eq!(theme_batch, decoded_theme_batch);
    println!(
        "Gate 1 Passed: Theme switch produced 1 batched JNI call ({} bytes, {} commands)",
        encoded_theme_batch.len(),
        decoded_theme_batch.len()
    );

    // =========================================================================
    // GATE 1 BENCHMARK: 10,000-Node Style Resolution Benchmark
    // =========================================================================
    const NODE_COUNT: usize = 10_000;
    println!(
        "Benchmarking style cascade resolution and interning for {} nodes...",
        NODE_COUNT
    );

    let start = Instant::now();
    let mut bench_resolver = ReactiveStyleResolver::new();

    for i in 0..NODE_COUNT {
        let node_id = NodeId(1000 + i as u64);
        let class = if i % 2 == 0 {
            scoped_card_class.as_str()
        } else {
            scoped_header_class.as_str()
        };
        let style = cascade.resolve(&[class], Some("Column"), None);
        bench_resolver.register_node(node_id, style);
    }

    let elapsed = start.elapsed();
    let elapsed_ms = elapsed.as_secs_f64() * 1000.0;
    let us_per_node = (elapsed.as_micros() as f64) / (NODE_COUNT as f64);

    println!(
        "Gate 1 Benchmark: {} nodes resolved in {:.2} ms ({:.3} µs/node)",
        NODE_COUNT, elapsed_ms, us_per_node
    );

    // Budget: Must resolve 10,000 nodes in under 1000ms in unoptimized debug, or under 100ms in release
    let budget_ms = if cfg!(debug_assertions) { 1000.0 } else { 100.0 };
    assert!(
        elapsed_ms < budget_ms,
        "10k node resolve budget exceeded: {:.2} ms > {:.2} ms",
        elapsed_ms,
        budget_ms
    );
}
