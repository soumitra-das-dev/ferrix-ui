# Plan: Automated Workspace Publishing to Crates.io on Push

## 1. Problem Statement
The user requested:
"make the workflow such as it auto publish to crates after every push"

Currently:
1. Releases are only triggered manually or on git tags (`v*`) in `.github/workflows/release.yml`.
2. Several workspace crates lack required crates.io fields (such as `description`) or omit version requirements on internal path dependencies (`path = "../..."` without `version = "0.1.0"`), which causes crates.io packaging verification to fail.
3. Crates.io does not allow republishing identical versions. An auto-publish workflow on every push must be idempotent (detect whether each crate version has already been published, and skip already-published versions cleanly).
4. Workspace publishing requires topological dependency ordering: base crates (`ferrix-renderer-protocol`, `ferrix-reactivity`, `ferrix-layout`, etc.) must be published and indexed before higher-level crates (`ferrix-core`, `ferrix-cli`).

## 2. Implementation Steps

### Step 1: Update All Workspace Crates for Crates.io Publishing Compliance
- Ensure every `crates/*/Cargo.toml` has:
  - `description = "..."`
  - `license.workspace = true`
  - `repository.workspace = true`
- Ensure all internal crate dependencies specify both version requirement and path:
  - `ferrix-<crate> = { version = "0.1.0", path = "../ferrix-<crate>" }`
- Validate locally with `cargo check --workspace` and `cargo package` dry runs.

### Step 2: Implement Auto-Publish GitHub Actions Workflow
- Create/update `.github/workflows/publish-crates.yml` (and/or integrate into CI):
  - Trigger:
    ```yaml
    on:
      push:
        branches: [ main, master ]
      workflow_dispatch:
    ```
  - Only execute after `rust-verification` and `android-verification` checks pass (or verify tests first).
  - Use `katyo/publish-crates@v2` with `secrets.CARGO_REGISTRY_TOKEN`:
    - Automatically computes topological workspace dependency order.
    - Idempotent: checks crates.io registry and skips already-published versions without failing.
    - Configures `publish-delay: 30000` to allow crates.io index propagation between dependent crates.
    - Gracefully handles missing secret (warns instead of hard-breaking if the user hasn't added `CARGO_REGISTRY_TOKEN` yet in GitHub settings).

### Step 3: Verification
- Verify `cargo fmt --check`
- Verify `cargo clippy --workspace -- -D warnings`
- Verify `cargo test --workspace`
- Verify `./gradlew testDebugUnitTest`
- Dry-run packaging on crates (`cargo package --no-verify`) to confirm manifest validity.

### Step 4: Commit & Sync
- Commit to `main`
- Sync to `master`
- Update `.agent/learnings/release.md`.
