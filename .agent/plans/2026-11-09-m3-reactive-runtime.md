# Plan: Milestone 03 — Reactive Runtime

## 1. Goal
Implement fine-grained reactivity in `crates/ferrix-reactivity` (signals, dependency tracking, memos, effects, batching) and component lifecycle + hooks (`useState`, `useSignal`, `useMemo`, `useEffect`, `useRef`, stable-order hook storage, keyed reconciliation) in `crates/ferrix-core`.

**Milestone 3 Gate**: Both **Counter** and **Todo** applications running in hand-written Rust (pure reactive runtime, no compiler yet), demonstrating:
1. Counter: state increment, fine-grained updates, single patch generation.
2. Todo app: dynamic item insertion, toggling completion, deletion, and keyed reconciliation with stable node IDs and cleanup.

---

## 2. Files to Touch / Create

### Reactivity Crate (`ferrix-reactivity`)
- `crates/ferrix-reactivity/src/lib.rs`
- `crates/ferrix-reactivity/src/graph.rs` (Dependency graph, observer stack, subscriber registry)
- `crates/ferrix-reactivity/src/signal.rs` (Signal<T>, ReadSignal<T>, WriteSignal<T>)
- `crates/ferrix-reactivity/src/memo.rs` (Memo<T> derived signals)
- `crates/ferrix-reactivity/src/effect.rs` (Effect with cleanup)
- `crates/ferrix-reactivity/src/batch.rs` (Batch execution & dirty queue deduplication)
- `crates/ferrix-reactivity/tests/reactivity_tests.rs` (Unit tests for signals, memos, batching, effects)

### Core Crate (`ferrix-core`)
- `crates/ferrix-core/src/component.rs` (Component trait, ComponentContext, hook slot storage)
- `crates/ferrix-core/src/hooks.rs` (useState, useSignal, useMemo, useEffect, useRef)
- `crates/ferrix-core/src/reconcile.rs` (Keyed children reconciliation with InsertBefore/RemoveChild/DestroyNode)
- `crates/ferrix-core/tests/component_hook_tests.rs` (Component render & hook stability tests)

### Hand-Written Example Apps (`examples/`)
- `examples/counter_app.rs` / `crates/ferrix-core/tests/counter_app_test.rs`
- `examples/todo_app.rs` / `crates/ferrix-core/tests/todo_app_test.rs`

### Android Host & Integration Bridge
- `crates/ferrix-android/src/lib.rs` (Support switching/hosting the interactive Todo and Counter apps)
- `android/ferrix-runtime/src/test/java/com/ferrix/runtime/FerrixE2ETest.kt` (Add Robolectric tests for Todo app: add item, toggle, delete)

### Documentation & Learnings
- `.agent/learnings/runtime.md`
- `.agent/learnings/milestone-03.md`
- `.agent/milestones.md`

---

## 3. Detailed Approach & Architecture

### 3.1 Fine-Grained Reactivity Graph (`ferrix-reactivity`)
- Thread-local active observer stack (`RefCell<Vec<ObserverId>>`).
- When a `Signal::get()` is called, if an observer is on the stack, register a bidirectional link `(SignalId <-> ObserverId)`.
- When `Signal::set(new_value)` is called:
  - If inside `batch(|| { ... })`, mark subscriber observers dirty in a thread-local dirty set.
  - If not in a batch, trigger dirty observers immediately.
- `Memo<T>`: wraps a computation, subscribes to inner signals, lazily re-evaluates when dirty.
- `Effect`: auto-tracks dependencies during execution; re-runs when signaled; calls prior cleanup before re-running.

### 3.2 Component Model & Hook Slots (`ferrix-core`)
- `ComponentContext`:
  - `instance_id: u64`
  - `allocator: NodeIdAllocator`
  - `hooks: Vec<Box<dyn Any>>`
  - `hook_cursor: usize`
  - `cleanups: Vec<Option<Box<dyn FnOnce()>>>`
- Invariant: Hooks are strictly retrieved in index order `0, 1, 2, ...`.
- Keyed Reconciliation:
  - If children have keys, `diff_children` builds an old key map:
    - Identifies matched nodes (same key and view type) and diffs them.
    - Emits `InsertBefore` for moved/new keyed nodes.
    - Emits `RemoveChild` and `DestroyNode` for absent keys.

### 3.3 Gate Applications
1. **Counter App**:
   - `useState(0)`
   - Increment button, Reset button
   - Asserts 1 patch emitted per click.
