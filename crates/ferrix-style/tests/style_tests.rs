use ferrix_layout::{DimensionValue, FlexDirection};
use ferrix_renderer_protocol::{NodeId, Patch, Property};
use ferrix_style::{
    cx, CascadeEngine, CascadeRule, Color, ColorScheme, ElementState, EnvironmentState,
    ReactiveStyleResolver, RuleSource, Specificity, StyleIr, StyleRegistry, VariantConfig,
};

#[test]
fn test_color_parsing_and_conversion() {
    // Hex
    assert_eq!(Color::hex("#FFF"), 0xFFFFFFFF);
    assert_eq!(Color::hex("#000"), 0xFF000000);
    assert_eq!(Color::hex("#2563EB"), 0xFF2563EB);
    assert_eq!(Color::hex("#FF000080"), 0x80FF0000);

    // RGB / RGBA
    assert_eq!(Color::parse("rgb(37, 99, 235)"), Some(0xFF2563EB));
    assert_eq!(Color::parse("rgba(37, 99, 235, 0.5)"), Some(0x7F2563EB));

    // HSL
    let hsl_red = Color::parse("hsl(0, 100%, 50%)").expect("valid hsl");
    assert_eq!(hsl_red, 0xFFFF0000);

    // OKLCH
    let oklch_color = Color::parse("oklch(0.6 0.25 250)").expect("valid oklch");
    assert_ne!(oklch_color, 0);

    // Color mix
    let mixed = Color::mix(0xFFFF0000, 0xFF0000FF, 0.5); // Red + Blue 50%
    let r = (mixed >> 16) & 0xFF;
    let b = mixed & 0xFF;
    assert!((r as i32 - 128).abs() <= 2);
    assert!((b as i32 - 128).abs() <= 2);
}

#[test]
fn test_style_ir_properties_and_merging() {
    let mut s1 = StyleIr::new();
    s1.background_color = Some(0xFF1E293B);
    s1.font_size = Some(14.0);
    s1.flex_direction = Some(FlexDirection::Column);

    let mut s2 = StyleIr::new();
    s2.font_size = Some(18.0); // overrides s1
    s2.color = Some(0xFFFFFFFF);
    s2.flex_direction = Some(FlexDirection::Row); // overrides s1

    let merged = s1.merge(&s2);
    assert_eq!(merged.background_color, Some(0xFF1E293B));
    assert_eq!(merged.font_size, Some(18.0));
    assert_eq!(merged.color, Some(0xFFFFFFFF));
    assert_eq!(merged.flex_direction, Some(FlexDirection::Row));

    let props = merged.to_properties();
    assert!(props.contains(&Property::BackgroundColor(0xFF1E293B)));
    assert!(props.contains(&Property::FontSize(18.0)));
    assert!(props.contains(&Property::TextColor(0xFFFFFFFF)));

    let ls = merged.to_layout_style();
    assert_eq!(ls.flex_direction, FlexDirection::Row);
}

#[test]
fn test_style_registry_deduplication_and_patches() {
    let mut registry = StyleRegistry::new();

    let mut s1 = StyleIr::new();
    s1.background_color = Some(0xFF1E293B);
    s1.font_size = Some(16.0);

    let id1 = registry.intern(s1.clone());
    assert_eq!(id1, 1);

    // Interning exact duplicate returns same ID
    let id2 = registry.intern(s1.clone());
    assert_eq!(id2, 1);

    let mut s3 = StyleIr::new();
    s3.background_color = Some(0xFF2563EB);
    let id3 = registry.intern(s3);
    assert_eq!(id3, 2);

    let patch = registry
        .take_registration_patch()
        .expect("has registration patch");
    if let Patch::RegisterStyles { styles } = patch {
        assert_eq!(styles.len(), 2);
        assert_eq!(styles[0].0, 1);
        assert_eq!(styles[1].0, 2);
    } else {
        panic!("expected RegisterStyles patch");
    }

    // Subsequent take returns None (zero uncommitted)
    assert!(registry.take_registration_patch().is_none());
}

#[test]
fn test_cascade_engine_merge_order_and_specificity() {
    let mut engine = CascadeEngine::new();

    // 1. Universal rule: * { color: #888888, font-size: 14 }
    let mut universal_style = StyleIr::new();
    universal_style.color = Some(0xFF888888);
    universal_style.font_size = Some(14.0);
    engine.add_rule(CascadeRule::new(
        "*",
        RuleSource::Universal,
        Specificity::UNIVERSAL,
        1,
        universal_style,
    ));

    // 2. Global rule: .card { background-color: #1E293B, font-size: 16 }
    let mut card_style = StyleIr::new();
    card_style.background_color = Some(0xFF1E293B);
    card_style.font_size = Some(16.0); // overrides universal 14
    engine.add_rule(CascadeRule::new(
        ".card",
        RuleSource::GlobalCss,
        Specificity::CLASS,
        2,
        card_style,
    ));

    // 3. Module rule: .highlight { color: #2563EB }
    let mut highlight_style = StyleIr::new();
    highlight_style.color = Some(0xFF2563EB); // overrides universal color
    engine.add_rule(CascadeRule::new(
        ".highlight",
        RuleSource::ModuleCss,
        Specificity::CLASS,
        3,
        highlight_style,
    ));

    // Resolve without inline style
    let resolved = engine.resolve(&["card", "highlight"], Some("Column"), None);
    assert_eq!(resolved.background_color, Some(0xFF1E293B));
    assert_eq!(resolved.font_size, Some(16.0));
    assert_eq!(resolved.color, Some(0xFF2563EB));

    // 4. Inline style takes final precedence: style={{ font_size: 24 }}
    let mut inline_style = StyleIr::new();
    inline_style.font_size = Some(24.0);
    let final_style = engine.resolve(&["card", "highlight"], Some("Column"), Some(&inline_style));
    assert_eq!(final_style.font_size, Some(24.0));
    assert_eq!(final_style.background_color, Some(0xFF1E293B));
}

