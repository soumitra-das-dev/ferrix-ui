# Milestone 03 Retrospective: Reactive Runtime

## Objective
Implement fine-grained reactivity in `crates/ferrix-reactivity` (signals, dependency tracking, memos, effects, batching) and component lifecycle + hooks (`useState`, `useSignal`, `useMemo`, `useEffect`, `useRef`, stable-order hook storage, keyed reconciliation) in `crates/ferrix-core`.

**Gate**: Counter + Todo applications running in hand-written Rust (pure reactive runtime, no compiler yet).

---

## Findings & Execution Log

### 2026-11-09 Reactive Runtime & Keyed Reconciliation Architecture
- **Dependency Graph & Batching**: `ferrix-reactivity` manages an observer stack with automatic dependency tracking on `signal.get()` and trigger dispatch on `signal.set()`. Batching via `batch(|| { ... })` queues dirty observers, executing deduplicated notifications only when the outermost batch exits.
- **Instance-Scoped Stable-Order Hook Storage**: `ComponentContext` assigns a deterministic hook slot index `0, 1, 2, ...` across render passes. Before each pass, `prepare_render()` resets the hook cursor so hooks unconditionally access their assigned state.
- **Keyed Reconciliation Invariant**:
  - In dynamic lists, items change positions or are deleted. When matching old and new elements by `key`, the reconciliation engine MUST reuse the existing `old_child.id` on the platform rather than treating them as different nodes due to sequential allocator offsets.
  - Doing so guarantees existing native Views (Rows, Text, Buttons) are preserved in the platform `LongSparseArray<View>` without being destroyed or recreated, yielding minimal patches (`RemoveChild` + `DestroyNode` on deleted items only).
- **Unkeyed Elements in Keyed Containers**: Unkeyed static siblings (e.g. titles, headers, add buttons) inside a container containing keyed items are reconciled by matching their index and view type, preventing static headers from being recreated when list items mutate.
- **Robolectric & Live Device Testing**:
  - JUnit/Robolectric test `FerrixE2ETest` validates the complete Todo application lifecycle (initial mount, add item, in-place toggle, keyed deletion) through JNI.
  - Live Android emulator (`emulator-5554`) verification confirms visual layout, button tap responsiveness, and UI hierarchy mutations, documented in `docs/m3_screen.png`.

## Gate Evaluation: PASSED 🟢
- Counter App: **Verified** (Hand-written Rust test in `crates/ferrix-core/tests/counter_app_test.rs`; exactly 1 `SetText` patch per click).
- Todo App: **Verified** (Hand-written Rust test in `crates/ferrix-core/tests/todo_app_test.rs`; Robolectric test in `FerrixE2ETest.kt`; live emulator run in `docs/m3_screen.png`).
