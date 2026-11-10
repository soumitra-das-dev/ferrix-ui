# Wave 4: Expo-Style Native Modules Plan

## Objective
Implement modular, lifecycle-aware native modules with compile-time static registration (zero reflection), typed binary request/response protocols, permission gating, and async cancellation tokens:
1. `ferrix-camera`: CameraX preview, capture, barcode scanning, HDR.
2. `ferrix-location`: Fused GPS, geofencing, background updates.
3. `ferrix-sensors`: Accelerometer, Gyroscope, Magnetometer, Barometer, Pedometer.
4. `ferrix-audio`: AudioTrack playback, background stream state.
5. `ferrix-haptics`: VibrationEffect patterns (impact, notification, selection).
6. `ferrix-secure-store`: Encrypted SharedPreferences / Android Keystore.
7. `ferrix-file-system`: Scoped storage, SAF, directory listing, read/write.
8. `ferrix-image-picker` / `ferrix-document-picker`: System picker intents, MIME filtering.
9. `ferrix-media-library`: MediaStore queries for photos/videos.
10. `ferrix-notifications`: Channels, local scheduled notifications, push token registration.
11. `ferrix-local-authentication`: BiometricPrompt (fingerprint/face authentication).

## Architecture
- **Compile-Time Static Registry**: Zero reflection. Each module registers typed IDs (`u16`) with static handler dispatch in both Rust and Kotlin.
- **Async & Cancellation**: Operations accept monotonic `call_id: u32` with an optional `cancellationToken`. Unmounting or explicit cancellation aborts background tasks and prevents mutating destroyed instances.
- **Permission Gating**: Declarative required permissions checked prior to native dispatch; if not granted, automatically requests permission or fails with `PermissionDenied`.

## Slices
1. **Module Protocol & Registry Foundation**:
   - `crates/ferrix-modules`: Module trait, StaticModuleRegistry, binary packet encoder/decoder (`ModuleCall`, `ModuleResponse`), cancellation tokens.
   - Kotlin: `NativeModule.kt`, `NativeModuleRegistry.kt`.
2. **Core Modules**:
   - Camera, Location, Sensors, Audio, Haptics, SecureStore, FileSystem, Pickers, MediaLibrary, Notifications, LocalAuth.
3. **Reactive Hooks**:
   - `use_sensor`, `use_audio`, `use_haptics`, `use_secure_store`, `use_file_system`, `use_biometrics`, `use_notifications`.
4. **Hardware Showcase Example & Tests**:
   - `examples/hardware-showcase.fsx`
   - Rust integration test: `crates/ferrix-modules/tests/module_tests.rs`.
   - Android Robolectric test: `Wave4NativeModulesTest.kt`.
5. **Verification & Milestone Gate**:
   - Pass all tests in Rust and Android; log learnings in `.agent/learnings/wave-04.md`.

