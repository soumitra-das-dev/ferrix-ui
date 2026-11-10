# Release & Distribution Learnings

## Distribution Pipeline & Artifacts
The Ferrix framework is distributed through four synchronized channels targeting `https://github.com/soumitra-das-dev/ferrix-ui`:

### 1. Multi-Architecture Prebuilt Binaries
- Built via GitHub Actions (`.github/workflows/release.yml`) for:
  - Linux `x86_64-unknown-linux-gnu`
  - Linux `aarch64-unknown-linux-gnu` (via `cross`)
  - macOS `x86_64-apple-darwin` (Intel)
  - macOS `aarch64-apple-darwin` (Apple Silicon M-series)
  - Windows `x86_64-pc-windows-msvc.exe`
- Attached automatically to GitHub Releases matching tag pattern `v*`.

### 2. Standalone Shell & PowerShell Installers
- `install.sh`: POSIX script detecting OS and CPU architecture, downloading the corresponding release binary, placing it into `~/.ferrix/bin`, and configuring the user's `~/.bashrc` / `~/.zshrc`.
- `install.ps1`: Windows PowerShell script downloading `ferrix.exe` and configuring the user `Path` environment variable.

### 3. npm Launcher (`create-ferrix-app`)
- Published under `packages/create-ferrix-app`.
- Enables running `npx create-ferrix-app my-app` without manual toolchain installation.
- Automatically checks local PATH for `ferrix` binary, downloads the prebuilt binary from GitHub Releases if missing, or falls back to `cargo install ferrix-cli`.

### 4. Git Repository & Remote
- Configured git remote origin:
  `origin  https://github.com/soumitra-das-dev/ferrix-ui.git`
- Structured CI workflow runs formatting, clippy, Rust workspace tests, and Android Robolectric tests on every PR and push.

### 5. Automated Crates.io Publishing on Push
- Configured in `.github/workflows/ci.yml` via the `publish-crates` job.
- **Trigger**: Runs on every `git push` to `main` and `master`, as well as manual trigger (`workflow_dispatch`), conditioned on passing `rust-verification` and `android-verification`.
- **Secret Detection**: Checks for either `CARGO_REGISTRY_TOKEN` or `CRATES_IO_TOKEN` in repository secrets. If neither is present, emits a GitHub Actions `::warning::` without failing CI.
- **Topological Ordering & Idempotency**:
  - Uses `katyo/publish-crates@v2` with `ignore-unpublished-changes: true` and `publish-delay: 30000`.
  - Crates are analyzed and published according to their internal dependency graph DAG (e.g. `ferrix-renderer-protocol`, `ferrix-reactivity` first, followed by `ferrix-core`, `ferrix-cli`, and `ferrix-android`).
  - Idempotent: Skips already-published versions cleanly without failing the run.
- **Path Dependency Version Rule**:
  - Crates.io refuses packages containing `{ path = "..." }` dependencies unless an explicit `version = "..."` is declared alongside it.
  - All workspace crates must declare internal dependencies as `{ version = "0.1.0", path = "../ferrix-<name>" }`.
  - All crates must specify `description`, `license.workspace = true`, and `repository.workspace = true`.
