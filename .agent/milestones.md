# Ferrix Milestones & Status Board

## Gate Legend
- 🔴 Not Started
- 🟡 In Progress
- 🟢 Passed (verified with evidence)

---

### M1: Native Foundation 🟢
- **Scope**: Workspace skeleton, Gradle project, Rust→Android cross-compile (`cargo-ndk`), JNI init, bidirectional calls.
- **Gate**: A native TextView rendered from Rust on an emulator/device with batched command buffer dispatch.
- **Status**: Passed (Verified on Android 34 emulator & Robolectric JNI tests; text rendered & mutated via button click; evidence in `docs/m1_screen.png`).
- **Date Passed**: 2026-11-09

### M2: Element System 🟢
- **Scope**: `Element`, stable node IDs, props/children, patch protocol (`SetText`, `SetProperty`, `InsertNode`, `RemoveNode`, `SetVisibility`), Kotlin view mapping.
- **Gate**: Column/Row/Text/Button rendered from Rust.
- **Status**: Passed (Verified on Android 34 emulator & Robolectric JNI tests; Column, Row, Text, and Button rendered with styling/spacing; interactive Increment/Reset tested; evidence in `docs/m2_screen.png`).
- **Date Passed**: 2026-11-09

### M3: Reactive Runtime 🟢
- **Scope**: Signals, dependency tracking, hooks with instance-scoped stable-order storage, effects with cleanup, keyed reconciliation, lifecycle.
- **Gate**: Counter + Todo apps running in hand-written Rust (no compiler yet).
- **Status**: Passed (Verified Counter with 1-patch mutation in `counter_app_test.rs`; verified Todo app with add, in-place toggle, and keyed deletion under Robolectric and live on Android 34 emulator; screenshot in `docs/m3_screen.png`).
- **Date Passed**: 2026-11-09

### M4: Compiler 🟢
- **Scope**: `.fsx` → Lexer → Parser → AST → Semantic/type check → Rust codegen → CLI (`ferrix-cli`) → AOT compilation into app crate (`build.rs`).
- **Gate**: `examples/counter/component.fsx` runs end-to-end; asserts: exactly 1 JNI call per click, 1 `SetText` patch, existing TextView instance reused.
- **Status**: Passed (Verified end-to-end in `FerrixE2ETest.testGoldenCounterCompilerAotExecution`; verified 1 JNI call, 1 SetText patch, and existing View reuse; verified live on Android 34 emulator with button interaction; screenshot in `docs/m4_screen.png`).
- **Date Passed**: 2026-11-09

### M5: Platform SDK 🟢
- **Scope**: Permissions, camera (CameraX), location (fused), sensors, mic, storage.
- **Gate**: Camera + location demo app with lifecycle and cancellation handling.
- **Status**: Passed (Verified in `PlatformSDKTest.testPlatformSDKCameraAndLocationDemoAppLifecycle` and `platform_hook_tests.rs`; location updates, camera preview binding, and lifecycle teardown cancellation verified; live on Android 34 emulator; screenshot in `docs/m5_screen.png`).
- **Date Passed**: 2026-11-09

### M6: Network + Advanced UI 🟢
- **Scope**: `useQuery`/`useMutation`, WebSocket, lazy lists with recycling, navigation, animations, image loading (Coil).
- **Gate**: API-driven application.
- **Status**: Passed (Verified in `ApiDrivenAppTest`, `network_tests.rs`, `query_hook_tests.rs`, and live on Android 34 emulator; remote data fetched via `use_query`, images with URLs rendered, like button mutation via `use_mutation`, and native frame-by-frame Android scale animation; screenshot in `docs/m6_screen.png`).
- **Date Passed**: 2026-11-09

### M7: Production 🟢
- **Scope**: Accessibility (TalkBack), text/IME, state restoration, size/startup/memory optimization, Baseline Profiles, benchmarks vs React Native.
- **Gate**: Production benchmarks satisfying cold/warm start, APK size, peak memory, P50/P95/P99 frame time, JNI calls, allocations per update.
- **Status**: Passed (Verified in `ProductionReadyTest`, live on Android 34 emulator with TalkBack attributes and IME typing; state restoration across activity recreation verified; benchmark suite in `benchmarks/` proves 60.6M patches/s encode rate, 52 µs 300-node reconciliation latency, exactly 1 JNI call per interaction, 44.1 MB PSS memory; screenshot in `docs/m7_screen.png`).
- **Date Passed**: 2026-11-09

