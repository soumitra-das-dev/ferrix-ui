//! Integration and unit tests for ferrix-router (F1).

use ferrix_router::deeplink::parse_deep_link;
use ferrix_router::hooks::{
    set_current_router, use_can_go_back, use_global_search_params, use_is_focused,
    use_local_search_params, use_pathname, use_router, use_segments,
};
use ferrix_router::manifest::RouteManifest;
use ferrix_router::state::RouterState;

#[test]
fn test_virtual_dir_scanning_and_manifest_construction() {
    let files = vec![
        "app/index.fsx",
        "app/_layout.fsx",
        "app/users/[id].fsx",
        "app/docs/[...rest].fsx",
        "app/(auth)/login.fsx",
        "app/(auth)/register.fsx",
        "app/(modal)/settings.fsx",
        "app/+not-found.fsx",
        "app/+error.fsx",
    ];

    let manifest = RouteManifest::scan_virtual_dir(&files).expect("Manifest scan failed");

    // Index
    let m_index = manifest.match_url("/");
    assert_eq!(m_index.pathname, "/");
    assert_eq!(m_index.route.pattern, "/");
    assert_eq!(m_index.route.component_name, "IndexScreen");

    // Dynamic param: /users/123
    let m_user = manifest.match_url("/users/123");
    assert_eq!(m_user.pathname, "/users/123");
    assert_eq!(m_user.params.get("id").map(|s| s.as_str()), Some("123"));

    // Catch-all: /docs/a/b/c
    let m_docs = manifest.match_url("/docs/a/b/c");
    assert_eq!(m_docs.params.get("rest").map(|s| s.as_str()), Some("a/b/c"));

    // Route groups: (auth)/login should map to /login
    let m_login = manifest.match_url("/login");
    assert_eq!(m_login.route.component_name, "LoginScreen");

    // Query parameters
    let m_query = manifest.match_url("/users/42?filter=active&sort=desc");
    assert_eq!(m_query.params.get("id").map(|s| s.as_str()), Some("42"));
    assert_eq!(
        m_query.query.get("filter").map(|s| s.as_str()),
        Some("active")
    );
    assert_eq!(m_query.query.get("sort").map(|s| s.as_str()), Some("desc"));

    // Not found fallback
    let m_404 = manifest.match_url("/unknown/path/here");
    assert!(m_404.route.is_not_found);
}

#[test]
fn test_router_state_stack_operations_and_signals() {
    let files = vec!["app/index.fsx", "app/feed.fsx", "app/post/[id].fsx"];
    let manifest = RouteManifest::scan_virtual_dir(&files).unwrap();
    let state = RouterState::new(manifest, "/");

    assert_eq!(state.current_pathname(), "/");
    assert_eq!(state.stack_len(), 1);
    assert!(!state.can_go_back());

    // Push /feed
    state.push("/feed");
    assert_eq!(state.current_pathname(), "/feed");
    assert_eq!(state.stack_len(), 2);
    assert!(state.can_go_back());
    assert_eq!(state.segments_signal().get(), vec!["feed".to_string()]);

    // Push /post/99?comment=1
    state.push("/post/99?comment=1");
    assert_eq!(state.current_pathname(), "/post/99");
    assert_eq!(state.stack_len(), 3);
    assert_eq!(
        state
            .local_params_signal()
            .get()
            .get("id")
            .map(|s| s.as_str()),
        Some("99")
    );
    assert_eq!(
        state
            .global_params_signal()
            .get()
            .get("comment")
            .map(|s| s.as_str()),
        Some("1")
    );

    // Pop back to /feed
    assert!(state.back());
    assert_eq!(state.current_pathname(), "/feed");
    assert_eq!(state.stack_len(), 2);
    assert!(state.can_go_back());

    // Pop back to /
    assert!(state.back());
    assert_eq!(state.current_pathname(), "/");
    assert_eq!(state.stack_len(), 1);
    assert!(!state.can_go_back());

    // Can't pop root
    assert!(!state.back());
    assert_eq!(state.stack_len(), 1);
}

#[test]
fn test_router_hooks_and_context() {
    let files = vec!["app/index.fsx", "app/profile/[id].fsx"];
    let manifest = RouteManifest::scan_virtual_dir(&files).unwrap();
    let state = RouterState::new(manifest, "/profile/soumo?theme=dark");

    set_current_router(state.clone());

    let router = use_router();
    let pathname = use_pathname();
    let segments = use_segments();
    let local_params = use_local_search_params();
    let global_params = use_global_search_params();
    let is_focused = use_is_focused();
    let can_go_back = use_can_go_back();

    assert_eq!(pathname.get(), "/profile/soumo");
    assert_eq!(segments.get(), vec!["profile", "soumo"]);
    assert_eq!(
        local_params.get().get("id").map(|s| s.as_str()),
        Some("soumo")
    );
    assert_eq!(
        global_params.get().get("theme").map(|s| s.as_str()),
        Some("dark")
    );
    assert!(is_focused.get());
    assert!(!can_go_back.get());

    router.push("/");
    assert_eq!(pathname.get(), "/");
    assert!(router.can_go_back());
    assert!(can_go_back.get());

    router.back();
    assert_eq!(pathname.get(), "/profile/soumo");
}

#[test]
fn test_process_death_and_serialization_roundtrip() {
    let files = vec![
        "app/index.fsx",
        "app/settings.fsx",
        "app/settings/privacy.fsx",
    ];
    let manifest = RouteManifest::scan_virtual_dir(&files).unwrap();
    let state = RouterState::new(manifest.clone(), "/");

    state.push("/settings");
    state.push("/settings/privacy");
    assert_eq!(state.stack_len(), 3);

    // Serialize to JSON snapshot
    let snapshot = state.snapshot();
    let serialized = serde_json::to_string(&snapshot).expect("Failed to serialize snapshot");

    // Deserialize and restore into new router state instance
    let deserialized: ferrix_router::state::RouterSnapshot =
        serde_json::from_str(&serialized).expect("Failed to deserialize snapshot");

    let restored_state = RouterState::new(manifest, "/");
    restored_state.restore(deserialized);

    assert_eq!(restored_state.stack_len(), 3);
    assert_eq!(restored_state.current_pathname(), "/settings/privacy");
    assert!(restored_state.can_go_back());

    assert!(restored_state.back());
    assert_eq!(restored_state.current_pathname(), "/settings");
}

#[test]
fn test_deep_linking_schemes() {
    // Custom scheme
    let path = parse_deep_link("myapp://profile/123?tab=activity", "myapp").unwrap();
    assert_eq!(path, "/profile/123?tab=activity");

    // Universal link
    let path_univ = parse_deep_link("https://app.example.com/checkout/step2", "myapp").unwrap();
    assert_eq!(path_univ, "/checkout/step2");
}

#[test]
fn test_router_error_escalation_and_handling() {
    let files = vec!["app/index.fsx", "app/dashboard.fsx", "app/+error.fsx"];
    let manifest = RouteManifest::scan_virtual_dir(&files).expect("Scan failed");
    let router = RouterState::new(manifest, "/dashboard");

    assert_eq!(router.current_pathname(), "/dashboard");
    assert!(!router.is_error());

    // Trigger an error in route logic
    router.trigger_error("Network timeout while fetching user records");

    assert!(router.is_error());
    assert_eq!(router.current_pathname(), "/dashboard/+error");
    assert_eq!(
        router.local_params().get("error").map(|s| s.as_str()),
        Some("Network timeout while fetching user records")
    );

    // Popping restores previous screen
    assert!(router.back());
    assert_eq!(router.current_pathname(), "/dashboard");
    assert!(!router.is_error());
}
