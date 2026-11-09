# Ferrix Styling Specification: Native CSS Subset & Pipeline

## 1. Architecture & Design Principles

Ferrix implements a three-layer styling pipeline compiled ahead-of-time (AOT) to Rust under **Architecture A**:
1. **Developer Surface**: `StyleSheet.create({...})`, `.css` files, `.module.css` (CSS Modules), and `className` / `class`.
2. **Build-Time Compilation**: CSS is parsed and normalized into a typed **Native Style IR**. Static styles are resolved at compile time to interned `u32` style IDs. Dynamic styles (variants, dark mode, media queries, CSS variables) compile to reactive signals.
3. **Runtime Execution**: Rust resolves cascade and specificity; Taffy computes layout geometry; Android Kotlin applies batched style patches (`RegisterStyles`, `SetStyle`, `SetStyleList`) to native Views.
4. **Zero Web Overhead**: No browser engine, no WebViews, no Node.js runtime, and no CSS string parsing ship in the final Android APK.

---

## 2. CSS Subset Support Matrix

### 2.1 Selectors

| Selector | Status | Native Mapping / Behavior | Notes |
|---|:---:|---|---|
| Class selector (`.card`) | ✅ | Interned style ID mapped to element `className` | Static resolution at codegen |
| Compound class (`.btn.primary`) | ✅ | Multi-class specificity combination | Ordered cascade resolution |
| State pseudo-class `:active` | ✅ | Mapped to native pressed state (`PressIn` / `PressOut`) | Native ripple & touch responder |
| State pseudo-class `:focus` | ✅ | Mapped to native focus state (`OnFocusChangeListener`) | Controlled by View focus hierarchy |
| State pseudo-class `:disabled` | ✅ | Mapped to native `isEnabled = false` | Disables touch and click dispatch |
| State pseudo-class `:checked` | ✅ | Mapped to native `isChecked` (`CompoundButton`) | Switch, Checkbox, RadioGroup |
| Root pseudo-class `:root` | ✅ | Global theme variables / design tokens | Stored in static / reactive token store |
| Universal selector `*` | ✅ | Applied as base rule for all matching elements | Lowest specificity |
| Direct-child combinator (`>`) | ✅ | Parent-context matching during element tree build | Resolves styles for immediate children |
| Descendant combinator (` `) | ✅ | Contextual inherited style propagation | Inherited via ComponentContext tree |
| Pseudo-elements (`::before`, `::after`) | ❌ | Not supported (violates 1:1 native view mapping) | Compile-time diagnostic with `file:line:col` |
| Web pseudo-classes (`:hover` on touch, `:visited`, `:target`) | ❌ | Unsupported on touch devices | Rejected with diagnostic (hover enabled only for mouse pointers) |

---

### 2.2 At-Rules

| At-Rule | Status | Native Mapping / Behavior | Notes |
|---|:---:|---|---|
| `@import` | ✅ | File inclusion and bundling into single style sheet | Fast-path bundling; cycles detected |
| `@media (prefers-color-scheme: dark)` | ✅ | Binds to reactive `ColorScheme` signal | Fine-grained patch update on theme change |
| `@media (min-width: ...dp)` | ✅ | Binds to `use_window_dimensions()` width signal | Responsive layout breakpoints |
| `@media (max-width: ...dp)` | ✅ | Binds to `use_window_dimensions()` width signal | Responsive layout breakpoints |
| `@media (orientation: portrait/landscape)` | ✅ | Evaluates window aspect ratio | Reactive orientation switches |
| `@media (prefers-reduced-motion)` | ✅ | Evaluates system accessibility motion preference | Disables native driver transitions |
| `@media (min-font-scale: ...)` | ✅ | Evaluates system accessibility font scaling | Adaptive text layout |
| `@media (platform: android)` | ✅ | Static compile-time target evaluation | Dead-code eliminated on other platforms |
| `@layer` | ✅ | CSS cascade layers with explicit priority order | Earlier layers overridden by later layers |
| `@supports` | ✅ | Static feature evaluation at compile time | Evaluates true for supported native properties |
| Nesting (`& > ...`, `&:active`) | ✅ | Flattened to compound selectors at compile time | Zero runtime nesting cost |
| `@keyframes` | ✅ | Compiles to native Android Animator / Keyframe specs | Frame-by-frame Kotlin animation driver |
| `@font-face` | ✅ | Maps to bundled OTF/TTF fonts in `assets/fonts/` | Bound via `ferrix-font` font asset manager |

