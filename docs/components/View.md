# View Component Specification

`View` is the fundamental building block for UI in Ferrix, equivalent to `<div>` on web or `View` in React Native.

## 1. Native Mapping
- **Android**: `android.view.ViewGroup` / `android.widget.FrameLayout` (or `com.ferrix.runtime.views.FerrixViewGroup`).
- Views can act as layout containers or visual containers (background, border, shadow).
- Layout-only views with no visual properties or event listeners can be flattened during optimization.

## 2. Props & Properties
```fsx
type ViewProps = {
    // Layout
    style?: Style | Style[],
    // Accessibility
    accessible?: bool,
    accessibilityLabel?: String,
    accessibilityHint?: String,
    accessibilityRole?: AccessibilityRole,
    accessibilityState?: AccessibilityState,
    importantForAccessibility?: "auto" | "yes" | "no" | "no-hide-descendants",
    // Identification
    testID?: String,
    nativeID?: String,
    // Events
    onLayout?: (event: LayoutChangeEvent) => void,
    // Children
    children?: Element | Element[],
};
```

## 3. Style Capabilities
- **Flexbox**: `flexDirection`, `flex`, `flexGrow`, `flexShrink`, `flexBasis`, `alignItems`, `alignSelf`, `alignContent`, `justifyContent`, `flexWrap`, `gap`, `rowGap`, `columnGap`.
- **Dimensions & Sizing**: `width`, `height`, `minWidth`, `maxWidth`, `minHeight`, `maxHeight`, `aspectRatio`.
- **Positioning**: `position` (`relative` | `absolute`), `top`, `bottom`, `left`, `right`, `start`, `end`, `zIndex`.
- **Spacing**: `margin`, `padding` (directional, axis, start/end).
- **Appearance**: `backgroundColor`, `opacity`, `overflow` (`visible` | `hidden` | `scroll`).
- **Borders**: `borderWidth`, `borderColor`, `borderRadius` (all sides and individual corners).
- **Elevation / Shadows**: `elevation`, `shadowColor`, `shadowOffset`, `shadowOpacity`, `shadowRadius`.
- **Transforms**: `translateX`, `translateY`, `scale`, `rotate`.

## 4. Children Rules
- May contain 0, 1, or multiple child `Element`s.
- Text strings cannot appear directly inside `View`; they must be wrapped in a `Text` component.

