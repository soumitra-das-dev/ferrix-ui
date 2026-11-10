# Plan: Ferrix Interactive Dev Server & Crash Elimination

## 1. Objectives & Context
- **Primary Goals**:
  1. Eliminate the `UnsatisfiedLinkError` / missing native library crash on physical devices (Moto G34 5G, `arm64-v8a`, Android 15) and ensure correct JNI library naming and packaging.
  2. Implement honest launch status detection: never declare success from `am start` alone; detect crashes, timeouts, and classify root causes with helpful remediation hints.
  3. Transform `ferrix dev` into an Expo-style interactive dev server with live key commands (`a`, `d`, `r`, `R`, `m`, `j`, `l`, `c`, `o`, `p`, `?`, `q`), multi-device picker, and unified logcat streaming.
  4. Implement tiered hot reload (Tier 1: styles/assets, Tier 2: components, Tier 3: native), recording latency benchmarks in `perf-log.md`.
  5. Provide unit tests, integration tests, and comprehensive documentation (`docs/dev-server.md`, `docs/hot-reload.md`).

---

## 2. Root Cause Analysis (Part A)
- **Reproduction**:
  - Device: Moto G34 5G (`arm64-v8a`, Android 15).
  - Error: `UnsatisfiedLinkError: dlopen failed: library "libmy-app.so" not found` at `MainActivity.<clinit>`.
  - Inspection: APK contained 0 native libraries in `lib/`.
- **Causes**:
  1. **Crate Name vs System.loadLibrary Name Mismatch**:
     `System.loadLibrary("{slug}")` requested `"my-app"`, but Cargo compiles crates with hyphens to underscore filenames (`libmy_app.so`). Android strictly looks for `libmy-app.so` when given `"my-app"`.
  2. **Silenced Compilation Failures**:
     `run.rs` and `dev.rs` silently fell back to host `cargo build` if `cargo ndk` failed or target ABI was mismatched, resulting in an empty `android/app/src/main/jniLibs` directory.
  3. **Premature Success Declaration**:
     The CLI declared success as soon as `adb shell am start` returned 0, ignoring immediate process crashes in `<clinit>`.
  4. **JNI Identifier Mangling**:
     JNI specification dictates that underscores in Java package/class identifiers must be encoded as `_1` in native C symbols.

---

## 3. Architecture & Implementation Plan

### Step 1: Fix Crash & Scaffolding Engine (Part A)
- In `templates.rs`:
  - Set explicit `[lib] name = "{pkg_name}"` in `Cargo.toml`.
  - In `MainActivity.kt`, always invoke `System.loadLibrary("{pkg_name}")` (e.g., `"my_app"`) inside a safeguarded block.
  - Implement JNI symbol mangling utility function and regression unit tests (`_` -> `_1`).
  - Update `build.gradle.kts` to package `jniLibs` for all active targets (`arm64-v8a`, `x86_64`).

### Step 2: Robust ABI Resolution & Compilation
- In `crates/ferrix-cli/src/commands/`:
  - Query device ABI using `getprop ro.product.cpu.abilist` first, then `ro.product.cpu.abi`.
  - Prioritize `arm64-v8a` -> `aarch64-linux-android`, `x86_64` -> `x86_64-linux-android`, `armeabi-v7a` -> `armv7-linux-androideabi`.
  - Require `cargo ndk` to succeed; fail fast with actionable guidance if compilation fails or NDK targets are missing.
  - Assert that `android/app/src/main/jniLibs/<abi>/lib<pkg_name>.so` exists before calling Gradle `assembleDebug`.

### Step 3: Honest Launch Status & Crash Classifier (Part B)
- Monitor logcat by package name and PID.
- Wait for runtime handshake / `ferrix:ready` marker within configurable timeout (default 30s).
- Crash classifier:
  - Parse `FATAL EXCEPTION`, `UnsatisfiedLinkError`, `SIGSEGV`, `SIGABRT`, `panic`, `StrictMode`, `ANR`.
  - Print categorized diagnosis and hint:
    - `UnsatisfiedLinkError` -> Library/ABI mismatch or JNI signature issue.
    - `panic` -> Rust source file and line.
    - `Theme.AppCompat` -> Manifest theme configuration error.
  - Prompt: `Press r to relaunch, R to rebuild, or q to quit`.

### Step 4: Expo-Style Interactive Dev Server (Part C)
- Architecture:
  - Terminal raw mode listener for keypresses:
    - `a`: open on Android / device prompt
    - `d`: select device/emulator
    - `r`: reload (fastest tier)
    - `R`: rebuild + reinstall + launch
    - `m`: dev menu (`input keyevent 82`)
    - `j`: devtools/inspector
    - `l`: toggle log filtering (app-only / all / errors)
    - `c`: clear console
    - `o`: open in editor
    - `p`: toggle perf overlay
    - `?`: display help
    - `q`: graceful exit
  - Dev Channel:
    - Embedded lightweight HTTP / WebSocket dev server on port `8081` (configurable via `--port`).
    - Setup `adb reverse tcp:8081 tcp:8081`.
    - Handshake with session token for security (debug builds only).
  - Multi-device discovery:
    - Parse `adb devices -l` and enumerate physical devices and emulators.
    - Persist selected device in `.ferrix/dev-state.json`.

### Step 5: Tiered Hot Reload (Part D)
- **Tier 1 (Style/Config/Asset)**: Push style updates over the dev channel; apply patches without Rust recompilation (<300 ms).
- **Tier 3 (Native)**: Full incremental build and `installDebug` when `AndroidManifest.xml` or Gradle files change.
- **Tier 2 (Component)**: Incremental component rebuild and reload with state preservation.
- Error overlay on device and CLI console.

### Step 6: Testing & Verification
- Unit tests:
  - Key command dispatcher.
  - Crash classifier regex and hint generation.
  - JNI symbol mangling (`test_jni_symbol_mangling_underscores`).
  - Device selection and ABI resolution against mock adb outputs.
- Integration tests:
  - Execute `ferrix dev --smoke` on emulator and verify all 9 steps pass.
  - Deliberate crash fixture verifying the crash classifier.
- Documentation:
  - Create `docs/dev-server.md` and `docs/hot-reload.md`.
  - Update `docs/cli.md` and `README.md`.
  - Record learnings in `.agent/learnings/dev-server.md`.
