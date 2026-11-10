# Phase 1: Stylesheet Support & Style Registry Plan

**Target**: Phase 1 of the Three-Layer Styling Pipeline
**Date**: 2026-11-09
**Status**: In Progress

---

## 1. Objectives

1. **Protocol Extension (Additive, Non-Breaking)**:
   - Add `RegisterStyles` (`0x0C`), `SetStyle` (`0x0D`), and `SetStyleList` (`0x0E`) to `ferrix-renderer-protocol`.
   - Update Android Kotlin runtime (`Patch.kt`, `CommandDecoder.kt`, `FerrixRenderer.kt`) to store registered styles in `styleRegistry` and apply them to native Views.
2. **Native Style IR & Runtime Engine (`crates/ferrix-style`)**:
   - `StyleIr`: Native style properties (layout, flex, grid, spacing, appearance, typography, transform).
   - `StyleRegistry`: Intern pool mapping `StyleIr` to monotonic `u32` style IDs.
   - `CascadeEngine`: Specificity calculation `(a, b, c)`, rule sorting, merge order: global CSS < module rules < inline `style`.
   - `ReactiveStyleResolver`: Signals for dark mode (`ColorScheme`), window width/height, font scale, `:active` / `:focus` / `:disabled` / `:checked` states, and CSS variables. Emits batched `SetStyle` / `SetStyleList` patches when signals change.
   - Utilities: `cx(...)` class merge, `variants(...)` (cva-style variant composer), `use_style`.
3. **Build-Time CSS Compiler & Adapter (`crates/ferrix-style-build`)**:
   - CSS subset parser for `.css` and `.module.css`.
   - Selector parsing: class `.a`, compound `.a.b`, state pseudo-classes (`:active`, `:focus`, `:disabled`, `:checked`), `:root`, `*`.
   - At-rules: `@import`, `@media` (color scheme, width/height breakpoints, reduced motion, orientation), `@layer`, `@supports`.
   - Values & units: `px/dp/rem/em/%/vw/vh/sp`, `var()`, `calc()`, color normalization (hex, rgb, rgba, hsl, oklch) to ARGB `u32`.
   - Shorthand expansions: `margin`, `padding`, `border`, `border-radius`, `gap`, `inset`, `flex`.
   - React Native layout defaults applied (`flex-direction: column`, `position: relative`, `display: flex`).
   - CSS Modules: deterministic class scoping hash generator (e.g. `card_header_7a8b9c`).
   - Diagnostics carrying `file:line:col` for unsupported properties or selectors.
4. **FSX Compiler Integration (`crates/ferrix-compiler`)**:
   - Parse CSS imports: `import "./global.css"` and `import styles from "./card.module.css"`.
   - Support `className` and `class` on all JSX elements.
   - Codegen: Resolve static `className` strings directly into style IDs during codegen.
5. **Gate 1 Deliverables & Verification**:
   - Example app: `examples/styling/` utilizing `StyleSheet.create`, `global.css`, a CSS module, dark-mode media query, pressed-state style, and CSS variable theme switch.
   - Assertions:
     - Unchanged style => 0 patches emitted.
     - Theme switch => 1 batched JNI call.
     - 10k-node list resolves within budget recorded in `.agent/learnings/perf-log.md`.
   - Full test suite: Rust unit & integration tests + Android Robolectric tests.

