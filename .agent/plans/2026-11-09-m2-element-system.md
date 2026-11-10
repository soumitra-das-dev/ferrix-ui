# Plan: Milestone 02 — Element System

## 1. Goal
Implement the core Element model, builder primitives (`Column`, `Row`, `Text`, `Button`), stable Node ID allocation, initial patch batch generation, and fine-grained reconciliation/diffing in `crates/ferrix-core`. Enhance Android runtime Kotlin renderer to handle all four primitives, nested layout, and dynamic patch applications.

**Milestone 2 Gate**: A native UI displaying `Column`, `Row`, `Text`, and `Button` rendered from Rust on an Android emulator/device, with verified properties, child hierarchy, and diffing.

---

## 2. Files to Touch / Create

### Rust Crates
- `crates/ferrix-core/src/lib.rs`
- `crates/ferrix-core/src/element.rs` (Element struct, ElementKind, props, children, builders)
- `crates/ferrix-core/src/node_id.rs` (NodeId allocator with stable sequence)
- `crates/ferrix-core/src/reconcile.rs` (Tree diffing, patch generation: SetText, SetProperty, InsertNode, RemoveNode, SetVisibility)
- `crates/ferrix-core/src/primitives.rs` (Column, Row, Text, Button builders matching fsx-spec)
- `crates/ferrix-core/tests/reconcile_tests.rs` (Unit tests for tree diffing and patch generation)
- `crates/ferrix-android/src/lib.rs` (Update sample/demo runtime to expose a composite Column/Row/Text/Button tree)

### Android Runtime
- `android/ferrix-runtime/src/main/java/com/ferrix/runtime/FerrixRenderer.kt` (Ensure correct layout, margins, child positioning, and property application)
- `android/ferrix-runtime/src/test/java/com/ferrix/runtime/FerrixE2ETest.kt` (Add test for Column, Row, Text, Button tree)
- `android/sample-app/src/main/java/com/ferrix/sample/MainActivity.kt` (Verify full layout rendering)

### Documentation & Knowledge
- `.agent/learnings/runtime.md`
- `.agent/learnings/milestone-02.md`
- `.agent/milestones.md`

---

## 3. Detailed Approach & Architecture

### 3.1 Element Representation (`ferrix-core`)
An `Element` is a declarative, immutable tree node describing a UI component:
- `id`: `Option<NodeId>` (assigned during build/mounting)
- `view_type`: `ViewType` (`Column`, `Row`, `Text`, `Button`, etc.)
- `props`: Map/struct of properties (`BackgroundColor`, `TextColor`, `FontSize`, `Padding`, `Spacing`, `CornerRadius`, `Enabled`)
- `text`: `Option<String>` (for Text/Button nodes)
- `children`: `Vec<Element>` (for container nodes like Column/Row)
- `events`: `Vec<EventType>` (registered event listeners like Click)
- `key`: `Option<String>` (for stable keyed list reconciliation)

### 3.2 Builder API (Matching FSX Specification)
```rust
Column::new()
    .padding(16.0)
    .spacing(8.0)
    .background_color(0xFFF5F5F5)
    .child(
        Text::new("Ferrix Element System")
            .size(22.0)
            .color(0xFF1E1E1E)
    )
    .child(
        Row::new()
            .spacing(8.0)
            .child(Button::new("Cancel").enabled(true))
            .child(Button::new("Submit").enabled(true))
    )
    .build(&mut id_allocator)
```

### 3.3 Reconciliation & Diffing (`ferrix-core::reconcile`)
- **Initial Mount**: Traverses tree depth-first, emits `CreateNode`, `SetProperty`, `SetText`, `AttachEventListener`, and `AppendChild` into a `PatchBatch`.
- **Tree Diffing (`diff_trees`)**:
  - Compares old `Element` vs new `Element`:
    - If `view_type` or `key` differs: emits `DestroyNode` (and descendants) and `CreateNode` for replacement.
    - If identical:
      - Text diff: emits `SetText` if changed.
      - Properties diff: emits `SetProperty` for changed/added properties.
      - Children reconciliation:
        - Compares child slices.
        - Emits `InsertBefore` / `AppendChild` for additions.
        - Emits `RemoveChild` and `DestroyNode` for removals.
        - Recursively diffs matching children.

### 3.4 Android Renderer (`FerrixRenderer.kt`)
- `Column`: `LinearLayout(VERTICAL)` with support for padding and child margin spacing.
- `Row`: `LinearLayout(HORIZONTAL)` with support for padding and child margin spacing.
- `Text`: `TextView` with padding, text color, size, text.
- `Button`: `Button` with text, click listener, enabled state, background color, text color.
- Layout parameter handling:
  - Children inside `LinearLayout` get `LinearLayout.LayoutParams(WRAP_CONTENT, WRAP_CONTENT)` with appropriate margins when `Spacing` is set on the parent.

