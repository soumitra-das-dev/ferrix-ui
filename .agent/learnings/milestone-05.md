# Milestone 05 Retrospective: Platform SDK & Lifecycle Cancellation

## Objective
Implement native Android platform capabilities:
1. Runtime permissions (`use_permission`, `PermissionStatus`, request & denial rationale handling).
2. Fused location (`use_location`, continuous updates, accuracy, lifecycle cancellation).
3. Camera (`use_camera`, preview & photo capture, CameraX adapter, lifecycle binding).
4. Storage (`use_storage`, persistent key-value SharedPreferences).

**Milestone 5 Gate**: Camera + Location demo app running end-to-end with lifecycle and cancellation handling.

## What Worked
1. **Platform Message Protocol**:
   Decoupled, binary-encoded request/response packet pairs (`PlatformRequest` / `PlatformResponse`) with `FXPL` magic header avoid ad-hoc JNI calls and maintain typed schemas without JSON or reflection.
2. **Lifecycle-Aware Cleanup**:
   Hooks bind cancellation closures to `ComponentContext` effects. When components unmount (e.g. activity teardown or navigation), cleanup functions immediately trigger cancellation requests (`StopLocationUpdates`, `StopCamera`), guaranteeing zero leaked listeners or background battery drain.
3. **Android Platform Adapters**:
   Kotlin adapters (`PermissionAdapter`, `LocationAdapter`, `CameraAdapter`, `StorageAdapter`) cleanly bridge native Android system services to Rust reactive signals.
4. **Tested Claims**:
   - `crates/ferrix-renderer-protocol/tests/platform_tests.rs`: binary round-trip encode/decode verified.
   - `crates/ferrix-core/tests/platform_hook_tests.rs`: verified permission routing, location updates, and lifecycle teardown cancellation on unmount.
   - `crates/ferrix-android/src/lib.rs`: verified full event-and-response cycle across the JNI bridge.
   - `android/ferrix-runtime/src/test/java/com/ferrix/runtime/PlatformSDKTest.kt`: Robolectric end-to-end verification of camera + location demo app lifecycle.
   - Live Android 34 emulator verification: live screenshot captured to `docs/m5_screen.png`.

## What Didn't Work / Gotchas
- **Effect Cleanup Storage**:
  `use_effect` initially stored cleanup closures purely in `EffectSlot<D>`, which could not be downcast by `ComponentContext::unmount()` without knowing the generic dependency type `D`. Refactoring cleanup tracking into shared cleanup cells stored in `self.cleanups` enabled clean unmount execution.

## Rule Going Forward
- Platform adapters must always provide explicit teardown (`destroy()` / `unmount()`) to prevent hardware resource leaks.
