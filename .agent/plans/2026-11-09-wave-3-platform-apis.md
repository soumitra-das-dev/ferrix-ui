# Wave 3: Platform APIs & Hooks Plan

## Objective
Implement React Native / Expo parity platform APIs and reactive hooks under Architecture A:
- **Dimensions & PixelRatio**: Screen & window dimensions, density scaling, font scaling, dp ↔ px conversion.
- **Platform**: OS detection, platform version, conditional `Platform.select`.
- **Keyboard**: Hardware & IME keyboard metrics, visibility tracking, `use_keyboard` hook, dismiss commands.
- **AppState**: Foreground, background, inactive state tracking and `use_app_state` hook.
- **BackHandler**: Android system back button interception and hardware navigation handler.
- **Linking, Share, Vibration, Clipboard, Alert**: Native intent handling, system share sheet, haptic pulses, system clipboard, native dialogs.
- **AccessibilityInfo & I18nManager**: Screen reader status, TalkBack announcements, RTL directionality.
- **NetInfo**: Network connectivity monitor (`use_net_info`, cellular/wifi/offline).

## Components & Modules to Add / Update
1. **Renderer Protocol (`ferrix-renderer-protocol/src/platform.rs`)**:
   - `PlatformRequest`:
     - `Vibrate { duration_ms: u64 }` (Tag 9)
     - `SetClipboard { text: String }` (Tag 10)
     - `GetClipboard { request_id: u32 }` (Tag 11)
     - `Share { title: String, message: String }` (Tag 12)
     - `OpenUrl { url: String }` (Tag 13)
     - `ShowAlert { title: String, message: String }` (Tag 14)
     - `ExitApp` (Tag 15)
     - `DismissKeyboard` (Tag 16)
     - `AnnounceAccessibility { message: String }` (Tag 17)
   - `PlatformResponse`:
     - `ClipboardContent { request_id: u32, text: String }` (Tag 7)
     - `NetInfoUpdate { is_connected: bool, is_wifi: bool }` (Tag 8)
     - `AppStateChange { state: u8 }` (Tag 9: 0=active, 1=background, 2=inactive)
     - `KeyboardChange { is_visible: bool, height_dp: f32 }` (Tag 10)
     - `DimensionsChange { width: f32, height: f32, scale: f32, font_scale: f32 }` (Tag 11)
     - `BackPressed` (Tag 12)
2. **Android Runtime Adapters (`android/ferrix-runtime`)**:
   - Update `PlatformProtocol.kt` to decode all new requests and encode all new responses.
   - Update `PlatformManager.kt` to handle clipboard, vibration, sharing, browser intents, alert dialogs, accessibility announcements, network callbacks, keyboard insets, and back presses.
3. **Core Rust Platform Module (`crates/ferrix-core/src/platform.rs`)**:
   - Global thread-safe state & reactive hooks:
     - `Dimensions` & `use_window_dimensions`
     - `PixelRatio`
     - `Platform` (`OS`, `version`, `select`, `is_android`, `is_testing`)
     - `Keyboard` & `use_keyboard`
     - `AppState` & `use_app_state`
     - `BackHandler` & `use_back_handler`
     - `Linking`
     - `Share`
     - `Vibration`
     - `Clipboard` & `use_clipboard`
     - `Alert`
     - `AccessibilityInfo`
     - `I18nManager`
     - `NetInfo` & `use_net_info`
4. **Verification**:
   - Rust unit tests in `crates/ferrix-core/tests/platform_wave3_tests.rs`.
   - Android Robolectric tests in `android/ferrix-runtime/src/test/java/com/ferrix/runtime/Wave3PlatformTest.kt`.
   - Example kitchen-sink or platform showcase in `examples/platform-showcase.fsx`.
   - Full workspace verification: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --workspace`, and `./android/gradlew test`.
