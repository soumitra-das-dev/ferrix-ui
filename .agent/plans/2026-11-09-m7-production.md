# Plan: Milestone 07 — Production

## 1. Goal
Harden Ferrix for production deployment and benchmark against industry standards:
1. **Accessibility (TalkBack)**:
   - Expose accessibility properties (`ContentDescription`, `AccessibilityRole`, `AccessibilityHeading`).
   - Apply TalkBack semantic attributes on native Android Views via `FerrixRenderer`.
2. **Text / IME**:
   - Introduce `TextInput` component with hint, text, and change listeners.
   - Handle native IME typing without infinite echo re-renders (`EventType::TextChanged`).
3. **State Restoration**:
   - Provide serialized state save and restore hooks (`nativeSaveState` / `nativeRestoreState`) to survive configuration changes and process death.
4. **Production Benchmarks & Profiling**:
   - Establish benchmark suite in `benchmarks/`:
     - JNI buffer serialization throughput & reconciliation latency.
     - Memory allocations and JNI call counts per interaction (verified <= 1).
     - APK binary footprint analysis.
     - Comparative performance analysis against React Native.

---

## 2. Architecture & Contracts

### 2.1 Accessibility Properties (`ferrix-renderer-protocol`)
- `Property::ContentDescription(String)` (prop ID 12)
- `Property::AccessibilityRole(String)` (prop ID 13)
- `Property::AccessibilityHeading(bool)` (prop ID 14)

### 2.2 Text / IME (`TextInput`)
- `ViewType::TextInput` mapped to `android.widget.EditText`
- `EventType::TextChanged` sends payload bytes containing the UTF-8 text string.
- In `FerrixRenderer`, `TextWatcher` sends updates only when user edits, ignoring programmatically applied text to avoid feedback loops.

### 2.3 State Restoration
- Rust: `FerrixAndroidRuntime::save_state() -> Vec<u8>` and `restore_state(&[u8]) -> Result<(), ...>`
- JNI:
  - `nativeSaveState(handle) -> jbyteArray`
  - `nativeRestoreState(handle, bytes) -> jboolean`
- Kotlin: `FerrixBridge.saveState(): ByteArray?` and `restoreState(state: ByteArray)`.

### 2.4 Benchmarks (`benchmarks/`)
- Criterion microbenchmarks for patch batch encoding/decoding and reconciliation.
- Performance counter verification: exactly 1 JNI call per interaction, 0 heap allocations during static frame diffs.
- `benchmarks/README.md` and `.agent/learnings/perf-log.md`.

---

## 3. Files to Touch / Create
1. `crates/ferrix-renderer-protocol/src/lib.rs` (Accessibility props & TextChanged payload)
2. `crates/ferrix-core/src/primitives.rs` (`TextInput` component & accessibility helpers)
3. `crates/ferrix-core/src/lib.rs` (Export `TextInput`)
4. `android/ferrix-runtime/src/main/java/com/ferrix/runtime/protocol/Types.kt`
5. `android/ferrix-runtime/src/main/java/com/ferrix/runtime/protocol/CommandDecoder.kt`
6. `android/ferrix-runtime/src/main/java/com/ferrix/runtime/FerrixRenderer.kt`
7. `android/ferrix-runtime/src/main/java/com/ferrix/runtime/FerrixBridge.kt`
8. `crates/ferrix-android/src/lib.rs` (Save/restore JNI and TextInput state handling)
9. `android/ferrix-runtime/src/test/java/com/ferrix/runtime/ProductionReadyTest.kt`
10. `benchmarks/` suite + `benchmarks/README.md`
11. Documentation, learnings, and milestone updates.

---

## 4. Verification Commands
- `cargo fmt --check`
- `cargo clippy --workspace -- -D warnings`
- `cargo test --workspace`
- `cd android && ./gradlew test`
- `cargo ndk -t arm64-v8a -t x86_64 -o android/ferrix-runtime/src/main/jniLibs build --release`
- `cd android && ./gradlew assembleDebug`
- Live emulator execution & screenshot `docs/m7_screen.png`.

