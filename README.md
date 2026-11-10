<div align="center">

# ⚡ Ferrix

**Ahead-of-time compiled native Android UI framework powered by Rust.**

[![Ferrix CI](https://github.com/soumitra-das-dev/ferrix-ui/actions/workflows/ci.yml/badge.svg)](https://github.com/soumitra-das-dev/ferrix-ui/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust 2021](https://img.shields.io/badge/Rust-2021_Edition-orange.svg)](https://www.rust-lang.org)
[![Android SDK 34](https://img.shields.io/badge/Android-SDK_34_|_API_26+-green.svg)](https://developer.android.com)

[Getting Started](#-quick-start) •
[Architecture](#-architecture-a) •
[Component Model](#-ui-file-format-fsx) •
[CLI & Tooling](#-cli-reference) •
[Documentation](docs/)

</div>

---

## 💡 What is Ferrix?

Ferrix is a statically compiled mobile framework engineered for Android under **Architecture A**:
- **Pure Rust Runtime**: Rust owns application state, reactive signal dependency graphs, component execution, routing, storage, scheduling, and Taffy flexbox layout.
- **Native Android Views**: Kotlin owns the native Android Activity lifecycle, native View rendering, frame scheduling (V-Sync), and platform APIs.
- **No JavaScript Engine**: Zero V8, zero Hermes, zero JavaScript interpreters, and zero runtime reflection.
- **Batched Binary IPC**: Rust computes minimal diffs and transmits typed, packed binary patch buffers (`PatchBatch`) over direct JNI memory buffers.
- **Fine-Grained Reactivity**: State updates trigger minimal node patches (e.g. 1 state change = 1 `SetText` patch = 1 JNI call per commit).

---

## 🚀 Quick Start

### 1. Scaffold a New Project
You can create a project in seconds using `npx`:

```bash
npx create-ferrix-app my-app
```

Or install the native CLI binary via Cargo:

```bash
cargo install ferrix-cli
ferrix new my-app --template hello-counter
```

### 2. Verify Your Toolchain
Check that Rust, Android SDK 34, NDK, and Java 17 are ready:

```bash
cd my-app
ferrix doctor --fix
```

### 3. Run the Development Server
Starts file watching on `.fsx` files, fast incremental rebuilds, and device execution:

```bash
ferrix dev
```

### 4. Build Production Release (APK / AAB)
```bash
# Build optimized signed APK
ferrix build --release --apk

# Build Google Play Android App Bundle (AAB)
ferrix build --release --aab
```

---

## 🎨 UI File Format: `*.fsx`

Developer UI code is authored in `.fsx` files—a statically-typed, JSX-like declarative syntax that compiles ahead-of-time directly into native Rust components:

```fsx
import { Column, Text, Button } from "ferrix/ui";

type CounterProps = { initial: i32 };

export function Counter(props: CounterProps): Element {
  const count = useState(props.initial);

  return (
    <Column padding={20} spacing={12}>
      <Text size={24}>Ferrix Counter</Text>
      <Text>Count: {count.value}</Text>
      <Button onClick={() => count.set(count.value + 1)}>
        Increment
      </Button>
    </Column>
  );
}
```

---

## 🏛️ Architecture A

```
┌────────────────────────────────────────────────────────┐
│               Rust Runtime Thread                      │
│                                                        │
│  Declarative .fsx ──► AOT Compilation ──► Rust Code    │
│  State Signals (Signal<T>) ──► Dependency Graph        │
│  Reconciler ──► Taffy Flexbox Layout ──► Minimal Diffs │
└──────────────────────────┬─────────────────────────────┘
                           │ Direct JNI ByteBuffer
                           │ (Batched binary PatchBatch)
                           ▼
┌────────────────────────────────────────────────────────┐
│               Android Main Thread (UI)                 │
│                                                        │
│  FerrixRenderer ──► Native View Pool (Recycled Views)  │
│  Applies Calculated Frames ──► Hardware Acceleration   │
│  Platform Events (Touch, Scroll) ──► Batched to Rust   │
└────────────────────────────────────────────────────────┘
```

---

## 📦 Framework Capabilities

| System | Implementation | Status |
|---|---|:---:|
| **Router** | File-based routing (`app/`), dynamic routes `[id]`, catch-all `[...rest]`, route groups `(group)`, `+error`, deep links | ✅ Stable |
| **Navigation** | Native stack & tab navigator containers, smooth transitions, screen options | ✅ Stable |
| **Storage** | Sync-read cache + crash-safe append-only WAL journal (CRC32 checksums, compaction) | ✅ Stable |
| **Database** | Embedded SQLite with WAL mode, schema migrations, and live reactive queries | ✅ Stable |
| **State & Forms** | Global store with selector slices, controlled inputs, and monotonic cursor synchronization | ✅ Stable |
| **Localization** | ICU MessageFormat parser (plurals, select, interpolation) with fine-grained locale switching | ✅ Stable |
| **Testing** | 100% headless host component test renderer (`ferrix-test`) with synthetic events & fake timers | ✅ Stable |
| **Tooling** | Lossless idempotent formatter (`ferrix-fmt`), static linter (`ferrix-lint`), and LSP server | ✅ Stable |
| **DevTools** | In-app HUD overlay, rolling FPS monitor, patch buffer inspector, signal debugger | ✅ Stable |

---

## 🛠️ CLI Reference

| Command | Action |
|---|---|
| `ferrix new <name>` | Scaffold project (`hello-counter`, `blank`, `tabs`, `stack`, `api-client`, `kitchen-sink`) |
| `ferrix doctor` | Diagnose and auto-remediate Android SDK/NDK/Rust toolchain prerequisites |
| `ferrix dev` | Start development loop with file watching, rebuilds, and logcat |
| `ferrix run android` | Compile native library and launch on connected device or emulator |
| `ferrix build` | Build release APK or Google Play AAB bundle |
| `ferrix check` | Run type checking and static linter rules |
| `ferrix fmt` | Idempotent code formatter preserving comments and layout trivia |
| `ferrix test` | Run headless component and unit tests |
| `ferrix add <module>` | Add platform module (`camera`, `location`, `sqlite`, `i18n`, `devtools`) |
| `ferrix prebuild` | Materialize native `android/` project for Android Studio |
| `ferrix clean` | Clean target and Gradle build caches |

---

## 🧪 Testing

Ferrix includes comprehensive host and Android unit test suites:

```bash
# Run 120+ host Rust tests across all 22 workspace crates
cargo test --workspace

# Run Kotlin Android runtime unit tests under Robolectric (SDK 34)
cd android && ./gradlew testDebugUnitTest
```

---

## 📄 License

Ferrix is dual-licensed under either:
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
