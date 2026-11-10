# Crash Reproduction and Diagnosis Report

## Device Context
- **Physical Device**: Motorola Moto G34 5G
- **OS Version**: Android 15 (API 35)
- **Reported ABI List (`ro.product.cpu.abilist`)**: `arm64-v8a,armeabi-v7a,armeabi`
- **Application ID**: `com.example.my_app`
- **Activity**: `com.example.my_app/.MainActivity`

## Crash Log Evidence
```text
11-10 17:00:48.217 31620 31620 D nativeloader: Load libmy-app.so using class loader ns clns-7 (caller=/data/app/~~xSJUxv-nE1_Hjv2m5xlkzg==/com.example.my_app-ccYGDCgJbC18zLY-pkY9AA==/base.apk!classes3.dex): dlopen failed: library "libmy-app.so" not found
11-10 17:00:48.217 31620 31620 D AndroidRuntime: Shutting down VM
11-10 17:00:48.219 31620 31620 E AndroidRuntime: FATAL EXCEPTION: main
11-10 17:00:48.219 31620 31620 E AndroidRuntime: Process: com.example.my_app, PID: 31620
11-10 17:00:48.219 31620 31620 E AndroidRuntime: java.lang.UnsatisfiedLinkError: dlopen failed: library "libmy-app.so" not found
11-10 17:00:48.219 31620 31620 E AndroidRuntime: 	at java.lang.Runtime.loadLibrary0(Runtime.java:1112)
11-10 17:00:48.219 31620 31620 E AndroidRuntime: 	at java.lang.Runtime.loadLibrary0(Runtime.java:1035)
11-10 17:00:48.219 31620 31620 E AndroidRuntime: 	at java.lang.System.loadLibrary(System.java:1764)
11-10 17:00:48.219 31620 31620 E AndroidRuntime: 	at com.example.my_app.MainActivity.<clinit>(MainActivity.kt:10)
```

## APK Inspection Evidence
```shell
$ unzip -l "$(find . -name app-debug.apk | head -1)" | grep 'lib/'
# Output: EMPTY (0 shared object files packaged in the APK)
```

## Diagnosis and Ruled Out Hypotheses
1. **Ruled Out: 16 KB page-size alignment**:
   - The device is a standard 4 KB ARM64 device; the crash occurred during dynamic linker loading (`dlopen`) before any native initialization code was executed.
2. **Ruled Out: Rust init panic crossing JNI (`SIGABRT`)**:
   - No native crash / SIGABRT tombstone was generated; JVM failed in static `<clinit>` due to missing shared library.
3. **Ruled Out: Theme / AppCompat mismatch**:
   - Theme is `Theme.Material.Light.NoActionBar` with standard `android.app.Activity`. The Activity failed to instantiate because `<clinit>` threw `UnsatisfiedLinkError`.
4. **CONFIRMED Root Causes**:
   - **Root Cause A (Wrong `System.loadLibrary` name)**:
     The scaffolded `MainActivity.kt` invoked `System.loadLibrary("{slug}")` (which evaluated to `"my-app"`).
     In the Rust compiler and Cargo toolchain, crate names containing hyphens (`my-app`) produce `.so` artifacts with underscores (`libmy_app.so`). Android's `System.loadLibrary("my-app")` strictly looks for `libmy-app.so` and fails.
   - **Root Cause B (Missing `lib/` in APK due to ABI misdetection / fallback)**:
     `ferrix dev` (and `ferrix run`) did not query device ABI properly or fell back silently on `cargo ndk` failure without failing the command. In standard `ferrix dev`, host `cargo build` was run, resulting in zero `.so` files in `android/app/src/main/jniLibs/<abi>`, so the packaged APK contained no `lib/` directory at all.
   - **Root Cause C (False positive launch status)**:
     `ferrix dev` and `ferrix run` relied solely on `adb shell am start` returning exit code 0 to declare:
     `Application launched successfully on target device!`
     `am start` merely schedules Activity launch in the Android system server; it returns 0 even when the process crashes 50ms later in `<clinit>`. No logcat monitoring, PID check, or handshake confirmation was performed in standard `ferrix dev`.
