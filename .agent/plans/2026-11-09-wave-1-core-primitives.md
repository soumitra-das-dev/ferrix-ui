# Plan: Wave 1 — Core Primitives (React Native Core Parity)

**Date**: 2026-11-09
**Status**: In Progress
**Wave**: Wave 1 (Core Primitives Parity)

---

## 1. Objectives & Scope
Implement the complete core primitive component suite matching React Native core standards under Architecture A:
1. **Component Specifications** (`docs/components/`):
   - `Image.md`, `ImageBackground.md`, `TextInput.md`, `Button.md`, `ScrollView.md`, `LazyList.md`, `SectionList.md`, `Switch.md`, `ActivityIndicator.md`, `Modal.md`, `StatusBar.md`, `SafeAreaView.md`, `KeyboardAvoidingView.md`, `RefreshControl.md`, `Touchables.md`, `Portal.md`.
2. **Renderer Protocol Additions** (`ferrix-renderer-protocol`):
   - View types: `Image`, `TextInput`, `ScrollView`, `RecyclerView` (`LazyList`), `Switch`, `ActivityIndicator`, `Modal`, `StatusBar`, `SafeAreaView`, `RefreshControl`, `Portal`.
   - Prop IDs:
     - TextInput: `ControlledText { version: u32, text: String }`, `Placeholder(String)`, `PlaceholderTextColor(u32)`, `SecureTextEntry(bool)`, `KeyboardType(u8)`, `ReturnKeyType(u8)`, `MaxLength(u32)`, `Multiline(bool)`, `AutoCapitalize(u8)`, `AutoCorrect(bool)`, `Editable(bool)`, `Selection { start: u32, end: u32 }`.
     - Image: `ImageSource { uri: String, resize_mode: u8, blur_radius: f32 }`, `TintColor(u32)`.
     - ScrollView: `Horizontal(bool)`, `PagingEnabled(bool)`, `ShowsScrollIndicator(bool)`, `ContentOffset { x: f32, y: f32 }`.
     - LazyList / FlatList: `ItemCount(u32)`, `ItemHeight(f32)`.
     - Switch: `SwitchValue(bool)`, `TrackColor { active: u32, inactive: u32 }`, `ThumbColor(u32)`.
     - ActivityIndicator: `IndicatorColor(u32)`, `IndicatorSize(u8)`, `Animating(bool)`.
     - Modal: `ModalVisible(bool)`, `AnimationType(u8)`, `Transparent(bool)`.
     - StatusBar: `StatusBarStyle(u8)`, `StatusBarHidden(bool)`, `StatusBarTranslucent(bool)`.
     - SafeAreaView & KeyboardAvoidingView: insets padding configuration.
     - RefreshControl: `Refreshing(bool)`, `ProgressBackgroundColor(u32)`.
   - Event Types:
     - `Scroll`, `EndReached`, `ValueChange` (Switch), `SubmitEditing`, `SelectionChange`, `RequestClose` (Modal), `Refresh`.
3. **Core Rust Components (`ferrix-core`)**:
   - `Image`, `ImageBackground`, `TextInput` (with controlled version counter state tracking), `Button`, `ScrollView`, `LazyList` / `FlatList`, `SectionList`, `Switch`, `ActivityIndicator`, `Modal`, `StatusBar`, `SafeAreaView`, `use_safe_area_insets`, `KeyboardAvoidingView`, `RefreshControl`, `TouchableOpacity`, `TouchableHighlight`, `TouchableWithoutFeedback`, `Portal`.
4. **Kotlin Android Renderer (`android/ferrix-runtime`)**:
   - `TextInput`: IME version counter guard preventing cursor jumps.
   - `LazyList`: `RecyclerView` adapter with item view pooling and windowed rendering of 10,000 items.
   - `Switch`: `SwitchCompat` with controlled onCheckedChangeListener.
   - `ActivityIndicator`: native indeterminate `ProgressBar`.
   - `Modal`: native `Dialog` with transparent window background and hardware back button dispatch.
   - `ScrollView`: `NestedScrollView` with throttled scroll event coalescing.
   - `RefreshControl`: `SwipeRefreshLayout` with refresh listener.
   - `StatusBar`: `WindowInsetsController` management.
   - `SafeAreaView`: window insets listener padding.
5. **Kitchen Sink & Performance Gate**:
   - `examples/kitchen-sink.fsx` showcasing all core primitives.
   - 10k-item list scrolling benchmark logging bounded memory (<60 MB) and P95 frame time (<16.6ms).
   - Test verifying zero cursor jumps on controlled `TextInput`.

---

## 2. Gate Verification Criteria
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --workspace` all green.
- `./gradlew test` in `android/` passing all unit and instrumented tests.
- 10,000-item virtualized list scrolling with bounded memory and P95 frame time logged.
- Controlled `TextInput` IME typing assertion without cursor jumping.
- Learnings logged in `.agent/learnings/wave-01.md`.
