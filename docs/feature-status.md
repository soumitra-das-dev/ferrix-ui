# Ferrix Framework Feature Status & Honest Audit

**Audit Date**: 2026-11-09
**Toolchain Status**: Rust 2021 Workspace (63 tests passing), Android SDK 34 / Robolectric (all unit tests passing).

---

## 1. Feature Status Matrix

| Feature ID | Feature Name | Status | Existing Implementation Evidence | Missing Capabilities |
|---|---|:---:|---|---|
| **F1** | **Router (File-based)** | ✅ Done | `crates/ferrix-router/` (manifest, state, hooks, deeplink), `tests/router_tests.rs` (8 tests passing), `examples/router/app/` (`_layout.fsx`, `index.fsx`, `details/[id].fsx`, `(auth)/login.fsx`, `+not-found.fsx`), `docs/router.md`. | Fully implemented: `app/` conventions, dynamic/catch-all/groups, typed state in Rust, reactive signals, snapshot/restore, deep linking. |
| **F2** | **Navigation (Native Containers)** | ✅ Done | `crates/ferrix-navigation/` (stack, tabs, options), `NavigationTest.kt` (Robolectric), `FerrixNavContainer.kt`, `FerrixScreenView.kt`, `FerrixHeaderView.kt`, `FerrixTabBarView.kt`, `examples/navigation/stack_and_tabs.fsx`, `docs/navigation.md`. | Fully implemented: StackNavigator, TabNavigator, native transitions, screen options, header updates, and command execution. |
| **F3** | **Storage Systems** | ✅ Done | `crates/ferrix-storage/` (KV store, WAL journal, CRC32, `use_storage`), `crates/ferrix-sqlite/` (SQLite, migrations, `use_live_query`), `examples/storage/kv_and_sqlite.fsx`, `docs/storage.md`. | Fully implemented: sync-read cache, append-only WAL journal, crash recovery, compaction, schema migrations, live queries with table invalidations. |
| **F4** | **State Management & Forms** | ✅ Done | `crates/ferrix-store/` (fine-grained selectors, persistence), `crates/ferrix-forms/` (controlled fields, validation, versioning), `examples/forms/login_form.fsx`, `docs/state-and-forms.md`. | Fully implemented: global store, selector slice isolation, storage persistence middleware, form controller, validation rules, monotonic cursor synchronization. |
| **F5** | **Config, Env, Assets, i18n** | ✅ Done | `crates/ferrix-config/` (schema validation, `.env` layering & filtering), `crates/ferrix-i18n/` (ICU parser, plurals, select, `use_translation`), `examples/i18n/multilingual_app.fsx`, `docs/config-and-i18n.md`. | Fully implemented: TOML schema validator, multi-file .env layering, public variable filtering, ICU message formatting, reactive locale switching. |
| **F6** | **Lint, Format, LSP Tooling** | ✅ Done | `crates/ferrix-fmt/`, `crates/ferrix-lint/`, `crates/ferrix-lsp/`, `docs/tooling.md`, 9 tests passing across tooling crates. | Fully implemented: lossless idempotent formatter (`ferrix-fmt`), static linter with auto-fix rules (`ferrix-lint`), and JSON-RPC LSP server (`ferrix-lsp`). |
| **F7** | **Testing Framework** | ✅ Done | `crates/ferrix-test/`, `docs/testing.md`, `examples/testing/counter_test.rs`, 6 tests passing in `tests/test_framework_tests.rs`. | Fully implemented: headless component test renderer, Testing Library query engine (`get_by_text`, `get_by_role`, `get_by_test_id`), synthetic event dispatch, fake timers, snapshot testing, and patch assertions. |
| **F8** | **Errors, Logging, DevTools** | ✅ Done | `crates/ferrix-core/src/error_boundary.rs`, `crates/ferrix-devtools/`, `crates/ferrix-router/src/state.rs`, `DevToolsAndWorkerTest.kt`, `examples/devtools/error_boundary_and_devtools.fsx`, `docs/devtools.md`. | Fully implemented: unwind-safe `render_with_error_boundary`, route `+error.fsx` escalation, live `FpsMonitor`, `PatchInspector`, `SignalInspector`, in-app `DevToolsOverlay`, and WorkManager background job adapter. |
| **F9** | **Config Single Source of Truth** | ✅ Done | `crates/ferrix-config/src/schema.rs`, `generator.rs`, `tests/golden_tests.rs`, `tests/config_tests.rs`, `crates/ferrix-cli/src/commands/config.rs`, `commands/deeplinks.rs`, `docs/config.md`, `docs/deep-links.md`, `.agent/learnings/config.md`. | Fully implemented: Schema v1, JSON Schema, strict validation (hyphen rejection with suggestion, Java/Kotlin keywords, semver, SDK ranges, colors, deep link syntax), deterministic artifact generation (`AndroidManifest.xml`, `themes.xml`, `strings.xml`, `colors.xml`, `network_security_config.xml`, `locales_config.xml`, adaptive icons), fixed host namespace `dev.ferrix.host`, CLI commands (`ferrix config check`, `print`, `explain`, `deeplinks assetlinks/test/verify`), verified on `emulator-5554` via `ferrix dev --smoke`. |
| **F10** | **Rust Interop from FSX** | ❌ Missing | `crates/ferrix-compiler/src/ast.rs` (`ImportDecl` parsed). | Missing: codegen translation of `.rs` and `rust:<crate>` imports into Rust `use`/`mod`, type mapping and error mapping, `ferrix add crate` CLI with Android target compatibility check, binary size reporting, supply-chain verification (`deny.toml`). |

---

## 2. Baseline Verification Evidence

- **Cargo Workspace Tests**: 63 passed across 10 crates (`cargo test --workspace` passed with 0 failures).
- **Android Runtime Tests**: All Robolectric tests passed under SDK 34 (`./gradlew testDebugUnitTest` passed).
- **Architecture Invariants Intact**: Architecture A preserved (Rust owns state, reconciliation, and layout; Kotlin renders Views and platform services; zero reflection, zero JS engine).
