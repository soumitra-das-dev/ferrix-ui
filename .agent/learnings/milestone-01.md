# Milestone 01 Retrospective: Native Foundation

## Objective
Establish the Rust + Android foundation:
- Rust workspace with modular crates.
- Android Gradle project (`ferrix-runtime` and `sample-app`).
- Rust→Android cross-compile (`cargo-ndk`).
- JNI initialization and bidirectional communication.
- Gate: A native TextView rendered from Rust on Android (emulator or verified runtime environment).

## Findings & Execution Log

### 2026-11-09 Gate Verification & Learnings
- **Toolchain setup in user space**: Adoptium Temurin JDK 21 (`~/.local/share/jvm/jdk-21*`) + Android SDK cmdline-tools (`~/.local/share/android-sdk`), NDK 26.3, platform-tools, build-tools 34.0.0, emulator, and system image `android-34;google_apis;x86_64` are fully automated and verified via portable `env.sh`.
- **Zero-overhead wire protocol**: `ferrix-renderer-protocol` has zero external dependencies, serializing patches with a 16-byte header (`FXUI`, version 1, flags, command count, payload size) followed by typed opcodes. Little-endian binary byte alignment ensures instant zero-copy decoding on Android.
- **DirectByteBuffer & Buffer Lifetime**: The Rust runtime stores the current patch buffer in its heap state mutex and passes raw pointers to JNI `new_direct_byte_buffer`. On the Kotlin side, `ByteBuffer.order(ByteOrder.LITTLE_ENDIAN)` maps directly to native memory without copying.
- **JNI Safety**: Every JNI export function (`nativeInit`, `nativeGetInitialPatches`, `nativeDispatchEvent`, `nativeDestroy`) is guarded with `std::panic::catch_unwind`. Panics return null pointers or 0 error codes, protecting Android ART runtime from crashing.
- **Bi-directional event loop**:
  - Android button click listener calls `FerrixBridge.dispatchEvent(UiEvent.Click(nodeId))`.
  - Rust runtime reacts, increments count, generates `SetText` patch, and returns the new buffer pointer.
  - Kotlin `FerrixRenderer` decodes `SetText` and updates the existing `TextView` instance in place on the Main Thread without rebuilding the layout or recreating views.
- **Robolectric & Live Emulator Testing**:
  - Automated tests run via JUnit/Robolectric in `android/ferrix-runtime` (`CommandDecoderTest`, `FerrixE2ETest`), dynamically loading the host `libferrix_android.so` to verify the full JNI bridge in CI without an emulator.
  - Live verification runs on `emulator-5554` (AVD `ferrix_test`), verified by `uiautomator dump` and screenshots (`docs/m1_screen.png`).

## Gate Evaluation: PASSED 🟢
- Native TextView rendered from Rust on an Android emulator: **Verified** (Initial text: `"Hello from Ferrix Native Rust!"`, bounds `[16,48][304,78]`).
- Button tap reactive state update: **Verified** (text mutated to `"Rust Reactive Count: 1"`, then `"Rust Reactive Count: 2"` via JNI `SetText` patch).
- Screenshot: Saved in `docs/m1_screen.png`.

