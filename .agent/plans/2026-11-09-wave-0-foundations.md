# Plan: Wave 0 — Cross-Cutting Systems & Architecture Foundations

**Date**: 2026-11-09
**Status**: In Progress
**Wave**: Wave 0 (Foundations)

---

## 1. Objectives & Scope
Wave 0 establishes the bedrock systems required by all primitive components in Ferrix:
1. **RN Parity Tracker**: Create `docs/rn-parity.md` documenting current parity against React Native Core + Expo SDK.
2. **Component Specs Directory**: Create `docs/components/` and define specifications for primitives.
3. **Text Measurement ADR**: Record `ADR-0003` in `.agent/learnings/decisions.md` evaluating text shaping vs batched native `StaticLayout` measurement.
4. **Style System (`ferrix-layout` & `ferrix-core`)**:
   - Complete `ferrix-layout` integration with Taffy 0.7: flexbox layout calculation, layout caching, RTL flipping, and custom intrinsic measurement function.
   - `Style`, `StyleSheet.create(...)`, style merging (`StyleValue`, arrays), color parsing (hex, rgb, hsl, named, dynamic), dp/sp/px scaling.
5. **Renderer Protocol Extensions (`ferrix-renderer-protocol`)**:
   - Extended `ViewType` (View, Text, Image, TextInput, Pressable, ScrollView, etc.).
   - Extended `Property` (all flexbox, margins, paddings, borders, colors, typography, transforms, accessibility, testID).
   - Typed imperative commands: `FocusNode`, `BlurNode`, `MeasureNode`, `ScrollToNode`.
   - Intrinsic text measure batch request/response protocol.
6. **Kotlin Android Runtime Updates (`android/ferrix-runtime`)**:
   - Native View pooling and recycling (`FerrixViewPool`).
   - Frame layout applicator: apply Taffy-computed bounds `(x, y, width, height)` directly to views with zero competing Kotlin layout.
   - Batched `StaticLayout` intrinsic text measurement service with LRU cache.
   - Gesture & responder arbitration: `Pressable` touch handling, ripple drawable, `hitSlop`, nested responder negotiation.
   - Accessibility & `testID` application on native views.
   - Imperative command handler (`focus`, `blur`, `scrollTo`, `measure`).
7. **Theming & i18n**:
   - `useColorScheme`, dynamic colors, `I18nManager` (RTL detection and layout direction).
8. **Refs & Keyed Fragments**:
   - `<Fragment>` transparent container support.
   - `useRef` handle binding and command dispatch.

---

## 2. Gate Verification Criteria
- Styled `View` + `Text` rendered with intrinsic size computed via Taffy.
- `Pressable` interactive pressed-state styling with ripple effect.
- Zero per-node JNI chatter (batch verification via counters).
- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --workspace` all passing.
- Android tests in `android/ferrix-runtime` passing.
- Learnings logged in `.agent/learnings/wave-00.md`.

---

## 3. Implementation Steps

### Step 1: Parity Board & ADR
- Write `docs/rn-parity.md` mapping out all primitives and platform modules.
- Write `docs/components/View.md`, `docs/components/Text.md`, `docs/components/Pressable.md`.
- Record `ADR-0003` in `.agent/learnings/decisions.md`.

### Step 2: Protocol Extensions (`ferrix-renderer-protocol`)
- Add new `ViewType` variants: `View`, `Pressable`, etc.
- Add extended `Property` variants covering flexbox, visual styling, transforms, and testID.
- Add typed imperative command definitions (`Command`, `BatchCommand`).
- Add intrinsic text measurement protocol structs.
- Unit test protocol serialization and roundtrips.

### Step 3: Layout Engine (`ferrix-layout`)
- Implement Taffy-based layout tree builder in `crates/ferrix-layout`.
- Map Ferrix `Style` to `taffy::style::Style`.
- Support RTL layout resolution (`start`/`end` mapping).
- Support intrinsic text size measure callbacks in Taffy.
- Expose layout calculation returning node frames: `(NodeId, Rect)`.

### Step 4: Core Style & Element System (`ferrix-core`)
- Implement `Style`, `FlexDirection`, `AlignItems`, `JustifyContent`, `Color`, etc.
- Implement `StyleSheet.create(...)` and style merge rules.
- Add `testID`, `nativeID`, accessibility attributes, and refs to `Element`.
- Implement `Fragment` element definition.
- Implement `useColorScheme()`, `I18nManager`.

### Step 5: Android Runtime Renderer Updates
- Update `FerrixRenderer.kt` to handle new properties and apply computed frames directly without layout passes.
- Implement `FerrixViewPool` for recycling views.
- Implement `StaticLayout` text measurement provider.
- Implement `PressableView` gesture responder with ripple and state dispatch.
- Implement imperative command handler.

### Step 6: Verification & Gate Assertion
- Create unit & integration tests verifying style computation, text measurement, and pressable events.
- Assert 1 JNI call per commit and zero per-node measure chatter.
- Log learnings in `.agent/learnings/wave-00.md`.

