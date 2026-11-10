# Text Component Specification

`Text` is the primitive component for displaying text in Ferrix.

## 1. Native Mapping
- **Android**: `android.widget.TextView`.
- Uses intrinsic text measurement calculated via batched Android `StaticLayout` synchronized to Taffy layout engine (see `ADR-0003`).

## 2. Props & Properties
```fsx
type TextProps = {
    // Content & Style
    style?: TextStyle | TextStyle[],
    numberOfLines?: i32,
    ellipsizeMode?: "head" | "middle" | "tail" | "clip",
    selectable?: bool,
    allowFontScaling?: bool,
    // Accessibility
    accessible?: bool,
    accessibilityLabel?: String,
    accessibilityHint?: String,
    accessibilityRole?: AccessibilityRole,
    // Events
    onPress?: () => void,
    // Children: string or nested Text spans
    children?: String | Element | (String | Element)[],
};
```

## 3. Style Capabilities
- **Color & Background**: `color`, `backgroundColor`.
- **Typography**:
  - `fontSize` (sp / scaled pixels).
  - `fontFamily` (system sans-serif, serif, monospace, or custom font asset).
  - `fontWeight` ('normal', 'bold', '100'..'900').
  - `fontStyle` ('normal', 'italic').
  - `lineHeight` (sp).
  - `letterSpacing` (sp / em).
  - `textAlign` ('auto', 'left', 'right', 'center', 'justify').
  - `textDecorationLine` ('none', 'underline', 'line-through', 'underline line-through').
  - `textTransform` ('none', 'uppercase', 'lowercase', 'capitalize').
- **Layout**: Text participates in flexbox layout with intrinsic dimensions computed based on text content, font styling, and max width constraints.