#[test]
fn test_reactive_style_resolver_zero_patch_contract_and_theme_switch() {
    let mut resolver = ReactiveStyleResolver::new();
    resolver.set_variable("--accent", 0xFF2563EB);

    let node1 = NodeId(10);
    let mut base_style = StyleIr::new();
    base_style.background_color = Some(0xFFFFFFFF);
    base_style.color = Some(0xFF000000);

    // Dark mode variant: dark: { background-color: #0F172A, color: #F8FAFC }
    let mut dark_variant = StyleIr::new();
    dark_variant.background_color = Some(0xFF0F172A);
    dark_variant.color = Some(0xFFF8FAFC);
    base_style.dark = Some(Box::new(dark_variant));

    // Active (pressed) variant: active: { opacity: 0.8 }
    let mut active_variant = StyleIr::new();
    active_variant.opacity = Some(0.8);
    base_style.active = Some(Box::new(active_variant));

    let (style_id, reg_patch) = resolver.register_node(node1, base_style);
    assert_eq!(style_id, 1);
    assert!(reg_patch.is_some());

    // 1. Unchanged interaction state => 0 patches emitted!
    let patch = resolver.update_element_state(node1, ElementState::default());
    assert_eq!(patch, None);

    // 2. State change (:active pressed) => exactly 1 SetStyle patch emitted
    let patch = resolver.update_element_state(
        node1,
        ElementState {
            active: true,
            focus: false,
            disabled: false,
            checked: false,
        },
    );
    assert!(patch.is_some());
    if let Some(Patch::SetStyle { node_id, style_id }) = patch {
        assert_eq!(node_id, node1);
        assert_eq!(style_id, 2); // new style interned for pressed state
    } else {
        panic!("expected SetStyle patch");
    }

    // 3. Repeat identical active state => 0 patches emitted!
    let patch = resolver.update_element_state(
        node1,
        ElementState {
            active: true,
            focus: false,
            disabled: false,
            checked: false,
        },
    );
    assert_eq!(patch, None);

    // Release active state => restores style 1
    let patch = resolver.update_element_state(node1, ElementState::default());
    assert_eq!(
        patch,
        Some(Patch::SetStyle {
            node_id: node1,
            style_id: 1
        })
    );

    // 4. Global theme switch to Dark => exactly 1 batched JNI call with updated styles
    let mut dark_env = EnvironmentState::default();
    dark_env.color_scheme = ColorScheme::Dark;
    let patches = resolver.update_environment(dark_env);
    // Should include RegisterStyles for the new dark style + SetStyle for node1
    assert_eq!(patches.len(), 2);
    assert!(matches!(patches[0], Patch::RegisterStyles { .. }));
    assert!(matches!(patches[1], Patch::SetStyle { node_id, .. } if node_id == node1));
}

#[test]
fn test_helpers_cx_and_variants() {
    let merged = cx([Some("btn"), Some("btn-primary"), None, Some(""), Some("lg")]);
    assert_eq!(merged, "btn btn-primary lg");

    let macro_merged = cx!("btn", Some("btn-primary"), None, "", Some("lg"));
    assert_eq!(macro_merged, "btn btn-primary lg");

    let mut base = StyleIr::new();
    base.padding_left = Some(DimensionValue::Points(12.0));
    base.padding_right = Some(DimensionValue::Points(12.0));

    let mut primary = StyleIr::new();
    primary.background_color = Some(0xFF2563EB);
    primary.color = Some(0xFFFFFFFF);

    let mut secondary = StyleIr::new();
    secondary.background_color = Some(0xFFE2E8F0);
    secondary.color = Some(0xFF0F172A);

    let config = VariantConfig::new(base)
        .with_variant("intent", [("primary", primary), ("secondary", secondary)])
        .with_default("intent", "primary");

    let resolved_default = config.resolve([]);
    assert_eq!(resolved_default.background_color, Some(0xFF2563EB));

    let resolved_secondary = config.resolve([("intent", "secondary")]);
    assert_eq!(resolved_secondary.background_color, Some(0xFFE2E8F0));
}