2. **Todo App**:
   - `useState<Vec<TodoItem>>`
   - Input/Add button, Toggle button, Delete button
   - Keyed items: `<TodoRow key={todo.id}>`
   - Asserts:
     - Adding a todo emits `CreateNode`, `AppendChild` with stable key.
     - Toggling emits `SetText` on completion status.
     - Removing emits `RemoveChild` and `DestroyNode`.

---

## 4. Risks & Mitigations
- **Cyclic dependency loops in effects**:
  - *Mitigation*: Limit effect iteration depth and forbid signal writes during read observer notifications.
- **Hook count mismatch on conditional invocation**:
  - *Mitigation*: Debug assertions in `ComponentContext` verifying hook cursor matches previous render hook count.

---

## 5. Verification Commands
1. Reactivity crate tests:
   `cargo test -p ferrix-reactivity`
2. Core component & hook tests:
   `cargo test -p ferrix-core`
3. Workspace checks:
   `cargo fmt --check && cargo clippy --workspace -- -D warnings && cargo test --workspace`
4. Android cross-compile & Robolectric tests:
   `cargo ndk -t arm64-v8a -t x86_64 -o android/ferrix-runtime/src/main/jniLibs build -p ferrix-android`
   `cd android && ./gradlew test`
5. Emulator Live Gate Run:
   - Install and launch Todo app on emulator.
   - Add todo item, toggle status, delete item.
   - Assert UI hierarchy and capture screenshot `docs/m3_screen.png`.

---

## 6. Rollback Strategy
Each slice compiles and passes tests before moving forward.

---

## 7. Results & Verification Evidence

### 7.1 Reactivity & Workspace Quality
- `cargo fmt --check`: Clean (0 diffs).
- `cargo clippy --workspace -- -D warnings`: Clean (0 warnings).
- `cargo test --workspace`: 18/18 tests passed.
  - `ferrix-reactivity`: `test_signal_basic_get_set`, `test_memo_derived_computation`, `test_batching_deduplication`, `test_effect_cleanup_execution`
  - `ferrix-core`: `test_component_context_use_state_and_use_memo`, `test_component_context_use_effect_and_cleanup`, `test_component_context_use_ref`, `test_counter_app_reactive_execution`, `test_todo_app_add_toggle_and_keyed_delete`, `test_keyed_children_reconciliation`, `test_complex_element_tree_mount_and_reconciliation`
  - `ferrix-android`: `test_runtime_initial_batch`, `test_runtime_event_handling`

### 7.2 NDK Cross-Compilation
- Compiled `arm64-v8a` and `x86_64` targets via `cargo ndk`:
  - `android/ferrix-runtime/src/main/jniLibs/arm64-v8a/libferrix_android.so`
  - `android/ferrix-runtime/src/main/jniLibs/x86_64/libferrix_android.so`

### 7.3 Android Test Suite
- `./gradlew test`: All Robolectric and unit tests passed cleanly (`FerrixE2ETest`, `CommandDecoderTest`).
- Verified Todo app full lifecycle:
  - Initial 2 items mounted.
  - "+ Add Task" button click appends item 3.
  - "Toggle" button click updates item 2 completion state and text in place.
  - "Delete" button click removes item 1 via keyed diffing, leaving remaining items intact.

### 7.4 Live Emulator Gate Verification
- Device: `emulator-5554` (Android 34 `x86_64`).
- App installed: `adb install -r android/sample-app/build/outputs/apk/debug/sample-app-debug.apk`.
- App launched: `adb shell am start -n com.ferrix.sample/.MainActivity`.
- Node hierarchy dump (`/data/local/tmp/dump_m3_initial.xml`):
  - Root `LinearLayout` (Column)
  - `TextView` ("Ferrix Todo App (M3)")
  - `Button` ("+ ADD TASK")
  - Row 1: `[x] Explore Ferrix Architecture`, `TOGGLE`
  - Row 2: `[ ] Build Reactive Runtime`, `TOGGLE`
- Interactive testing:
  - Tapped `+ ADD TASK`: item 3 `[ ] New Ferrix Task #3` appended dynamically with `TOGGLE` and `DELETE` buttons.
  - Tapped `TOGGLE` on Item 2: text mutated to `[x] Build Reactive Runtime`.
  - Tapped `DELETE` on Item 3: item was removed from the native View hierarchy without recreating or disturbing existing items.
- Visual Screenshot captured: `docs/m3_screen.png`.
- **Milestone 3 Gate Passed: 100% verified.**

