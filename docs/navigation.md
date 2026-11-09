# Ferrix Navigation Specification (F2)

## 1. Overview & Architecture
Ferrix Navigation provides native navigation containers (`Stack`, `Tabs`, `Drawer`, `Modal`) for Android under **Architecture A**:
- **Single Source of Truth in Rust**: Navigation hierarchy, screen stack, active tab selection, and transition states are owned by Rust (`crates/ferrix-navigation`).
- **Native View Containers in Kotlin**: Rendered as high-performance custom `ViewGroup`s (`FerrixNavContainer`, `FerrixScreenView`, `FerrixHeaderView`, `FerrixTabBarView`) avoiding Android `Fragment` / `FragmentManager` transaction deadlocks and asynchronous race conditions.
- **Hardware-Accelerated Transitions**: Native 60fps / 120fps animations run frame-by-frame on Android using Kotlin `ValueAnimator` / `ViewPropertyAnimator` (SlideFromRight, Fade, SlideFromBottom for modals). Rust specifies transition targets; Kotlin drives animations without blocking Rust.
- **Predictive Back & Android BackHandler**: Integrated with Android 14+ `OnBackPressedDispatcher` / `OnBackAnimationCallback`.

---

## 2. Protocol Extensions

### 2.1 ViewTypes
- `NavContainer = 31`: Root navigation container managing screen transitions.
- `Screen = 32`: Single screen view containing page elements and lifecycle state.
- `Header = 33`: Native navigation header/toolbar with title, back button, and actions.
- `TabBar = 34`: Native bottom tab navigation bar.
- `Drawer = 35`: Side navigation drawer container.

### 2.2 Imperative Commands
- `Command::Navigate { container_id, screen_id, animation }` (opcode `0x05`): Transitions active screen in container.
- `Command::GoBack { container_id, animation }` (opcode `0x06`): Pops the active screen.
- `Command::SetScreenOptions { screen_id, title, header_shown, presentation }` (opcode `0x07`): Updates screen metadata and native header options.

---

## 3. Native Android Containers

### 3.1 `FerrixNavContainer`
Custom `FrameLayout` that holds child `FerrixScreenView`s.
- `pushScreen(screenView, animation)`: Animates incoming screen into view and marks preceding screen inactive.
- `popScreen(animation)`: Animates outgoing screen out of view and reactivates previous screen.
- `replaceScreen(screenView, animation)`: Replaces active screen without adding to stack history.

### 3.2 `FerrixScreenView`
Custom `FrameLayout` with lifecycle status:
- `Active`: Screen is visible and interacting with user.
- `Inactive`: Screen is kept mounted in memory in the backstack but hidden.
- `Transitioning`: Screen is currently animating in or out.

### 3.3 `FerrixHeaderView`
Custom `LinearLayout` / toolbar:
- Back button with ripple effect, automatically hidden on stack root.
- Title text view configured via `SetScreenOptions`.
- Dispatches `Click` events when back button is pressed.

### 3.4 `FerrixTabBarView`
Custom bottom bar with item icons and labels:
- Emits `ValueChange` events with selected tab index.

---

## 4. Rust Navigation Engine (`crates/ferrix-navigation`)
- `StackNavigator`: Stack state machine managing push, pop, replace, and screen options. Emits `CommandBatch` to platform renderer.
- `TabNavigator`: Tab state machine managing active index and tab bar configuration.
- Seamless bridge with `ferrix-router`: maps router routes to navigation containers.

