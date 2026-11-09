# Switch Component Specification

`Switch` renders a native two-state toggle switch.

## 1. Native Mapping
- **Android**: `androidx.appcompat.widget.SwitchCompat` or `android.widget.Switch`.

## 2. Props & Properties
```fsx
type SwitchProps = {
    value: bool,
    onValueChange?: (value: bool) => void,
    disabled?: bool,
    trackColor?: { false?: u32, true?: u32 },
    thumbColor?: u32,
    accessible?: bool,
    accessibilityLabel?: String,
    testID?: String,
};
```
