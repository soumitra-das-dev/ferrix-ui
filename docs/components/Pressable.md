# Pressable Component Specification

`Pressable` is the core touch gesture responder primitive in Ferrix, replacing legacy touchables (`TouchableOpacity`, `TouchableHighlight`).

## 1. Native Mapping
- **Android**: `android.view.ViewGroup` or custom responder view configured with Android ripple drawable (`android.R.attr.selectableItemBackground`) and gesture responder arbitration.

## 2. Props & Properties
```fsx
type PressableStateCallbackType = {
    pressed: bool,
};

type PressableProps = {
    // Style can be static or dynamic function of pressed state
    style?: Style | ((state: PressableStateCallbackType) => Style),
    // State & Gestures
    disabled?: bool,
    hitSlop?: Rect | f32,
    pressRetentionOffset?: Rect | f32,
    android_ripple?: RippleConfig,
    // Events
    onPress?: () => void,
    onPressIn?: () => void,
    onPressOut?: () => void,
    onLongPress?: () => void,
    // Accessibility
    accessible?: bool,
    accessibilityRole?: AccessibilityRole,
    accessibilityLabel?: String,
    accessibilityHint?: String,
    // Children: static or dynamic based on pressed state
    children?: Element | ((state: PressableStateCallbackType) => Element),
};
```

## 3. Responder Arbitration
- Handled natively in Kotlin to avoid cross-thread touch jitter.
- Intercepts touch down, tracks touch slop window (`hitSlop`).
- Cancels press if gesture exceeds `pressRetentionOffset` or parent scroll view claims responder.
- Updates pressed visual feedback instantaneously on Android UI thread and dispatches batched press events back to Rust runtime.