---

## 4. Risks & Mitigations
- **Android Spacing in LinearLayout**:
  - *Risk*: `dividerPadding` in Android LinearLayout only applies to drawable dividers.
  - *Mitigation*: For `Spacing`, set margins on children or use a custom divider drawable with intrinsic height/width equal to spacing. Applying top margin on children > 0 in Column and left margin on children > 0 in Row gives exact and consistent spacing.
- **Node ID reuse & collisions**:
  - *Mitigation*: Strict monotonic sequence allocator in `NodeIdAllocator`.

---

## 5. Verification Commands
1. Rust Workspace Checks:
   - `cargo fmt --check`
   - `cargo clippy --workspace -- -D warnings`
   - `cargo test --workspace`
2. NDK Cross-Compilation:
   - `cargo ndk -t arm64-v8a -t x86_64 -o android/ferrix-runtime/src/main/jniLibs build -p ferrix-android`
3. Kotlin/Android Unit & Robolectric Tests:
   - `cd android && ./gradlew test`
4. Live Android Emulator Verification:
   - `adb install -r android/sample-app/build/outputs/apk/debug/sample-app-debug.apk`
   - `adb shell am start -n com.ferrix.sample/.MainActivity`
   - `adb shell uiautomator dump /data/local/tmp/dump.xml`
   - Assert presence of `Column` (LinearLayout vertical), `Row` (LinearLayout horizontal), `Text` (TextView), and `Button` (Button).
   - Screenshot capture to `docs/m2_screen.png`.

---

## 6. Rollback Strategy
Each slice is verified independently. Changes can be reverted via git commit history.

---

## 7. Results & Verification Evidence

### 7.1 Rust Workspace Quality
- `cargo fmt --check`: Clean (0 diffs).
- `cargo clippy --workspace -- -D warnings`: Clean (0 warnings).
- `cargo test --workspace`: 10/10 tests passed.
  - `ferrix-renderer-protocol`: `test_batch_round_trip`, `test_event_round_trip`
  - `ferrix-core`: `test_allocator_reset`, `test_allocator_sequential`, `test_diff_text_mutation`, `test_diff_children_addition_and_removal`, `test_mount_patches_generation`
  - `ferrix-core/tests/element_system_tests`: `test_complex_element_tree_mount_and_reconciliation`
  - `ferrix-android`: `test_runtime_initial_batch`, `test_runtime_event_handling`

### 7.2 NDK Cross-Compilation
- Compiled `arm64-v8a` and `x86_64` targets via `cargo ndk`:
  - `android/ferrix-runtime/src/main/jniLibs/arm64-v8a/libferrix_android.so`
  - `android/ferrix-runtime/src/main/jniLibs/x86_64/libferrix_android.so`

### 7.3 Android Test Suite
- `./gradlew test`: All Robolectric and unit tests passed cleanly (`FerrixE2ETest`, `CommandDecoderTest`).
- Verified all 4 view types (`Column`, `Row`, `Text`, `Button`) instantiated and laid out natively.
- Verified interactive clicking with bidirectional JNI event dispatch and minimal `SetText` patch diff application.

### 7.4 Live Emulator Gate Verification
- Device: `emulator-5554` (Android 34 `x86_64`).
- App installed: `adb install -r android/sample-app/build/outputs/apk/debug/sample-app-debug.apk`.
- App launched: `adb shell am start -n com.ferrix.sample/.MainActivity`.
- Node hierarchy dump (`/data/local/tmp/dump_m2.xml`):
  - Root `LinearLayout` (Column) at `[0,24][320,640]`
  - Node 0: `TextView` with `text="Ferrix Element System (M2)"` bounds `[20,44][288,74]`
  - Node 1: `TextView` with `text="Demonstrating Column, Row, Text, and Button from Rust"` bounds `[20,90][300,125]`
  - Node 2: `LinearLayout` (horizontal Row) bounds `[20,141][300,189]`
    - Child 0: `Button` with `text="INCREMENT"` bounds `[20,141][108,189]`
    - Child 1: `Button` with `text="RESET"` bounds `[120,141][208,189]` (12dp spacing applied)
  - Node 3: `TextView` with `text="Counter: 0"` bounds `[20,205][102,230]` (16dp spacing applied)
- Interactive testing:
  - Tapped `INCREMENT` twice -> Counter updated to `Counter: 2`.
  - Tapped `RESET` once -> Counter updated to `Counter: 0`.
- Visual Screenshot captured: `docs/m2_screen.png`.
- **Milestone 2 Gate Passed: 100% verified.**

