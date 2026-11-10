# Learning: Dev Server & Crash Elimination (`.agent/learnings/dev-server.md`)

## 1. Context
During physical device verification on Motorola Moto G34 5G (Android 15, `arm64-v8a`), running `ferrix dev` generated an app that immediately crashed with:
`java.lang.UnsatisfiedLinkError: dlopen failed: library "libmy-app.so" not found`
at `com.example.my_app.MainActivity.<clinit>`.
Simultaneously, the CLI printed `Application launched successfully` and became non-interactive in watch mode.

---

## 2. Findings & Root Causes

### Root Cause 1: Hyphen vs. Underscore Shared Object Naming
- In Cargo/Rust toolchains, a package named `my-app` produces a shared object named `libmy_app.so` on Linux/Android platforms (Cargo converts hyphens to underscores).
- Android's `System.loadLibrary("my-app")` literally seeks `libmy-app.so` in `lib/<abi>/` within the APK and fails with `UnsatisfiedLinkError`.
- **Fix**:
  1. In `templates.rs`, explicitly set `[lib] name = "{pkg_name}"` where `pkg_name` is `slug.replace('-', "_")`.
  2. In `MainActivity.kt`, invoke `System.loadLibrary("{pkg_name}")` (e.g., `"my_app"`) and never the hyphenated slug.

### Root Cause 2: Missing `lib/` in APK Due to Silenced NDK Failures
- `run.rs` and `dev.rs` previously fell back to host `cargo build` when `cargo ndk` returned a non-zero exit code.
- Host compilation placed libraries in `target/debug/` instead of `android/app/src/main/jniLibs/<abi>/`.
- Consequently, Gradle built an APK with an empty `lib/` directory.
- **Fix**:
  1. Enforce strict error handling: fail fast if `cargo ndk` fails, providing actionable remediation (`rustup target add <target>`).
  2. Assert that `android/app/src/main/jniLibs/<abi>/lib<pkg_name>.so` exists before invoking Gradle `assembleDebug`.

### Root Cause 3: False Positive Launch Reporting
- `am start` returns exit code 0 as soon as the activity launch is scheduled with Android's system server, even if the target process crashes 20 ms later in `<clinit>`.
- **Fix**:
  1. Never declare success from `am start` alone.
  2. Implement honest launch status: stream logcat lines for the app package, parse for `ferrix:ready`, and detect fatal crashes using `classify_crash`.
  3. Classify crashes with actionable diagnostic hints (`UnsatisfiedLinkError`, `Rust panic`, `Theme mismatch`, `SIGSEGV`, `ANR`).

### Root Cause 4: JNI Symbol Mangling for Packages with Underscores
- In Oracle's JNI specification, underscores in Java identifiers must be encoded as `_1` in C function symbols (e.g., `com.example.my_app.MainActivity.nativeInit` -> `Java_com_example_my_1app_MainActivity_nativeInit`).
- **Fix**:
  1. Created `crates/ferrix-cli/src/jni.rs` with `mangle_jni_symbol` and comprehensive unit tests.

---

## 3. Rules Going Forward
1. **Explicit Crate Lib Names**: Always specify `[lib] name = "{pkg_name}"` in scaffolded `Cargo.toml` so shared library output filenames match Kotlin `System.loadLibrary` calls with 100% determinism.
2. **Never Silence NDK Failures**: Treat NDK compilation errors as hard build errors. An APK without native libraries must never be installed or launched.
3. **Honest Launch Invariant**: Always confirm `ferrix:ready` from the runtime before declaring application launch success.
4. **Target ABI Discovery**: Query `ro.product.cpu.abilist` first, then `ro.product.cpu.abi`, to dynamically select the correct compilation target (`aarch64-linux-android`, `x86_64-linux-android`, `armv7-linux-androideabi`).

---

## 4. Evidence
- **Smoke test verified clean**:
  `🎉 Smoke test passed in 35.56s with zero errors!`
- **Underscore package verified clean**:
  `sample_org_app` scaffolded, compiled to `libsample_org_app.so`, installed, and launched with `ferrix:ready` confirmed on `emulator-5554`.
- **Unit test coverage**:
  All 15 tests in `ferrix-cli` passing (`cargo test -p ferrix-cli`).

