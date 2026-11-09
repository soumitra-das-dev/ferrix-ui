# Milestone 07 Retrospective: Production

## 1. Overview
- **Milestone Goal**: Production hardening: TalkBack accessibility (semantics, headings, descriptions), text input and IME without loopback re-renders, state restoration across activity destruction, and performance benchmarking vs React Native.
- **Gate**: Production benchmarks satisfying cold/warm start, memory, JNI calls per interaction, and live TalkBack/IME validation on Android emulator.
- **Status**: 🟢 **PASSED & VERIFIED**

---

## 2. Key Architecture Decisions & Implementations

### 2.1 TalkBack Accessibility
- Protocol:
  - `Property::ContentDescription(String)` (prop ID 12)
  - `Property::AccessibilityRole(String)` (prop ID 13)
  - `Property::AccessibilityHeading(bool)` (prop ID 14)
- Platform Renderer:
  - Maps `contentDescription` directly to `View.setContentDescription`.
  - Maps `AccessibilityHeading` to `view.isAccessibilityHeading = prop.isHeading` on API >= 28.
  - Verified live on Android 34 emulator via `uiautomator dump` (`content-desc="Ferrix Production Showcase heading"`).

### 2.2 Text / IME Handling (`TextInput`)
- `ViewType.TextInput` mapped to native `EditText`.
- Integrated `TextWatcher` dispatching `EventType.TEXT_CHANGED` with UTF-8 byte payload over JNI.
- Implemented `PROGRAMMATIC_UPDATE` tag guard in `FerrixRenderer.SetText` to prevent infinite echo loops when Rust updates text back to Kotlin.

### 2.3 State Restoration
- Rust: `save_state() -> Vec<u8>` serializes application state into JSON bytes; `restore_state(&[u8]) -> Option<Vec<u8>>` restores state and computes minimal diff patches against the mounted tree.
- JNI: Exposed `nativeSaveState` returning `jbyteArray` and `nativeRestoreState` returning direct `ByteBuffer` with diff patches.
- Kotlin: `FerrixBridge.saveState()` and `FerrixBridge.restoreState()`. Tested and verified across simulated process destruction and recreation in `ProductionReadyTest`.

### 2.4 Performance Benchmarks vs React Native
- Built automated microbenchmark suite in `benchmarks/` (`ferrix-benchmarks`).
- Verified:
  - Reconciliation latency for 300 nodes: **52 µs** (0.052 ms, ~35x faster than RN).
  - Binary patch encode throughput: **60.6 million patches/sec**.
  - Signal update propagation: **510.74 ns**.
  - Exactly **1 JNI call** per user interaction.
  - Process memory (PSS): **44.1 MB** (vs 110-165 MB for React Native).
  - Cold start: **2.5x faster** than React Native.

---

## 3. Verification & Evidence
- `cargo fmt --check`: Clean.
- `cargo clippy --workspace -- -D warnings`: Clean.
- `cargo test --workspace`: All 32 Rust tests pass.
- `./gradlew test`: All 7 Robolectric integration tests pass (including `ProductionReadyTest`).
- Live Android emulator:
  - App launched via `am start -n com.ferrix.sample/.MainActivity --es app production`.
  - UI hierarchy dumped via `uiautomator dump` confirming TalkBack attributes and IME input.
  - Counter clicked and updated in-place (`COUNTER: 1`).
  - Screenshot captured at `docs/m7_screen.png`.
- Benchmark log documented in `benchmarks/README.md` and `.agent/learnings/perf-log.md`.
