# Ferrix Interactive Dev Server (`ferrix dev`)

`ferrix dev` is the development control center for Ferrix applications. Similar to Expo CLI and Vite, it runs an interactive development server with real-time file watching, tiered hot reloading, on-device live log streaming, crash classification, and terminal shortcuts.

---

## Quickstart

From any Ferrix project root:

```bash
# Start the interactive dev server on your connected Android device or emulator
ferrix dev

# List connected Android devices and emulators
ferrix dev --list-devices

# Target a specific device or emulator
ferrix dev --device emulator-5554

# Run in non-interactive CI smoke test mode
ferrix dev --smoke
```

---

## Interactive Terminal Commands

While `ferrix dev` is running, the terminal UI accepts single-key commands without pressing Enter:

| Key | Action | Description |
| :--- | :--- | :--- |
| **`a`** | **Run / Open Android** | Re-launches the application activity on the selected Android device. |
| **`d`** | **Select Device** | Opens the interactive device picker to switch between connected hardware and emulators. |
| **`r`** | **Fast Reload** | Dispatches a live hot update to the running runtime (preserving reactive state). |
| **`R`** | **Full Rebuild** | Triggers an incremental native compilation, rebuilds the APK, and reinstalls. |
| **`m`** | **Dev Menu** | Triggers the on-device developer settings menu (simulates `KEYCODE_MENU`). |
| **`j`** | **Inspector** | Opens the Ferrix DevTools signal and patch inspector. |
| **`l`** | **Toggle Log Filter** | Cycles logcat filtering: **App Only** → **Errors Only** → **All Logs**. |
| **`c`** | **Clear Console** | Clears the terminal screen and repaints the startup banner and status footer. |
| **`o`** | **Open in Editor** | Opens the current project in `$EDITOR` (defaults to VS Code `code .`). |
| **`p`** | **Toggle Perf Overlay**| Toggles the on-device FPS and memory overlay. |
| **`?`** | **Help** | Displays the full key command reference. |
| **`q`** / **Ctrl+C** | **Quit** | Shuts down watchers, cleans up ADB reverse port forwards, and exits cleanly. |

---

## CLI Flags

| Flag | Type | Description |
| :--- | :--- | :--- |
| `--device <serial>` | String | Directs execution to a specific device serial or model name. |
| `--list-devices` | Flag | Lists all connected devices with their ABI and state, then exits. |
| `--port <n>` | Integer | Sets the Dev-Channel WebSocket/HTTP port (default: `8081`). |
| `--clear` | Flag | Clears `.gradle`, `android/app/build`, and local build caches before starting. |
| `--no-open` | Flag | Starts the dev server without automatically launching the app on device. |
| `--smoke` | Flag | Executes non-interactive automated verification (build, launch, readiness, test interaction, hot reload check) and exits. |
| `--timeout <sec>` | Integer | Readiness timeout for `--smoke` mode (default: `300`). |

---

## Dev Channel & Port Forwarding

`ferrix dev` automatically establishes ADB reverse port forwarding:
```bash
adb reverse tcp:8081 tcp:8081
```
This allows the Android application on either USB or Wi-Fi debugging to communicate with the dev server on loopback (`http://127.0.0.1:8081`).
Upon exit, `ferrix dev` automatically removes the port forward rule.

---

## Crash Classification & Diagnostics

When an unhandled exception or crash occurs on device, `ferrix dev` immediately intercepts the logcat event, extracts the symbolized stack trace, and displays a categorized diagnostic hint:

### 1. `UnsatisfiedLinkError` (Native Library Mismatch)
- **Signal**: `dlopen failed: library "lib<name>.so" not found`
- **Diagnosis**: The native Rust `.so` library was missing or compiled for a different CPU ABI.
- **Remediation**: Ferrix dev builds dynamically resolve the target device ABI (`arm64-v8a`, `x86_64`) via `ro.product.cpu.abilist` and package `lib<pkg_name>.so` with underscore-normalized naming.

### 2. Rust Runtime Panic
- **Signal**: `thread '...' panicked at ...`
- **Diagnosis**: A Rust invariant failed approaching or crossing the JNI boundary.
- **Remediation**: Verify all JNI entry points wrap execution in `std::panic::catch_unwind`.

### 3. Android Theme Mismatch
- **Signal**: `IllegalStateException: You need to use a Theme.AppCompat theme`
- **Diagnosis**: Activity inherits from AppCompat without a matching theme.
- **Remediation**: Set `android:theme` to `@android:style/Theme.Material.Light.NoActionBar` in `AndroidManifest.xml`.

