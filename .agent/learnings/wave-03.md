# Wave 3 Learnings: Platform APIs & Hooks

## Architecture A Platform Bridging Pattern
Under Architecture A, Rust owns reactive state, component execution, and layout, while Kotlin owns native Android platform APIs, system services, and ephemeral interaction metrics.

### 1. Inbound Asynchronous System Telemetry vs Outbound Imperative Requests
- **Outbound Requests**: Operations initiated by Rust (`PlatformRequest`: `Vibrate`, `SetClipboard`, `Share`, `OpenUrl`, `ShowAlert`, `ExitApp`, `DismissKeyboard`, `AnnounceAccessibility`) are queued into a thread-safe `PlatformOutbox` channel without blocking the Rust runtime thread. When the runtime ticks or flushes a batch, outbound buffers are passed across JNI to `PlatformManager`.
- **Inbound Telemetry**: Hardware events, window insets, system back button presses, app lifecycle transitions, and connectivity callbacks are pushed from Android to Rust via `PlatformResponse` buffers (`DimensionsChange`, `KeyboardChange`, `AppStateChange`, `NetInfoUpdate`, `BackPressed`).
- `route_platform_response` safely routes incoming binary telemetry to atomic global states or registered callbacks, triggering reactivity only when values actually change.

### 2. Clipboard String Marshalling
- Platform clipboard operations can occur imperatively or reactively. To prevent synchronous JNI stalls while waiting for Android's `ClipboardManager`, `Clipboard::set_string` is non-blocking fire-and-forget, while `Clipboard::get_string` carries a monotonic `request_id` that resolves asynchronously via `PlatformResponse::ClipboardContent`.

### 3. BackHandler Hardware Arbitration
- Android's hardware/gesture back navigation requires cooperative cancellation. In Kotlin, `onBackPressed` triggers `PlatformManager.sendBackPressed()`.
- On the Rust side, `BackHandler` invokes registered component handlers in reverse mount order (LIFO). If any handler returns `true`, the event is consumed. If unhandled, it falls through to default platform activity navigation or `exit_app`.

### 4. Zero Reflection and Type Safety in Protocol Payloads
- Unlike traditional React Native bridges which marshal platform requests via dynamic JSON or dictionary objects, Ferrix encodes all 17 platform requests and 12 platform responses into compact binary byte buffers with 2-byte tags and 4-byte LE length prefixes.
- Zero-copy string slicing and UTF-8 validation ensure zero crash vulnerabilities or reflection overhead at the native boundary.
