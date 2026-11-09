# F8 Learnings: Errors, Logging, DevTools & Background Services (`ferrix-core`, `ferrix-router`, `ferrix-devtools`)

## Architecture Decisions & Invariants
Under **Architecture A**, error containment, performance metrics calculation, and background scheduling logic reside in Rust, with Kotlin managing native View rendering and Android platform execution (e.g. WorkManager).

### 1. Error Containment & Unwind Safety (`render_with_error_boundary`)
- `render_with_error_boundary` wraps component render execution in `std::panic::catch_unwind(AssertUnwindSafe(...))`.
- If an unhandled panic or calculation error occurs within a component subtree:
  - The panic payload is safely intercepted.
  - An `ErrorInfo` struct is synthesized (`message`, `component_name`).
  - The fallback UI builder renders a recovery element (e.g. error banner with retry action).
- Invariant preserved: Zero unhandled Rust panics cross the JNI boundary to crash the Android process.

### 2. Route `+error.fsx` Escalation (`ferrix-router`)
- When an error occurs during route execution or data loading, `router.trigger_error(err)` pushes the matching `+error` screen onto the routing stack.
- The route manifest resolves the nearest ancestor `+error` screen in the directory tree or falls back to a global `+error` screen.
- Navigating back via `router.back()` automatically restores the previous healthy screen.

### 3. In-App DevTools & Metric Overlays (`ferrix-devtools`)
- `FpsMonitor`: Tracks rolling average FPS and detects frame drops exceeding 16.67ms (60Hz cadence).
- `PatchInspector`: Inspects binary `PatchBatch` dispatches over JNI, categorizing operations (`SetText`, `SetProperty`, `CreateNode`) and monitoring payload byte estimates.
- `SignalInspector`: Debug registry tracking reactive signals and their live stringified values.
- `DevToolsOverlay`: Produces a declarative, non-intrusive floating HUD element (`Element`) embedded at the root of debug builds.

### 4. Background Services & WorkManager Integration
- Binary protocol tags `ScheduleBackgroundTask` (18) and `CancelBackgroundTask` (19) allow Rust to schedule one-shot or periodic background jobs.
- Kotlin runtime manages native WorkManager job registration and reports completion back to Rust via `BackgroundTaskCompleted` (13).

### 5. Verification Evidence
- `ferrix-core`: 2 error boundary tests passing in `crates/ferrix-core/tests/error_boundary_tests.rs`.
- `ferrix-router`: Route error escalation test passing in `crates/ferrix-router/tests/router_tests.rs`.
- `ferrix-devtools`: 5 unit tests passing in `crates/ferrix-devtools/tests/devtools_tests.rs`.
- Android runtime: `DevToolsAndWorkerTest.kt` passing under Robolectric.