---

### 2.3 Values & Functions

| Value / Function | Status | Native Mapping / Behavior | Notes |
|---|:---:|---|---|
| `var(--custom-prop, fallback)` | ✅ | Scoped CSS variable resolution | `:root` or component context |
| `calc(...)` | ✅ | Static arithmetic resolved at compile time; dynamic `%` resolved by Taffy | Mixes dp, sp, `%` |
| Colors: Hex (`#rgb`, `#rrggbb`, `#rrggbbaa`) | ✅ | Parsed to 32-bit ARGB `u32` | Zero allocation |
| Colors: `rgb(...)` / `rgba(...)` | ✅ | Normalized to ARGB `u32` at compile time | Supports floating-point alpha |
| Colors: `hsl(...)` / `hsla(...)` | ✅ | Converted to sRGB ARGB `u32` at compile time | Accurate color space conversion |
| Colors: `oklch(...)` | ✅ | Converted to perceptual sRGB `u32` at compile time | Supports modern wide-gamut palettes |
| Colors: `color-mix(in srgb, ...)` | ✅ | Linearly interpolated ARGB `u32` at compile time | Compile-time color blending |
| Units: `px`, `dp` | ✅ | Density-independent pixels (1dp = 1 physical pt at 160dpi) | Native layout baseline |
| Units: `sp` | ✅ | Scaled pixels for typography; scaled by system font scale | Accessible font rendering |
| Units: `rem`, `em` | ✅ | Relative units (1rem = 16dp baseline, or configurable) | Converted to dp at compile time |
| Units: `%` | ✅ | Taffy `Dimension::Percent(f32)` | Relative to parent frame |
| Units: `vw`, `vh` | ✅ | Window width/height viewport percentage | Resolved from window dimensions |
| `env(safe-area-inset-*)` | ✅ | Inset metrics provided by Android `WindowInsetsCompat` | Dynamic padding / margin offsets |
| Shorthand expansion | ✅ | `margin`, `padding`, `border`, `border-radius`, `gap`, `inset`, `flex` expanded to longhands | Compile-time normalization |

---

### 2.4 Layout & Flexbox Defaults (RN vs Web)

> [!IMPORTANT]
> Ferrix defaults follow **React Native conventions**, not web defaults:
> - `display: flex` (default for all layout containers)
> - `flex-direction: column` (not `row`)
> - `position: relative` (not `static`)
> - `box-sizing: border-box` (all padding and border included in width/height)
> - `min-width: 0`, `min-height: 0` (flex shrink by default)

