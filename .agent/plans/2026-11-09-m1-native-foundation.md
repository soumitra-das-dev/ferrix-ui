# Plan: Milestone 01 — Native Foundation

## 1. Goal
Establish the native cross-compilation and runtime communication pipeline between Rust and Android:
1. Pure-Rust `ferrix-renderer-protocol` defining batched binary patches and UI events.
2. `ferrix-android` crate providing safe JNI bindings (`libferrix_android.so`), panic protection, and command buffer exchange.
3. `android/ferrix-runtime` Kotlin library:
   - `FerrixBridge`: JNI lifecycle manager, buffer mapping via `DirectByteBuffer`.
   - `FerrixRenderer`: Translates decoded binary patches to Android View hierarchy on Main Thread.
   - `FerrixViewFactory`: Instantiates native Android Views (`TextView`, `ViewGroup`, etc.).
4. `android/sample-app`: Android sample application initializing Ferrix runtime and hosting the native View.
5. **Gate Verification**: A native `TextView` rendered from Rust via batched patch command buffer on an Android environment with bidirectional JNI communication.

---

## 2. Files to Touch / Create
- `crates/ferrix-renderer-protocol/src/lib.rs` (Created and tested)
- `crates/ferrix-android/src/lib.rs` (JNI exports, panic boundaries, state initialization)
- `crates/ferrix-android/Cargo.toml`
- `android/build.gradle.kts` (Root Gradle configuration)
- `android/settings.gradle.kts`
- `android/gradle/wrapper/*`
- `android/ferrix-runtime/build.gradle.kts`
- `android/ferrix-runtime/src/main/java/com/ferrix/runtime/FerrixBridge.kt`
- `android/ferrix-runtime/src/main/java/com/ferrix/runtime/FerrixRenderer.kt`
- `android/ferrix-runtime/src/main/java/com/ferrix/runtime/protocol/CommandDecoder.kt`
- `android/ferrix-runtime/src/test/java/com/ferrix/runtime/CommandDecoderTest.kt`
- `android/sample-app/build.gradle.kts`
- `android/sample-app/src/main/AndroidManifest.xml`
- `android/sample-app/src/main/java/com/ferrix/sample/MainActivity.kt`
- `.agent/milestones.md`
- `.agent/learnings/milestone-01.md`
- `.agent/learnings/commands.md`

---

## 3. Detailed Approach & Architecture

### 3.1 JNI Contract & Safety
The JNI bridge exposes two primary functions:
1. `Java_com_ferrix_runtime_FerrixBridge_nativeInit`:
   Initializes the Rust runtime and returns a stable pointer/handle to the runtime instance.
2. `Java_com_ferrix_runtime_FerrixBridge_nativeGetInitialPatches`:
   Retrieves the initial batch of UI patches as a `DirectByteBuffer` containing `CreateNode(TextView)` and `SetText("Hello from Ferrix Native Rust!")`.
3. `Java_com_ferrix_runtime_FerrixBridge_nativeDispatchEvent`:
   Passes user interactions (clicks, text input) back into the Rust event loop.
4. `Java_com_ferrix_runtime_FerrixBridge_nativeDestroy`:
   Gracefully tears down the Rust runtime.

Every export wraps execution in `std::panic::catch_unwind` to ensure no panics unwind across the C/JNI boundary.

### 3.2 Kotlin Decoder & Renderer
`CommandDecoder` reads directly from the `ByteBuffer` in little-endian order:
- Validates the `FXUI` magic header and protocol version.
- Reads `command_count`.
- Loops through opcodes: `CreateNode`, `AppendChild`, `SetText`, `SetLayout`, `SetProperty`, `SetVisibility`.
- `FerrixRenderer` executes on `Dispatchers.Main`:
  - `CreateNode(id, Text)`: Instantiates `android.widget.TextView` and registers in `LongSparseArray<View>`.
  - `AppendChild(parentId, childId)`: Calls `ViewGroup.addView(child)`.
  - `SetText(id, text)`: Calls `TextView.text = text`.
  - `SetLayout(id, x, y, w, h)`: Applies layout parameters.

