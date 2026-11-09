# Plan: Wave 2 — Extended UI Primitives

**Date**: 2026-11-09
**Author**: Antigravity Pair Programmer
**Status**: In Progress

---

## 1. Goal
Implement Wave 2 of the Ferrix component library and native platform module layer under Architecture A:
- Layout helpers: `Stack` / `ZStack`, `Spacer`, `Divider`, `Center`, `Grid`.
- Inputs: `Checkbox`, `RadioGroup`, `Slider`.
- Overlays & Display: `ProgressBar`, `Badge`, `LinearGradient`, `Icon`.
- Protocol expansion: typed properties & events for sliders, progress, gradients, and checkboxes with zero-allocation binary encode/decode.
- Android View mapping with pooling and event routing.
- Comprehensive Rust + Robolectric test coverage and FSX compilation.

---

## 2. Implementation Steps
1. **Renderer Protocol (`crates/ferrix-renderer-protocol`)**:
   - Add `ViewType` variants: `Stack` (18), `Spacer` (19), `Divider` (20), `Center` (21), `Grid` (22), `Checkbox` (23), `RadioGroup` (24), `Slider` (25), `Picker` (26), `ProgressBar` (27), `Badge` (28), `LinearGradient` (29), `Icon` (30).
   - Add `Property` variants 68..=76: `Checked`, `SliderValue`, `SliderRange`, `Progress`, `GradientColors`, `IconName`, `IconSize`, `BadgeContent`, `GridColumns`.
   - Add `EventType::SliderChange` (15).
   - Binary encode/decode + unit tests.
2. **Android Runtime (`android/ferrix-runtime`)**:
   - Update `Types.kt` and `CommandDecoder.kt`.
   - Update `FerrixViewFactory.kt` (`CheckBox`, `RadioGroup`, `SeekBar`/`Slider`, `ProgressBar`, `GradientDrawable`, etc.).
   - Update `FerrixRenderer.kt` property setters and event listeners.
3. **Core Rust Components (`crates/ferrix-core`)**:
   - Add builders in `crates/ferrix-core/src/primitives.rs`:
     - `Stack`, `Spacer`, `Divider`, `Center`, `Grid`.
     - `Checkbox`, `RadioGroup`, `Slider`.
     - `ProgressBar`, `Badge`, `LinearGradient`, `Icon`.
   - Re-export in `crates/ferrix-core/src/lib.rs` and `prelude`.
4. **Verification & Benchmarks**:
   - Rust integration tests in `crates/ferrix-core/tests/primitives_wave2_tests.rs`.
   - Robolectric tests in `android/ferrix-runtime/src/test/java/com/ferrix/runtime/Wave2ExtendedPrimitivesTest.kt`.
   - FSX compilation test in `crates/ferrix-compiler/tests/compiler_tests.rs` with `examples/extended-kitchen-sink.fsx`.
   - Run `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --workspace`, and `./gradlew test`.
5. **Log Learnings & Gate Pass**:
   - Document decisions and learnings in `.agent/learnings/wave-02.md`.
   - Mark Wave 2 as Passed in `.agent/milestones.md` and `docs/rn-parity.md`.
   - Commit `feat(wave-2): extended ui primitives`.

