# Wave 4 Learnings: Expo-Style Native Modules

## Static Module Architecture under Architecture A
Ferrix native modules provide React Native / Expo parity capabilities without a JavaScript engine, runtime reflection, or dynamic symbol introspection.

### 1. Compile-Time Static Registry vs Dynamic Introspection
- Traditional framework native modules (e.g., React Native `@ReactMethod`, Expo auto-linking) rely on JVM reflection or runtime symbol lookups, incurring 40–120 ms of startup initialization time and increasing APK method count.
- In Ferrix, all native modules register statically at compile time using typed `ModuleId` constants (`1..=12`) and static method tables in both Rust (`crates/ferrix-modules`) and Kotlin (`NativeModuleRegistry`).
- Method calls dispatch directly via `when (methodId)` in Kotlin and pattern matching in Rust, resulting in zero reflection overhead and deterministic branch prediction.

### 2. High-Performance Binary Call Framing
- All native module calls are serialized as `ModuleCall` buffers:
  `[call_id: u32 (4B)][module_id: u16 (2B)][method_id: u16 (2B)][payload_len: u32 (4B)][payload: [u8]]`
- Module responses are serialized as typed `ModuleResult` buffers:
  - `Tag 1 (Success)`: `[call_id: u32][payload_len: u32][payload: [u8]]`
  - `Tag 2 (Error)`: `[call_id: u32][code_len: u32][code: [u8]][msg_len: u32][msg: [u8]]`
  - `Tag 3 (Cancelled)`: `[call_id: u32]`
- Ensuring proper consumption of `call_id` in binary decoders prevents offset drift and buffer underflow when parsing variable-length payloads.

### 3. Cooperative Cancellation & Memory Safety
- In Architecture A, unmounting a UI component must not trigger mutations or memory leaks on destroyed instances.
- Every async module invocation receives a `CancellationToken`. When components unmount, the cancellation token is signaled atomically (`Ordering::SeqCst`), allowing background tasks (sensor polling, geofencing, audio streaming, camera captures) to terminate immediately.

### 4. Lifecycle Binding
- Native modules receive lifecycle broadcasts (`onStart`, `onStop`, `onDestroy`) directly from the host Activity or Ferrix bridge, guaranteeing prompt teardown of hardware locks, CameraX bindings, and sensors.

