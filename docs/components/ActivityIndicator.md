# ActivityIndicator Component Specification

`ActivityIndicator` displays a circular loading indicator.

## 1. Native Mapping
- **Android**: `android.widget.ProgressBar` with `isIndeterminate = true`.

## 2. Props & Properties
```fsx
type ActivityIndicatorSize = "small" | "large" | f32;

type ActivityIndicatorProps = {
    animating?: bool,
    color?: u32,
    size?: ActivityIndicatorSize,
    hidesWhenStopped?: bool,
    style?: Style | Style[],
    testID?: String,
};
```
