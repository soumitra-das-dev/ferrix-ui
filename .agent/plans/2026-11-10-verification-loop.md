# Plan: Autonomous Verify-and-Fix Convergence Loop

**Date**: 2026-11-10
**Author**: Antigravity Pair Programmer
**Target**: Autonomous Template Convergence Across All Starter Templates

---

## 1. Goal
Execute an autonomous verify-and-fix convergence loop over every Ferrix template until two consecutive full-matrix passes have zero errors and zero warnings, every GitHub Actions workflow is green on the pushed commit, and a final pass from a fresh clone with the CI-built artifact is clean.

---

## 2. Secure Sandbox Specification
- **Isolation**: Fresh temporary directory `/tmp/ferrix-sandbox/my-app` per case; clean up after each run.
- **Environment**: User `soumo`, Linux x86_64 with `/dev/kvm` hardware acceleration.
- **Headless Emulator**: AVD `ferrix_test` running headless (`-no-window -no-audio -gpu swiftshader_indirect`).
- **Toolchain**:
  - Rust 1.85.0-nightly / stable targets (`aarch64-linux-android`, `x86_64-linux-android`).
  - OpenJDK 21 (`/home/soumo/.local/share/jvm/jdk-21.0.13+9`).
  - Android SDK (`/home/soumo/.local/share/android-sdk`), NDK 26.3.11579264.
  - Gradle 8.7.
- **Network Egress**: Restricted to crates.io, Android/Google Maven, Gradle distributions, npm (if required), and github.com. No credentials printed or logged.

---

## 3. Precondition: `ferrix dev --smoke` & CLI Enhancements
1. **`ferrix dev --smoke`**:
   - Add flags: `--smoke` (bool), `--timeout [secs]` (default 300), `--no-watch` (bool).
   - In `--smoke` mode:
     - Compile Rust JNI cdylib for target ABI into `android/app/src/main/jniLibs/<abi>/lib<pkg_name>.so`.
     - Build and install APK via Gradle `installDebug` (or `assembleDebug` + `adb install -r`).
     - Clear logcat and launch MainActivity via ADB.
     - Monitor logcat for `ferrix:ready`.
     - Fail fast on logcat signals: `FATAL EXCEPTION`, `ANR`, `UnsatisfiedLinkError`, `panic`, `thread '...' panicked`, `ferrix:error`, `StrictMode`, `{{`.
     - Perform scripted interaction (tap button via testID or input tap).
     - Perform hot-reload check (modify scratch `.fsx` file, trigger build, assert `ferrix:patch-applied` or clean reload).
     - Exit with code 0 on success, non-zero on failure.
2. **`ferrix new`**:
   - Support `-y` / `--yes` flag to bypass interactive prompts in automated scripts.
   - Refuse non-empty directories unless `--use-existing-dir` is specified.
   - Fix `MainActivity.kt` in template scaffolding:
     - `System.loadLibrary("{pkg_name}")` (using underscore instead of hyphen to match cdylib output).
     - Emit `Log.i("ferrix", "ferrix:ready")` on resume/render.
3. **Subcommands & Aliases**:
   - `ferrix lint`: alias for `ferrix check`.
   - `ferrix style check`: add style validation command.

---

## 4. Test Matrix Derivation
- **Templates**:
  - `blank`
  - `hello-counter`
  - `tabs`
  - `stack`
  - `router`
  - `api-client`
  - `kitchen-sink`
- **Variations**:
  - Default (`--yes`)
  - All features enabled (`--router --navigation --storage --sqlite --tailwind --i18n`)
  - Minimal / all optional features disabled
  - Pairwise combinations
  - Negative cases: invalid name, directory collision refusal without `--use-existing-dir`, unknown template.
  - Interactive prompt test via pty.
- Generate `matrix.json` in `.agent/runs/<timestamp>/matrix.json`.

---

## 5. Per-Case Execution Protocol (Steps 1–9)
1. `ferrix new my-app <flags>` exits 0; verify structure and unreplaced `{{` variables.
2. `ferrix doctor` clean.
3. `ferrix fmt --check`, `ferrix lint`, `ferrix check`, `cargo clippy -D warnings`, `ferrix style check`.
4. `ferrix test` passes.
5. `ferrix build` debug APK succeeds.
6. `ferrix dev --smoke` passes on headless emulator.
7. Release build verification: `ferrix build --release --apk`.
8. GitHub Actions workflow linting / dry-run.
9. Collision check: re-run without `--use-existing-dir` fails; with `--use-existing-dir` succeeds.

---

## 6. Stop & Convergence Conditions
- **Two consecutive clean passes** across all matrix cases with zero errors and zero warnings.
- All GitHub Actions workflows green on pushed commit.
- Fresh-clone verification with CI-built artifact passes.
- Learning logs updated (`.agent/learnings/verification-loop.md`, `pitfalls.md`, `commands.md`, `perf-log.md`).

---

## 7. Results & Verification Log
*(To be populated with real command execution results)*
