# Plan: Multi-File Project Scaffolding & Router Starter Template

## 1. Problem Statement
The user requested:
"where is router template,and also the project only component.fsx file not other file create template for multiple file"

Identified Gaps:
1. **Missing `router` Starter Template**: While `ferrix-router` exists and `--router` is a CLI flag, there is no official starter template named `router` in `TEMPLATES`. Users running `ferrix new my-app --template router` receive an `Unknown template 'router'` error.
2. **Single-File Restriction in Scaffolding**: All existing templates only generate a single `src/component.fsx` file, and `build.rs` / `src/lib.rs` are hardcoded to compile only `src/component.fsx`. If a user adds more files (e.g. `HomeScreen.fsx`, `DetailsScreen.fsx`), `build.rs` ignores them and `lib.rs` doesn't export them.
3. **Multi-File Architecture**: Users building real applications need multi-screen and modular component architectures where multiple `.fsx` files are automatically discovered, compiled ahead-of-time by `build.rs`, and exposed as Rust modules.

## 2. Implementation Steps

### Step 1: Multi-File Compilation Engine in `build.rs` and `src/lib.rs`
- Update the generated `build.rs` in `crates/ferrix-cli/src/templates.rs`:
  - Recursively scans `src/` for all `*.fsx` files.
  - Generates `cargo:rerun-if-changed=<file>` for each `.fsx` file.
  - Compiles each `.fsx` file to `$OUT_DIR/<stem>_generated.rs` via `ferrix_compiler::compile_file`.
  - Generates an index file `$OUT_DIR/generated_modules.rs` containing `pub mod <stem_snake_case> { include!(...); }` for every `.fsx` file.
- Update `src/lib.rs`:
  - Includes `$OUT_DIR/generated_modules.rs`.
  - Re-exports `pub use component::*;` for backwards compatibility and ease of use.

### Step 2: Implement Official `router` Template
- Add `TemplateInfo` for `router` to `TEMPLATES` in `crates/ferrix-cli/src/templates.rs`:
  - ID: `router`
  - Name: `Router`
  - Description: `Multi-file router navigation with separate modular screens`
- Add aliases in `resolve_template`: `"router" | "routes" | "routing" | "route"`.
- If `--router` flag is passed without `--template`, default to `"router"`.
- Ensure `Cargo.toml` includes `ferrix-router` and `ferrix-navigation`.
- Scaffold multiple `.fsx` files in `src/` for `router`:
  - `src/component.fsx` (Root router / StackNavigator coordinator)
  - `src/HomeScreen.fsx` (Home screen component)
  - `src/DetailsScreen.fsx` (Details screen component)
  - `src/ProfileScreen.fsx` (User profile screen component)
  - `src/Header.fsx` (Reusable header component)

### Step 3: Create Reference Template Catalog Entry (`templates/router/`)
- Create `templates/router/` with complete project files matching the other catalog entries under `templates/`.

### Step 4: Verification
- Run `cargo fmt --check`.
- Run `cargo clippy --workspace -- -D warnings`.
- Run `cargo test --workspace` (including unit tests verifying multi-file generation and compilation).
- Verify `android` build with `./gradlew testDebugUnitTest`.
- Install CLI locally: `cargo install --path crates/ferrix-cli`.
- Test `ferrix new test-router-app --template router` and verify generated file structure and compilation.

### Step 5: Update Learnings & Documentation
- Update `.agent/learnings/cli.md` and `.agent/learnings/router.md`.
