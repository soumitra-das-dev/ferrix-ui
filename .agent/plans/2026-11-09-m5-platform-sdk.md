# Plan: Milestone 05 — Platform SDK

## 1. Goal
Implement the Ferrix Platform SDK allowing reactive components to access native Android platform capabilities:
1. **Runtime Permissions** (`use_permission`, `PermissionStatus`, request & denial rationale handling).
2. **Fused Location** (`use_location`, continuous updates, accuracy, lifecycle cancellation).
3. **Camera** (`use_camera`, preview & photo capture via CameraX adapter, lifecycle binding).
4. **Storage & Sensors** (Key-value storage adapter and accelerometer/sensor adapter).

**Milestone 5 Gate**:
A Camera + Location demo app running end-to-end:
- Handles permission request and denial gracefully.
- Displays location updates (lat, lng, accuracy).
- Binds camera lifecycle.
- Cancels location & camera subscriptions on component unmount / teardown without leaking instances.

---

## 2. Architecture & Contract

### 2.1 Platform Message Protocol (`ferrix-renderer-protocol` or platform module)
Platform communication uses typed binary or structured command packets crossing the JNI boundary:
- **Rust to Kotlin (Platform Requests)**:
  - `PlatformRequest::RequestPermission { permission: String, request_id: u32 }`
  - `PlatformRequest::StartLocationUpdates { min_interval_ms: u64, min_distance_m: f32 }`
  - `PlatformRequest::StopLocationUpdates`
  - `PlatformRequest::StartCameraPreview { target_node_id: NodeId }`
  - `PlatformRequest::StopCamera`
  - `PlatformRequest::TakePicture`
  - `PlatformRequest::ReadStorage { key: String }`
  - `PlatformRequest::WriteStorage { key: String, value: String }`
- **Kotlin to Rust (Platform Responses & Events)**:
  - `PlatformEvent::PermissionResult { request_id: u32, granted: bool, should_show_rationale: bool }`
  - `PlatformEvent::LocationUpdate { latitude: f64, longitude: f64, accuracy: f32, timestamp: u64 }`
  - `PlatformEvent::PictureCaptured { path: String, width: u32, height: u32 }`
  - `PlatformEvent::StorageResult { key: String, value: Option<String> }`

### 2.2 Kotlin Platform Adapters (`android/ferrix-runtime`)
- `PlatformManager`: Central dispatcher for platform requests received from Rust.
- `PermissionAdapter`: Interacts with Android Activity/Context to check and request permissions.
- `LocationAdapter`: Interacts with `LocationManager` / fused location with location listener callbacks.
- `CameraAdapter`: Manages CameraX preview and image capture lifecycle.
- `StorageAdapter`: Manages `SharedPreferences` persistence.

### 2.3 Rust Reactive Hooks (`ferrix-core::platform` or `ferrix-platform`)
- `use_permission(cx, permission: &str) -> (PermissionState, impl Fn())`
- `use_location(cx, config: LocationConfig) -> Option<LocationData>`
- `use_camera(cx) -> CameraState`
- Lifecycles: `useEffect` with cleanup ensures unmounting the component immediately dispatches cancellation (`StopLocationUpdates`, `StopCamera`).

---

## 3. Files to Touch / Create

### 1. Protocol
- `crates/ferrix-renderer-protocol/src/platform.rs` (Platform requests and events encoding/decoding)
- `crates/ferrix-renderer-protocol/src/lib.rs` (Export platform types)

### 2. Core Reactive Hooks
- `crates/ferrix-core/src/platform.rs` (`use_permission`, `use_location`, `use_camera`, `use_storage`)
- `crates/ferrix-core/src/lib.rs` (Export platform hooks)

### 3. JNI Bridge
- `crates/ferrix-android/src/lib.rs` (JNI entry points for platform message routing between Rust and Kotlin)

### 4. Kotlin Runtime Adapters
- `android/ferrix-runtime/src/main/java/com/ferrix/runtime/platform/PlatformProtocol.kt`
- `android/ferrix-runtime/src/main/java/com/ferrix/runtime/platform/PlatformManager.kt`
- `android/ferrix-runtime/src/main/java/com/ferrix/runtime/platform/PermissionAdapter.kt`
- `android/ferrix-runtime/src/main/java/com/ferrix/runtime/platform/LocationAdapter.kt`
- `android/ferrix-runtime/src/main/java/com/ferrix/runtime/platform/CameraAdapter.kt`
- `android/ferrix-runtime/src/main/java/com/ferrix/runtime/platform/StorageAdapter.kt`
- `android/ferrix-runtime/src/main/java/com/ferrix/runtime/FerrixBridge.kt` (Integrate `PlatformManager`)

### 5. Verification & Tests
- `crates/ferrix-renderer-protocol/tests/platform_protocol_tests.rs` (Round-trip encode/decode)
- `crates/ferrix-core/tests/platform_hook_tests.rs` (Lifecycle and unmount cleanup)
- `android/ferrix-runtime/src/test/java/com/ferrix/runtime/PlatformSDKTest.kt` (Robolectric verification of camera + location demo app)
- `examples/platform_demo/` (Demo app running camera & location)
- Live emulator execution & screenshot `docs/m5_screen.png`.

---

## 4. Verification Commands
- `cargo fmt --check`
- `cargo clippy --workspace -- -D warnings`
- `cargo test --workspace`
- `cargo ndk -t arm64-v8a -t x86_64 -o android/ferrix-runtime/src/main/jniLibs build --release`
- `cd android && ./gradlew assembleDebug`
- `cd android && ./gradlew test`

---

## 5. Execution Results & Gate Verification
- **Platform Protocol**: Implemented typed binary encoding for `PlatformRequest` and `PlatformResponse` with `FXPL` magic in `crates/ferrix-renderer-protocol/src/platform.rs`. Round-trip tests verified.
- **Core Platform Hooks**: Implemented `use_permission`, `use_location`, `use_camera` in `crates/ferrix-core/src/platform.rs`. Verified mount registration and `unmount` teardown cancellation.
- **Android Adapters**: Implemented `PlatformManager`, `PermissionAdapter`, `LocationAdapter`, `CameraAdapter`, `StorageAdapter` in `android/ferrix-runtime`.
- **Gate Test Passed (`PlatformSDKTest.testPlatformSDKCameraAndLocationDemoAppLifecycle`)**:
  - Permissions requested and granted.
  - Location tracking started, simulated updates processed and displayed.
  - Camera preview started and bound.
  - Unmount / teardown verified: both location tracking and camera preview immediately stopped and cancelled without leaks.
- **Live Device Verification**:
  - Installed debug APK to emulator `emulator-5554`.
  - Verified UI rendering and interaction.
  - Screenshot captured to `docs/m5_screen.png`.
- **Status**: 🟢 **PASSED**


