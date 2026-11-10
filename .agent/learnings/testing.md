# F7 Learnings: Headless Testing Framework (`ferrix-test`)

## Architecture Decisions & Invariants
Under **Architecture A**, UI logic, state updates, reconciliation, and layout are computed in Rust. Consequently, component tests can execute completely headless on the host system without Android emulators, JVM runtimes, or physical devices.

### 1. Headless Virtual Tree & Queries (`TestRenderer`)
- `render<F>(f: F) -> TestRenderer` mounts a component into an in-memory virtual tree.
- Queries follow standard Testing Library patterns:
  - `get_by_text`, `query_by_text`, `find_all_by_text`: search text content across nodes.
  - `get_by_test_id`, `query_by_test_id`: match `Property::TestId`.
  - `get_by_role`, `find_all_by_role`: match `Property::AccessibilityRole`.
  - `get_by_label`: match `Property::AccessibilityLabel` or `ContentDescription`.
- Queries return `TestNode` or clear `QueryError` variants (`TextNotFound`, `MultipleWithText`, `TestIdNotFound`).

### 2. Synthetic Event Simulation & Patch Contracts (`fire_event`)
- `fire_event::press(&mut renderer, node_id)`
- `fire_event::change_text(&mut renderer, node_id, text)`
- `fire_event::toggle(&mut renderer, node_id, value)`
- Simulating events invokes registered event handlers, re-renders the component, runs `diff_elements()`, and records emitted patches.
- Guarantees the fine-grained reactivity contract: changing a single counter value produces exactly one `SetText` patch (`renderer.assert_single_patch()`).

### 3. Deterministic Mock Timers (`FakeTimers`)
- Min-heap based timer queue handling `set_timeout` and `set_interval`.
- `advance_timers_by_time(ms)` simulates clock ticks deterministically without sleeping OS threads.
- Enables testing debounced inputs, delayed transitions, and time-based workflows with zero flaky wall-clock dependencies.

### 4. Verification Evidence
- `ferrix-test`: 6 tests passing in `crates/ferrix-test/tests/test_framework_tests.rs`.
- Total workspace: 114 tests passing across 21 crates, 0 clippy warnings.
