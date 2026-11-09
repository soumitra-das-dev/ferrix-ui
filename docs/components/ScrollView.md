# ScrollView Component Specification

`ScrollView` is a scrollable container wrapping components that exceed viewport boundaries.

## 1. Native Mapping
- **Android**: `androidx.core.widget.NestedScrollView` (for vertical) or `android.widget.HorizontalScrollView` (for horizontal).
- Throttles and coalesces high-frequency `onScroll` events before crossing JNI into Rust runtime.

## 2. Props & Properties
```fsx
type ScrollEvent = {
    contentOffset: { x: f32, y: f32 },
    contentSize: { width: f32, height: f32 },
    layoutMeasurement: { width: f32, height: f32 },
};

type ScrollViewProps = {
    horizontal?: bool,
    pagingEnabled?: bool,
    showsHorizontalScrollIndicator?: bool,
    showsVerticalScrollIndicator?: bool,
    scrollEnabled?: bool,
    onScroll?: (event: ScrollEvent) => void,
    scrollEventThrottle?: i32,
    refreshControl?: Element,
    style?: Style | Style[],
    children?: Element | Element[],
    testID?: String,
};
```
