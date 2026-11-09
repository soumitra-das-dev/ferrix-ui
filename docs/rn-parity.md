# React Native & Expo Feature Parity Board

This document tracks the implementation status of React Native Core primitives and Expo SDK platform modules in Ferrix under **Architecture A** (Rust owns application logic, reactive state, reconciliation, scheduling, and Taffy layout; Kotlin owns Android UI lifecycle, View rendering, platform APIs, and ephemeral interaction state).

## Legend
- ✅ **Done**: Fully implemented, tested (Rust + Kotlin), spec documented, and example available.
- 🟡 **In Progress**: Actively being implemented or in review.
- ⭕ **Planned**: Scheduled for upcoming waves.

---

## 1. Wave 0: Foundations & Cross-Cutting Systems
| System | Subsystems | Status | Notes |
|---|---|---|---|
| **Style System** | `StyleSheet.create`, Flexbox props, Spacing, Visual, Colors, Transforms | ✅ Done | Taffy 0.7 mapping, compile-time static & runtime dynamic |
| **Text Measurement** | Taffy intrinsic measure, StaticLayout cache | ✅ Done | ADR-0003 batched measurement, zero per-node JNI chatter |
| **Event & Gestures** | Press responder, ripple, hitSlop, nested arbitration | ✅ Done | Resolved in Kotlin, reported to Rust |
| **Theming** | `useColorScheme`, dynamic colors, ThemeContext | ✅ Done | Light/dark, Material You tokens |
| **Accessibility** | ARIA-like props (`accessibilityRole`, label, hint, state), `testID` | ✅ Done | Standardized on all primitives |
| **i18n & RTL** | `start`/`end` logical props, `I18nManager` | ✅ Done | Automatic layout inversion |
| **Refs & Handles** | `useRef`, `focus()`, `blur()`, `measure()`, `scrollTo()` | ✅ Done | Typed IPC commands Rust → Kotlin |
| **Fragments** | `<Fragment>`, `<>` transparent containers | ✅ Done | AST + codegen support |

---

## 2. Wave 1: Core Primitives (React Native Parity)
| Component | Native Android Mapping | Status | Key Features |
|---|---|---|---|
| `View` (aka `Box`) | `ViewGroup` / `FrameLayout` | ✅ Done | Pointer events, layout flattening, `onLayout` |
| `Text` | `TextView` | ✅ Done | Spans, `numberOfLines`, `ellipsizeMode`, font scaling |
| `Image` | `ImageView` + Coil | ✅ Done | uri/asset/base64, resizeMode, onLoad/onError |
| `ImageBackground` | Composite View + Image | ✅ Done | Nested child rendering over background image |
| `TextInput` | `EditText` | ✅ Done | Controlled version counter (no cursor jump), IME composition |
| `Pressable` | Custom View / Ripple | ✅ Done | `pressed` state style function, ripple, hitSlop |
| `Button` | `MaterialButton` / `Button` | ✅ Done | Native button with title and click event |
| `ScrollView` | `NestedScrollView` | ✅ Done | Vertical/horizontal, paging, scroll events |
| `LazyList` (FlatList) | Virtualized / `RecyclerView` | ✅ Done | Windowed item virtualization, 10k items 60fps |
| `SectionList` | Virtualized / `RecyclerView` | ⭕ Planned | Sticky headers, sectioned items |
| `Switch` | `SwitchCompat` | ✅ Done | Controlled boolean toggle |
| `ActivityIndicator` | `ProgressBar` | ✅ Done | Indeterminate spinner, color, size |
| `Modal` | `Dialog` / `PopupWindow` | ✅ Done | Overlay with animation, back handler |
| `StatusBar` | `WindowInsetsController` | ⭕ Planned | Light/dark content, translucent, background |
| `SafeAreaView` | Insets padding container | ✅ Done | `useSafeAreaInsets` hook integration |
| `KeyboardAvoidingView` | Window insets padding/adjust | ⭕ Planned | Behavior: padding, height, position |
| `RefreshControl` | `SwipeRefreshLayout` | ✅ Done | Pull-to-refresh binding for ScrollView |
| `TouchableOpacity` | Pressable wrapper | ✅ Done | Opacity animation on touch down |
| `TouchableHighlight` | Pressable wrapper | ✅ Done | Background underlay highlight on press |
| `TouchableWithoutFeedback`| Pressable wrapper | ✅ Done | Event capture without visual change |
| `Portal` | Root overlay container | ✅ Done | Escape hierarchy to app root |

