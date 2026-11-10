# Plan: Production-Ready CLI, Templates, Release Packaging & Distribution

## 1. Objectives
Transform Ferrix into a shippable, production-ready framework where developers can scaffold, run, build, test, and release Android apps `create-next-app`-style, targeted for release from `https://github.com/soumitra-das-dev/ferrix-ui`.

## 2. Phase Breakdown
### Phase 1: `ferrix-cli` Full Command Suite
1. **Scaffolding (`ferrix new <name>`, `ferrix create`)**:
   - Templates: `blank`, `hello-counter`, `tabs`, `stack`, `api-client`, `kitchen-sink`.
   - Feature flags: `--router`, `--navigation`, `--storage`, `--sqlite`, `--tailwind`, `--i18n`.
   - Standalone embedded template generator producing `ferrix.config.toml`, `.env`, Rust project, and native Android Gradle wrapper + project.
2. **Environment & Toolchain Diagnostics (`ferrix doctor`)**:
   - Verification of: Rust (`rustc`, `cargo`), Android targets (`aarch64-linux-android`, `x86_64-linux-android`), Android SDK (`ANDROID_HOME`), NDK, Java (JDK 17), `adb`, `emulator`.
   - Actionable remediation advice and `--fix` where safe.
3. **Execution & Hot Dev Loop (`ferrix dev`, `ferrix run android`)**:
   - Building debug `.so` with `cargo-ndk`.
   - Deploying via `adb` or Gradle.
   - Live file watch on `*.fsx` files for fast incremental rebuild.
   - Stream logcat filtered to Rust `tracing` output.
4. **Production Build & Packaging (`ferrix build`)**:
   - Flags: `--release`, `--apk`, `--aab`, `--abi <abi>`.
   - Automatic keystore signing checks (`.env.production` / `FERRIX_KEYSTORE_*`).
5. **Quality Tooling (`ferrix check`, `ferrix fmt`, `ferrix test`, `ferrix clean`)**:
   - Runs `ferrix-lint`, `ferrix-fmt`, and `ferrix-test` directly via CLI.
6. **Module Extensibility (`ferrix add <module>`)**:
   - Injects crate dependencies, updates `ferrix.config.toml` permissions, and prints sample code snippets.
7. **Native Materialization (`ferrix prebuild`)**:
   - Materializes the standalone native Android Studio project.

### Phase 2: Template Catalog (`templates/`)
- `templates/blank/`
- `templates/hello-counter/`
- `templates/tabs/`
- `templates/stack/`
- `templates/api-client/`
- `templates/kitchen-sink/`

### Phase 3: Packaging, Installers & npm Launcher
- `packages/create-ferrix-app/`: npm package forwarding to `ferrix` binary or downloading prebuilt release.
- `install.sh`: POSIX shell installer for Linux / macOS.
- `install.ps1`: PowerShell installer for Windows.

### Phase 4: CI/CD Workflows
- `.github/workflows/ci.yml`: Full multi-platform test, fmt, clippy, Robolectric, and sample app builds.
- `.github/workflows/release.yml`: Release builder for multi-arch CLI binaries, GitHub releases, and npm publishing.

### Phase 5: Documentation & Repo Polish
- `README.md` for `soumitra-das-dev/ferrix-ui` (features, quickstart, architecture, commands).
- `docs/cli.md`: CLI documentation.
- `.agent/learnings/cli.md` & `release.md`.