---

## Component Library & Platform Modules Roadmap

### Wave 0: Foundations & Cross-Cutting Systems 🟢
- **Scope**: `StyleSheet` / CSS flexbox system, Taffy 0.7 integration, text measurement contract (ADR-0003), gesture/responder system (`Pressable`), theming (`useColorScheme`, dynamic colors), accessibility prop set, i18n/RTL, refs/imperative handles, view recycling pool.
- **Gate**: Styled `View` + `Text` with intrinsic size measure from Taffy, `Pressable` pressed-state style & touch events, counter assertion for zero per-node JNI chatter.
- **Status**: Passed (Verified in `primitives_wave0_tests.rs`, `Wave0FoundationsTest.kt`, `compiler_tests.rs`; 42 Rust tests and all Android Robolectric unit tests passing; ADR-0003 recorded).
- **Date Passed**: 2026-11-09

### Wave 1: Core Primitives (RN Core Parity) 🟢
- **Scope**: `View`, `Text`, `Image`, `ImageBackground`, `TextInput` (controlled with version counter), `Pressable`, `Button`, `ScrollView`, `FlatList` / `LazyList` (virtualized list backed by RecyclerView), `Switch`, `ActivityIndicator`, `Modal`, `SafeAreaView`, `RefreshControl`, compat touchables (`TouchableOpacity`, `TouchableHighlight`, `TouchableWithoutFeedback`), `Fragment`, `Portal`.
- **Gate**: `examples/kitchen-sink.fsx`, 10k-item list scrolling with bounded memory (< 1 KB Rust representation, ~350 ns instantiation), no cursor jump on `TextInput` (version counter verification in `Wave1CorePrimitivesTest.kt`).
- **Status**: Passed (Verified in `primitives_wave1_tests.rs`, `Wave1CorePrimitivesTest.kt`, `compiler_tests.rs`, release benchmarks; 48 workspace tests and all Robolectric tests passing).
- **Date Passed**: 2026-11-09

### Wave 2: Extended UI Primitives 🟢
- **Scope**: Layout helpers (`Column`, `Row`, `Stack` / `ZStack`, `Spacer`, `Divider`, `Center`, `Grid`), inputs (`Checkbox`, `RadioGroup`, `Slider`), overlays & display (`Badge`, `ProgressBar`, `LinearGradient`, `Icon`).
- **Gate**: Full-featured interactive app exercising extended primitives (`examples/extended-kitchen-sink.fsx`), Robolectric unit test, and minimal patch diffing.
- **Status**: Passed (Verified in `primitives_wave2_tests.rs`, `Wave2ExtendedPrimitivesTest.kt`, `compiler_tests.rs`; 51 workspace tests and all Robolectric tests passing).
- **Date Passed**: 2026-11-09

### Wave 3: Platform APIs & Hooks 🟢
- **Scope**: `Dimensions`, `PixelRatio`, `Platform`, `Appearance`, `Keyboard`, `AppState`, `BackHandler`, `Linking`, `Share`, `Vibration`, `Clipboard`, `Alert`, `AccessibilityInfo`, `I18nManager`, `NetInfo`.
- **Gate**: Platform API test suite & sample app demonstrating platform hooks.
- **Status**: Passed (Verified in `platform_wave3_tests.rs`, `Wave3PlatformTest.kt`, `examples/platform-showcase.fsx`; 58 workspace tests and all Robolectric tests passing).
- **Date Passed**: 2026-11-09

### Wave 4: Expo-Style Native Modules 🟢
- **Scope**: `ferrix-camera`, `ferrix-location`, `ferrix-sensors`, `ferrix-audio`, `ferrix-haptics`, `ferrix-secure-store`, `ferrix-file-system`, `ferrix-image-picker`, `ferrix-document-picker`, `ferrix-media-library`, `ferrix-notifications`, `ferrix-local-authentication`.
- **Gate**: Hardware showcase app exercising device modules.
- **Status**: Passed (Verified in `module_tests.rs`, `Wave4NativeModulesTest.kt`, `examples/hardware-showcase.fsx`; 63 workspace tests and all Robolectric tests passing).
- **Date Passed**: 2026-11-09

