# Plan: Configuration as Single Source of Truth & Rust Interoperability

## 1. Overview & Objectives
This plan covers two major framework capabilities:
1. **PART 1: `ferrix.config.toml` as Single Source of Truth**:
   - Formal schema v1 with published JSON Schema and `schema_version = 1`.
   - Comprehensive app metadata: ID, name, version, icon, splash, deep links, permissions, orientation, variants, signing.
   - Precise validation: reject hyphens in Android `app.id` (suggest `_`), forbid Java/Kotlin keywords, validate semver, SDK ranges, colors, deep link syntax.
   - Fixed host namespace `dev.ferrix.host` to decouple JNI symbols from user application IDs.
   - Deterministic generation of `AndroidManifest.xml`, resources, splash, adaptive icons, Gradle flavors, intent filters.
   - CLI commands: `ferrix config check`, `ferrix config print`, `ferrix config explain <key>`, `ferrix deeplinks assetlinks/test/verify`.
   - Gate 1 verification.
2. **PART 2: Rust Interop from `.fsx`**:
   - Import local Rust modules: `import { fib, Counter } from "./math.rs";`.
   - Import cargo crates: `import { Uuid } from "rust:uuid";`.
   - Auto-declaration of modules and code generation in `crates/ferrix-compiler`.
   - CLI crate manager: `ferrix add crate <name>`, `ferrix remove crate`, `ferrix crates list`.
   - Android target compatibility checks on add (`aarch64-linux-android`, `x86_64-linux-android`).
   - Supply-chain security check (`deny.toml`).
   - Gate 2 verification with `examples/rust-interop/`.

---

## 2. Architectural Decision Records (ADRs)

### ADR 1: Fixed Host Namespace (`dev.ferrix.host`)
- **Context**: In previous versions, Kotlin host files were placed in `com.example.<pkg_name>`. If `<pkg_name>` had hyphens or underscores, JNI function exports and ClassLoader lookups depended on user-chosen application IDs, causing `UnsatisfiedLinkError` and mangling bugs.
- **Decision**: Decouple Android `applicationId` from the Kotlin host package. All host classes reside in `dev.ferrix.host.MainActivity` and `dev.ferrix.host.FerrixBridge`. The user's `app.id` in `ferrix.config.toml` only configures Gradle `applicationId`. JNI symbol names remain compile-time invariant (`Java_dev_ferrix_host_FerrixBridge_...`).

### ADR 2: Deterministic VersionCode Generation
- **Context**: Google Play requires strictly increasing positive integer `versionCode` values.
- **Decision**: `versionCode = major * 1_000_000 + minor * 1_000 + patch`. For `1.4.2`, `versionCode = 1_004_002`. Can be overridden by setting `FERRIX_VERSION_CODE` in CI.

### ADR 3: Module Auto-Declaration for Local `.rs` Files
- **Context**: When importing `./math.rs` into `.fsx`, developers should not need to edit `lib.rs` manually.
- **Decision**: The compiler scans imports, locates relative `.rs` files, and generates `#[path = ".../math.rs"] pub mod math; use math::*;` in the synthesized Rust component file.

### ADR 4: Compiler Strategy for Crates (`rust:<crate>`)
- **Context**: Re-implementing Rust type checking inside the FSX compiler would create maintenance drift.
- **Decision**: The FSX compiler emits standard Rust `use <crate>::{...};` and lets `rustc` be the ultimate source of truth. Rustc JSON diagnostics (`--error-format=json`) are mapped back to `.fsx` source spans.

### ADR 5: Ownership Model for `.fsx` $\leftrightarrow$ Rust
- **Context**: Rust values used in reactive `.fsx` components must maintain memory safety.
- **Decision**: Values are native Rust values. Parameters are auto-borrowed when `&T` or `&mut T` is expected. Mutating state values requires `useState::update`. Panics inside imported Rust functions are caught by `std::panic::catch_unwind` and routed to error boundaries without crossing JNI.

### ADR 6: Inline Rust Blocks
- **Context**: Small helper functions are sometimes convenient directly in `.fsx`.
- **Decision**: Top-level `rust { ... }` blocks are parsed and emitted directly into the generated component module above UI components.

---

## 3. Implementation Plan

### Part 1: Config as Single Source of Truth
1. **Schema & Validation (`crates/ferrix-config`)**:
   - Add `schema_version`, `[app]`, `[android]`, `[icon]`, `[splash]`, `[deep_links]`, `[build.variants]`, `[locale]`, `[signing]`, `[plugins]`.
   - Implement strict validation: check segment count, identifier syntax, Java/Kotlin reserved words, reject hyphens in `app.id` with `_` suggestions.
   - Generate JSON Schema (`ferrix.schema.json`).
2. **Generation Pipeline (`crates/ferrix-config/src/generator.rs` & `ferrix-cli`)**:
   - Generate `AndroidManifest.xml` with fixed host namespace `dev.ferrix.host` and user `app.id`.
   - Generate `strings.xml`, `themes.xml`, `styles.xml` (Splash screen API), `network_security_config.xml`, `locales_config.xml`.
   - Generate intent filters for custom schemes and Android App Links (`autoVerify=true`).
   - Generate Gradle product flavors and variant configurations.
3. **CLI Commands (`crates/ferrix-cli/src/commands/config.rs` & `deeplinks.rs`)**:
   - `ferrix config check`: Validate `ferrix.config.toml` with span-aware errors.
   - `ferrix config print`: Output resolved configuration for active variant.
   - `ferrix config explain <key>`: Explain key purpose and defaults.
   - `ferrix deeplinks assetlinks`: Print `assetlinks.json`.
   - `ferrix deeplinks test <url>`: Test launching deep link via ADB.
   - `ferrix deeplinks verify`: Query `pm get-app-links`.
4. **Verification & Gate 1**:
   - Golden tests: config $\to$ generated manifest & resources.
   - Build test: launch with new config, splash, deep link.
   - Log learnings to `.agent/learnings/config.md`.

### Part 2: Rust Interop from `.fsx`
1. **Compiler Extensions (`crates/ferrix-compiler`)**:
   - Update AST, lexer, and parser to handle:
     - `import { ... } from "./path.rs";`
     - `import { ... } from "rust:crate";`
     - `import { ... } from "rust:crate::subpath";`
     - `rust { ... }` inline blocks.
   - Update `codegen.rs` to emit module includes and `use` statements.
2. **Crate Management CLI (`crates/ferrix-cli/src/commands/crate_cmd.rs`)**:
   - `ferrix add crate <name> [--features ...]` wrapping `cargo add`.
   - Run Android compilation check immediately after adding.
   - Display binary size impact and supply-chain warnings (`deny.toml`).
   - `ferrix remove crate <name>` and `ferrix crates list`.
3. **Verification & Gate 2**:
   - Create `examples/rust-interop/` app with local `.rs` and crates (`uuid`, `regex`, `serde_json`).
   - Verify on device/emulator with `--smoke`.
   - Log learnings to `.agent/learnings/rust-interop.md`.

