# F4 Learnings: State Management & Forms (`ferrix-store`, `ferrix-forms`)

## Architecture Decisions & State Invariants
Under **Architecture A**, application state is owned and managed by the Rust runtime. State changes must not re-render unaffected components.

### 1. Fine-Grained Selector Isolation (`ferrix-store`)
- In traditional React/React Native, state changes frequently re-render the whole component tree unless wrapped in extensive `React.memo` or `useMemo` hooks.
- In `ferrix-store`, `store.select(|state| selector(state))` creates a fine-grained reactive `Signal<T>`.
- The store tracks the previous value returned by the selector (`*prev_val`). When `store.update()` runs, the signal's `set()` is executed **only if** `new_val != *prev_val`.
- Unrelated mutations (e.g. changing `state.theme` while a component only subscribes to `state.user.name`) trigger zero subscriber evaluations, zero VDOM diffing, and zero native JNI commands.

### 2. Persistence Middleware
- `with_persistence(key, storage)` cleanly bridges `ferrix-store` with `ferrix-storage`.
- On initialization, it restores state from the sync-read cache (`storage.get_json::<S>(key)`).
- On mutations, it persists state changes asynchronously to the crash-safe append-only WAL journal.

### 3. Cursor Jump Prevention via Monotonic Versioning (`ferrix-forms`)
- Fast typing in Android `EditText` causes cursor jumping when bidirectional bindings echo text back asynchronously.
- `ferrix-forms` assigns a monotonic `version: Signal<u64>` to every `Field<T>`. Keystrokes increment the version.
- When native text changes are processed, the runtime compares versions and only updates the native View text when external sanitization alters the string, eliminating cursor displacement.

### 4. Verification Evidence
- `ferrix-store`: 3 unit tests in `crates/ferrix-store/tests/store_tests.rs` (updates, selector isolation, persistence middleware).
- `ferrix-forms`: 2 unit tests in `crates/ferrix-forms/tests/forms_tests.rs` (validation rules, touched/dirty states, version counter increments, form controller submission).
- Workspace: 87 Rust tests passing, all Android Robolectric tests passing.