| Property | Status | Native Mapping / Taffy Value |
|---|:---:|---|
| `flex-direction: column / row / column-reverse / row-reverse` | ✅ | `taffy::style::FlexDirection` |
| `flex-wrap: nowrap / wrap / wrap-reverse` | ✅ | `taffy::style::FlexWrap` |
| `justify-content: flex-start / center / flex-end / space-between / space-around / space-evenly` | ✅ | `taffy::style::JustifyContent` |
| `align-items: flex-start / center / flex-end / stretch / baseline` | ✅ | `taffy::style::AlignItems` |
| `align-self: auto / flex-start / center / flex-end / stretch / baseline` | ✅ | `taffy::style::AlignSelf` |
| `align-content: flex-start / center / flex-end / stretch / space-between / space-around` | ✅ | `taffy::style::AlignContent` |
| `flex-grow`, `flex-shrink`, `flex-basis` | ✅ | Taffy flex items |
| `gap`, `row-gap`, `column-gap` | ✅ | Taffy flex and grid gaps |
| `grid-template-columns`, `grid-template-rows` | ✅ | Taffy CSS Grid track sizing |
| `grid-column`, `grid-row` | ✅ | Taffy grid item placement |
| `position: relative / absolute` | ✅ | Taffy position |
| `top`, `bottom`, `left`, `right`, `inset`, `start`, `end` | ✅ | Taffy inset edges (supporting RTL start/end) |
| `width`, `height`, `min-width`, `max-width`, `min-height`, `max-height` | ✅ | Taffy dimensions |
| `aspect-ratio` | ✅ | Taffy aspect ratio |
| `overflow: visible / hidden / scroll` | ✅ | View clipping and scroll enablement |
| `z-index` | ✅ | Android View elevation / z-ordering |

---

### 2.5 Visual Appearance & Typography

| Property | Status | Native Mapping / Property |
|---|:---:|---|
| `background-color` | ✅ | `Property::BackgroundColor(u32)` |
| `opacity` | ✅ | `Property::Opacity(f32)` |
| `border-width` (all sides, per-side) | ✅ | `Property::BorderWidth { left, top, right, bottom }` |
| `border-color` | ✅ | `Property::BorderColor(u32)` |
| `border-radius` (all corners, per-corner) | ✅ | `Property::BorderRadius { top_left, top_right, bottom_right, bottom_left }` |
| `box-shadow` / `elevation` | ✅ | Native Android elevation + shadow parameters |
| `transform` (`translate`, `scale`, `rotate`) | ✅ | `Property::Transform { translate_x, translate_y, scale_x, scale_y, rotate_deg }` |
| `color` | ✅ | `Property::TextColor(u32)` |
| `font-size` | ✅ | `Property::FontSize(f32)` (sp) |
| `font-weight` (100–900, `bold`, `normal`) | ✅ | `Property::FontWeight(u16)` |
| `font-style` (`italic`, `normal`) | ✅ | `Property::FontStyle(u8)` |
| `line-height` | ✅ | `Property::LineHeight(f32)` |
| `letter-spacing` | ✅ | `Property::LetterSpacing(f32)` |
| `text-align` (`left`, `center`, `right`, `justify`) | ✅ | `Property::TextAlign(u8)` |
| `text-decoration` (`underline`, `line-through`, `none`) | ✅ | `Property::TextDecoration(u8)` |
| `text-transform` (`uppercase`, `lowercase`, `capitalize`) | ✅ | Text transformation in layout text preparation |
| `pointer-events` (`auto`, `none`, `box-none`, `box-only`) | ✅ | Touch responder dispatch policy |

---

### 2.6 Unsupported Web-Only Properties (Compile-Time Diagnostic ❌)

The following properties do not exist in native mobile view hierarchies and will produce a compile-time diagnostic with `file:line:col`:
- `float`, `clear` (flex/grid layout must be used instead)
- `cursor` (touch targets do not have mouse cursors)
- `columns`, `column-count`
- `table`, `display: table-cell`
- `white-space` (use `numberOfLines` / ellipsize modes)
- `visibility: collapse`
- `filter` (blur/filters must be specified via native image/shader effects)

---

## 3. Merge Order & Specificity

Styles on any element are merged in the following order (from lowest to highest precedence):
$$\text{Universal (*)} < \text{Global CSS Rules} < \text{CSS Module Rules} < \text{Inline } \texttt{style} \text{ Props}$$

Within CSS rules, standard CSS specificity applies:
$$(a, b, c) \text{ where } a = \text{ID selectors (0 in Ferrix)}, b = \text{Classes \& Pseudo-classes}, c = \text{Type/Universal selectors}$$

When specificities are equal, source order applies (later rule wins).
Inline `style={...}` props always take final precedence.

