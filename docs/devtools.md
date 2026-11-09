# Ferrix DevTools, Error Handling & Background Services Specification

## 1. Architectural Invariants
Under **Architecture A**:
- Rust owns application logic, reactive graph updates, error boundaries, performance tracking metrics, and background task definitions.
- Kotlin owns native Android frame cadence, platform WorkManager scheduling, and native View rendering.
- Rust panic boundaries (`std::panic::catch_unwind`) prevent unhandled component crashes from taking down the JNI boundary or the Android application process.

---

## 2. Error Boundary Architecture (`ferrix-core`)

### `render_with_error_boundary`
Wraps a component subtree with an unwind-safe barrier. If child rendering or state evaluation panics or emits a critical failure:
1. Panic is caught safely.
2. An `ErrorInfo` structure is constructed with the failure message.
3. The fallback UI builder is called to construct a user-friendly error view (e.g., "Something went wrong" with a retry button).

```rust
use ferrix_core::error_boundary::{render_with_error_boundary, ErrorInfo};

let tree = render_with_error_boundary(
    cx,
    |cx| buggy_component(cx),
    |err: &ErrorInfo| {
        Column::new()
            .child(Text::new("An unexpected error occurred"))
            .child(Text::new(format!("Details: {}", err.message)))
    },
);
```

---

## 3. Route `+error.fsx` Escalation (`ferrix-router`)
File-based routes define error handlers using the `+error.fsx` naming convention:
- When an unhandled error occurs during route execution or data loading, `router.trigger_error(err)` is called.
- The router manifest searches upward from the active route to find the nearest ancestor `+error` screen.
- If found, it renders the ancestor `+error` screen with error context; otherwise, it falls back to a global `+error` or synthetic fallback screen.

---

## 4. In-App DevTools & Overlay (`ferrix-devtools`)

### 1. Frame Rate & Performance Monitor (`FpsMonitor`)
- Records frame durations (in milliseconds) and computes a rolling 30-frame average FPS.
- Identifies dropped frames (frames exceeding 16.67ms for 60Hz or 8.33ms for 120Hz).

### 2. Patch Buffer Inspector (`PatchInspector`)
- Inspects `PatchBatch` dispatches over JNI: count of patches, patch types, and serialized binary byte size.
- Detects redundant renders or oversized patch payloads.

### 3. Reactive Signal Inspector (`SignalInspector`)
- Maintains an in-memory registry of active reactive signals and their stringified values for real-time debugging.

### 4. Visual DevTools Overlay (`DevToolsOverlay`)
- Declarative floating component rendered at the top of the screen when DevTools are toggled.
- Displays live FPS, memory/patch counters, and quick toggles.

---

## 5. Background WorkManager Services (`PlatformRequest`)
Background jobs (periodic sync, data refresh, cleanup) are scheduled via platform requests:
- `PlatformRequest::ScheduleBackgroundTask { task_id, delay_ms, is_periodic, interval_ms }`
- Android WorkManager schedules the job without keeping the activity or main thread alive.
- When executed, WorkManager notifies the Rust runtime via `PlatformResponse::BackgroundTaskCompleted`.
