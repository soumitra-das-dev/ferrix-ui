# Ferrix Milestones & Status Board

## Gate Legend
- 🔴 Not Started
- 🟡 In Progress
- 🟢 Passed (verified with evidence)

---

### M1: Native Foundation 🟢
- **Scope**: Workspace skeleton, Gradle project, Rust→Android cross-compile (`cargo-ndk`), JNI init, bidirectional calls.
- **Gate**: A native TextView rendered from Rust on an emulator/device with batched command buffer dispatch.
- **Status**: Passed (Verified on Android 34 emulator & Robolectric JNI tests; text rendered & mutated via button click; evidence in `docs/m1_screen.png`).
- **Date Passed**: 2026-11-09

### M2: Element System 🟢
- **Scope**: `Element`, stable node IDs, props/children, patch protocol (`SetText`, `SetProperty`, `InsertNode`, `RemoveNode`, `SetVisibility`), Kotlin view mapping.
- **Gate**: Column/Row/Text/Button rendered from Rust.
- **Status**: Passed (Verified on Android 34 emulator & Robolectric JNI tests; Column, Row, Text, and Button rendered with styling/spacing; interactive Increment/Reset tested; evidence in `docs/m2_screen.png`).
- **Date Passed**: 2026-11-09

### M3: Reactive Runtime 🟢
- **Scope**: Signals, dependency tracking, hooks with instance-scoped stable-order storage, effects with cleanup, keyed reconciliation, lifecycle.
- **Gate**: Counter + Todo apps running in hand-written Rust (no compiler yet).
- **Status**: Passed (Verified Counter with 1-patch mutation in `counter_app_test.rs`; verified Todo app with add, in-place toggle, and keyed deletion under Robolectric and live on Android 34 emulator; screenshot in `docs/m3_screen.png`).
- **Date Passed**: 2026-11-09

### M4: Compiler 🔴
- **Scope**: `.fsx` → Lexer → Parser → AST → Semantic/type check → Rust codegen.
- **Gate**: `examples/counter/component.fsx` runs end-to-end; asserts: exactly 1 JNI call per click, 1 `SetText` patch, existing TextView instance reused.
- **Status**: Not Started

### M5: Platform SDK 🔴
- **Scope**: Permissions, camera (CameraX), location (fused), sensors, mic, storage.
- **Gate**: Camera + location demo app with lifecycle and cancellation handling.
- **Status**: Not Started

### M6: Network + Advanced UI 🔴
- **Scope**: `useQuery`/`useMutation`, WebSocket, lazy lists with recycling, navigation, animations, image loading (Coil).
- **Gate**: API-driven application.
- **Status**: Not Started

### M7: Production 🔴
- **Scope**: Accessibility (TalkBack), text/IME, state restoration, size/startup/memory optimization, Baseline Profiles, benchmarks vs React Native.
- **Gate**: Production benchmarks satisfying cold/warm start, APK size, peak memory, P50/P95/P99 frame time, JNI calls, allocations per update.
- **Status**: Not Started
