# Ferrix Testing Framework Specification (`ferrix-test`)

## 1. Overview & Architectural Role
Under **Architecture A**, Ferrix UI components, fine-grained state graphs, reconciliation, and layout logic execute entirely in Rust. The Kotlin layer exists exclusively for rendering native Android Views and platform services.

Consequently, component testing in Ferrix does **not** require an Android device, an emulator, or the Android JVM runtime. `ferrix-test` provides a 100% host-native headless test renderer with queries, synthetic event dispatch, patch assertions, and simulated timers.

```
       Host Test (Rust `cargo test`)
                    │
            render(Component)
                    ▼
          ┌───────────────────┐
          │   ferrix-test     │ ◄─── Queries: get_by_text, get_by_role, get_by_test_id
          │ (Headless Harness)│ ◄─── Events: fire_event::press, fire_event::change_text
          └─────────┬─────────┘ ◄─── Timers: advance_timers_by_time, run_all_timers
                    │
                    ▼
          ┌───────────────────┐
          │    Patch Sink     │ ───► Assert exact patches (e.g. 1 SetText per click)
          └───────────────────┘
```

---

## 2. Headless Test Renderer

### `render<F>(f: F) -> TestRenderer`
Instantiates a headless component environment, mounts the initial tree, records initial `PatchBatch` operations, and prepares query targets.

```rust
use ferrix_test::prelude::*;

let mut renderer = render(|cx| {
    let count = cx.use_state(0);
    Column::new()
        .child(Text::new(format!("Count: {}", count.get())))
        .child(
            Button::new("Increment")
                .test_id("inc-btn")
                .on_click(move || count.set(count.get() + 1)),
        )
        .build(cx)
});
```

---

## 3. Query APIs
Queries adhere to standard Testing Library semantics:
- `get_by_*`: Returns matching `TestNode` or panics/errors if not found or if ambiguous.
- `query_by_*`: Returns `Option<TestNode>`, returning `None` if not found.
- `find_all_by_*`: Returns `Vec<TestNode>` matching criteria.

Supported queries:
| Query | Description |
|---|---|
| `get_by_text(text: &str)` | Matches node text content (exact or substring). |
| `get_by_test_id(test_id: &str)` | Matches `Property::TestId(val)`. |
| `get_by_role(role: &str)` | Matches `Property::AccessibilityRole(val)`. |
| `get_by_label(label: &str)` | Matches `Property::AccessibilityLabel(val)` or `ContentDescription`. |

---

## 4. Synthetic Event Simulation (`fire_event`)
Events simulate native Android interactions dispatched over JNI without running native code:

```rust
// Simulate button press
renderer.fire_event(node_id, EventType::Click);
// or via TestNode convenience
renderer.get_by_test_id("inc-btn").unwrap().press();
```

When an event fires:
1. Associated callback/handler executes.
2. If reactive signals update, the component re-renders.
3. Minimal `PatchBatch` is computed via `diff_elements(old_tree, new_tree)`.
4. Emitted patches are recorded in `renderer.recent_patches()`.

---

## 5. Mock Timers (`FakeTimers`)
Enables deterministic time travel without sleeping real threads:
```rust
let timers = FakeTimers::new();
timers.set_timeout(1000, || { ... });

timers.advance_timers_by_time(500); // Callback not yet executed
timers.advance_timers_by_time(500); // Callback executed
```

---

## 6. Snapshot & Patch Assertions
Enables visual inspection of the declarative tree and verification of the batched JNI contract:
```rust
// Assert exact tree snapshot
let snapshot = renderer.to_snapshot();
assert!(snapshot.contains("<Text>Count: 1</Text>"));

// Assert zero redundant patches (1 state change -> 1 patch)
assert_eq!(renderer.recent_patches().len(), 1);
```
