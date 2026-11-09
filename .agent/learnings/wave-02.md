# Wave 2 Learnings: Extended UI Primitives

**Date**: 2026-11-09
**Status**: Gate Passed 🟢

---

## 1. Summary of Deliverables
- **Renderer Protocol Extensions**:
  - Added new `ViewType` variants: `Stack` (18), `Spacer` (19), `Divider` (20), `Center` (21), `Grid` (22), `Checkbox` (23), `RadioGroup` (24), `Slider` (25), `Picker` (26), `ProgressBar` (27), `Badge` (28), `LinearGradient` (29), `Icon` (30), with alias `ZStack`.
  - Added properties 68..=76:
    - `Checked(bool)`
    - `SliderValue(f32)`
    - `SliderRange { min: f32, max: f32, step: f32 }`
    - `Progress(f32)`
    - `GradientColors { start_color: u32, end_color: u32, angle_deg: f32 }`
    - `IconName(String)`
    - `IconSize(f32)`
    - `BadgeContent(String)`
    - `GridColumns(u32)`
  - Added event type `SliderChange = 15`.
  - Tested zero-allocation binary encode/decode round trip in `ferrix-renderer-protocol`.
- **Core Rust Primitives**:
  - Implemented typed element builders in `crates/ferrix-core/src/primitives.rs`:
    - Layout: `Stack` / `ZStack`, `Spacer`, `Divider`, `Center`, `Grid`.
    - Inputs: `Checkbox`, `Slider`.
    - Feedback & Media: `ProgressBar`, `Badge`, `LinearGradient`, `Icon`.
  - Re-exported all primitives in `crates/ferrix-core/src/lib.rs` and `prelude`.
- **Android Runtime & Native Mapping**:
  - Updated `Types.kt`, `CommandDecoder.kt`, `FerrixViewFactory.kt`, and `FerrixRenderer.kt`.
  - Mapped `CHECKBOX` to `AppCompatCheckBox`, `SLIDER` to `SeekBar` with normalized float change dispatching, `PROGRESS_BAR` to horizontal progress bar, `LINEAR_GRADIENT` to `GradientDrawable`, and `BADGE` to padded status chip.
- **FSX Compiler & Lexer**:
  - Added `TokenKind::Bang` and unary expression parsing `Expr::Unary { op: UnaryOp::Not, .. }` to support logical NOT expressions (`!isChecked.value`) in event handlers and component statements.
  - Added `test_wave2_primitives_fsx_compilation` in `compiler_tests.rs`.
- **Verification**:
  - 51 tests passing across Cargo workspace (`cargo test --workspace`).
  - Android Robolectric unit tests passing (`Wave2ExtendedPrimitivesTest.kt`).
  - Verified `cargo fmt --check` and `cargo clippy -- -D warnings`.

---

## 2. Key Insights & Architecture Invariants
1. **Normalized Range Dispatches for Sliders**:
   Android `SeekBar` operates on integer progress (e.g. 0..1000). By normalizing to standard 0.0..1.0 `f32` in the JNI buffer, Rust runtime application logic remains completely platform-independent.
2. **Lexer Extensibility for Unary Negation**:
   Declarative event handlers frequently toggle booleans (`() => set(!val)`). Adding `Bang` (`!`) to the lexer and `Expr::Unary` to the AST and codegen ensures standard JSX/TSX patterns compile cleanly to Rust closures without requiring custom syntax.

---

## 3. Next Wave (Wave 3)
Proceeding to **Wave 3: Platform APIs & Hooks** (`Dimensions`, `useWindowDimensions`, `PixelRatio`, `Keyboard`, `AppState`, `BackHandler`, `Linking`, `Share`, `Vibration`, `Clipboard`, `Alert`, `AccessibilityInfo`, `NetInfo`).