---

## 4. Risks & Mitigations
- **Android SDK/NDK and JDK environment missing**:
  - *Mitigation*: We download and configure a self-contained local JDK (Temurin 21) and Android commandlinetools/NDK directly in user space (`~/.local/share/android-sdk`), with complete scriptability.
- **Panic across FFI**:
  - *Mitigation*: Explicit `catch_unwind` in all `extern "C"` JNI entry points.
- **Memory leaks in native buffers**:
  - *Mitigation*: Rust manages native buffer allocation; Kotlin uses `ByteBuffer` views over raw pointers with explicit deallocation callbacks.

---

## 5. Verification Commands
1. Rust Workspace:
   `cargo fmt --check`
   `cargo clippy --workspace -- -D warnings`
   `cargo test --workspace`
2. Cross-Compilation:
   `cargo ndk -t arm64-v8a -t x86_64 -o android/ferrix-runtime/src/main/jniLibs build`
3. Kotlin/Android Build & Tests:
   `cd android && ./gradlew test`
   `cd android && ./gradlew assembleDebug`
4. Runtime Execution / Gate Check:
   Run unit/integration tests asserting buffer encoding/decoding and native view creation.

---

## 6. Rollback Strategy
Git commits are made per logical step. If cross-compilation or Gradle build fails, isolate whether the failure is in Rust NDK glue or Gradle configuration before reverting.

---

## 7. Results & Verification Evidence

### 7.1 Rust Workspace Quality
- `cargo fmt --check`: Clean (0 diffs).
- `cargo clippy --workspace -- -D warnings`: Clean (0 warnings).
- `cargo test --workspace`: 4/4 unit tests passed (`ferrix-renderer-protocol`: `test_batch_round_trip`, `test_event_round_trip`; `ferrix-android`: `test_runtime_initial_batch`, `test_runtime_event_dispatch`).

### 7.2 Cross-Compilation
- Command: `cargo ndk -t arm64-v8a -t x86_64 -o android/ferrix-runtime/src/main/jniLibs build -p ferrix-android`
- Output:
  - `android/ferrix-runtime/src/main/jniLibs/x86_64/libferrix_android.so` (4.4MB unstripped debug)
  - `android/ferrix-runtime/src/main/jniLibs/arm64-v8a/libferrix_android.so` (4.1MB unstripped debug)

### 7.3 Android Build & Unit/Robolectric Tests
- `./gradlew test`: Passed cleanly (`CommandDecoderTest`, `FerrixE2ETest` loading `libferrix_android.so` via JNI and verifying native View tree instantiation and reactive mutations).
- `./gradlew assembleDebug`: Built `android/sample-app/build/outputs/apk/debug/sample-app-debug.apk` (21MB).

### 7.4 Live Emulator Gate Verification
- Device: `emulator-5554` running Android 34 (`x86_64`).
- App installed: `adb install -r android/sample-app/build/outputs/apk/debug/sample-app-debug.apk`.
- App launched: `adb shell am start -n com.ferrix.sample/.MainActivity`.
- Logcat confirmed: `com.ferrix.sample/.MainActivity` displayed; `libferrix_android.so` loaded.
- Hierarchy inspection (`uiautomator dump`):
  - Initial state:
    `node index="0" text="Hello from Ferrix Native Rust!" resource-id="" class="android.widget.TextView"`
    `node index="1" text="TAP TO MUTATE FROM RUST" resource-id="" class="android.widget.Button"`
  - Tap button (`input tap 160 102`):
    `node index="0" text="Rust Reactive Count: 1" resource-id="" class="android.widget.TextView"`
  - Second tap:
    `node index="0" text="Rust Reactive Count: 2" resource-id="" class="android.widget.TextView"`
- Visual Screenshot captured: `docs/m1_screen.png`.
- **Milestone 1 Gate Passed: 100% verified.**