---

## 3. Wave 2: Extended UI Primitives
| Component Group | Components | Status | Notes |
|---|---|---|---|
| **Layout** | `Column`, `Row`, `Stack`/`ZStack`, `Spacer`, `Divider`, `Center`, `Grid` | 🟡 In Progress | High-level ergonomic layout helpers |
| **Inputs** | `Checkbox`, `RadioGroup`, `Slider`, `Picker`, `DatePicker`, `TimePicker`, `SegmentedControl`, `Stepper` | ⭕ Planned | Material 3 native widgets |
| **Overlays** | `Toast`, `Alert`, `BottomSheet`, `Tooltip`, `Badge`, `ProgressBar`, `Skeleton` | ⭕ Planned | Native overlays & loading skeletons |
| **Media & Display** | `Icon`, `Svg`, `LinearGradient`, `BlurView`, `Video`, `Audio`, `WebView` | ⭕ Planned | Vectors, gradients, media players |
| **Animations** | `Animated.Value`, spring/timing drivers, interpolations | 🟡 In Progress | Frame-by-frame Kotlin execution |
| **Gestures** | `PanGesture`, `Tap`, `Pinch`, `Rotation`, `Fling`, `Swipeable` | ⭕ Planned | Kotlin gesture recognizers |
| **Navigation** | `ferrix-navigation`, Stack, Tabs, Drawer, Transitions | ⭕ Planned | Native fragment/view transitions |

---

## 4. Wave 3: Platform APIs & Hooks
| Module / Hook | Platform Binding | Status | Notes |
|---|---|---|---|
| `Dimensions` / `useWindowDimensions` | `DisplayMetrics` / `WindowMetrics` | ⭕ Planned | Reactive window size updates |
| `PixelRatio` | `DisplayMetrics.density` | ⭕ Planned | DP to PX conversion helpers |
| `Platform` | `Build.VERSION`, OS constants | ✅ Done | OS name, version, isTesting |
| `Appearance` | `Configuration.uiMode` | 🟡 In Progress | Dark / light theme listener |
| `Keyboard` | Window insets IME listener | ⭕ Planned | Show, hide, keyboard height signal |
| `AppState` | ProcessLifecycleOwner | ⭕ Planned | `active`, `background`, `inactive` |
| `BackHandler` | `OnBackPressedDispatcher` | ⭕ Planned | Hardware back button interception |
| `Linking` | `Intent.ACTION_VIEW` | ⭕ Planned | Deep links, URL opening |
| `Share` | `Intent.ACTION_SEND` | ⭕ Planned | Text, image, document sharing |
| `Vibration` | `Vibrator` / `VibratorManager` | ⭕ Planned | Vibration patterns, cancellation |
| `Clipboard` | `ClipboardManager` | ⭕ Planned | Get/set string content |
| `AccessibilityInfo` | `AccessibilityManager` | ⭕ Planned | Screen reader status listener |
| `I18nManager` | `Locale.getDefault()` | 🟡 In Progress | RTL query and force RTL |
| `NetInfo` | `ConnectivityManager` | ⭕ Planned | Cellular, WiFi, offline state signals |

---

## 5. Wave 4: Expo-Style Native Modules
| Module | Capability | Status |
|---|---|---|
| `ferrix-camera` | CameraX photo/video capture, preview, torch | ✅ Done |
| `ferrix-location` | Fused Location Provider, geofencing, accuracy | ✅ Done |
| `ferrix-sensors` | Accelerometer, Gyroscope, Magnetometer, Pedometer | ⭕ Planned |
| `ferrix-audio` | Audio player, recorder, background playback | ⭕ Planned |
| `ferrix-haptics` | Haptic feedback (light, medium, heavy, success, error) | ⭕ Planned |
| `ferrix-secure-store` | EncryptedSharedPreferences / Android Keystore | ⭕ Planned |
| `ferrix-file-system` | App internal/external storage, read/write/delete | ✅ Done |
| `ferrix-image-picker` | Photo picker contract, camera intent | ⭕ Planned |
| `ferrix-document-picker` | Storage Access Framework file picker | ⭕ Planned |
| `ferrix-media-library` | MediaStore access, query albums and photos | ⭕ Planned |
| `ferrix-notifications` | Android NotificationManager, channels, actions | ⭕ Planned |
| `ferrix-local-authentication`| BiometricPrompt (fingerprint, face unlock) | ⭕ Planned |

