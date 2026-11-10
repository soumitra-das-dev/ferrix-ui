# Styling Pipeline Learnings

## Architecture & Boundaries
- **Architecture A Invariant**:
  - Rust runtime owns style resolution, CSS cascade, specificity ranking, variant resolution (`:active`, dark mode, breakpoints), CSS variables substitution, and Taffy layout mapping.
  - Kotlin runtime owns applying interned packed properties to native Android Views via `styleRegistry: HashMap<Int, List<Property>>`.
  - Zero web engines, zero browser components, zero Node.js runtime, zero CSS parsing in the production APK.

## Style Registry Protocol (`ferrix-renderer-protocol`)
- Three additive opcodes added to the binary protocol:
  - `Opcode::RegisterStyles = 12 (0x0C)`: Batched array of `(style_id: u32, properties: Vec<Property>)`.
  - `Opcode::SetStyle = 13 (0x0D)`: `(node_id: NodeId, style_id: u32)`.
  - `Opcode::SetStyleList = 14 (0x0E)`: `(node_id: NodeId, style_ids: Vec<u32>)`.
- Nodes reference interned integer style IDs instead of repeating property bytes over JNI.
- Kotlin's `FerrixRenderer` maintains a `styleRegistry` map and applies properties directly to views when `SetStyle` is dispatched.

## CSS Compilation & Native Style IR (`ferrix-style`, `ferrix-style-build`)
- **Native-compatible CSS subset**:
  - Layout & Flexbox (React Native defaults: `flex-col`, `relative`, `flex`).
  - Shorthand expansions: `margin`, `padding`, `border`, `border-radius`, `gap`, `flex`.
  - Colors: Hex (3, 4, 6, 8 digit), `rgb()`, `rgba()`, `hsl()`, `oklch()`, and `color-mix(in srgb, ...)`.
  - Unsupported web-only properties (e.g. `float`, `cursor`, `columns`, `table`) and pseudo-elements (`::before`, `::after`) are caught at build-time with clean file:line:col diagnostics.
- **Cascade & Specificity**:
  - Pre-parsed selectors (`ParsedSelector::Universal`, `ParsedSelector::Tag`, `ParsedSelector::Classes`) eliminate string allocations during matching.
  - Strict merge precedence:
    $$\text{Universal (*)} < \text{Global CSS} < \text{CSS Modules (layer, specificity, source order)} < \text{Inline style prop}$$
- **Zero-Patch Interaction Filter**:
  - `ReactiveStyleResolver` tracks applied style IDs per node.
  - Re-evaluating state when visual output doesn't change emits `None` (0 JNI patches).
  - Pressing/releasing `:active` elements emits exactly 1 `SetStyle` patch.
  - Global theme switch emits exactly 1 batched JNI dispatch for all affected nodes.

## Performance
- 10,000-node resolution and interning completed in **45.91 ms** in release mode (**4.59 µs per node**).
- Zero heap allocations in `ir.rs` hashing via `std::mem::discriminant`.

## Phase 2: PostCSS Pipeline & Hot Style Reloading
- **Dual-Path Strategy**:
  - Fast-path (default): Sub-millisecond direct native compilation (`0.166 ms`) when no `postcss.config.*` is present.
  - PostCSS-path (opt-in): When `postcss.config.*` exists, runs `tools/postcss-runner/runner.js` subprocess at build time only.
- **`@ferrix/postcss-plugin`**:
  - Normalizes `rem` / `em` units to density-independent `px` points (`1rem = 16px`).
  - Converts unsupported `position: fixed` to `position: absolute`.
  - Warns on unsupported properties (`float`, `cursor`, `columns`, `table`) and pseudo-elements (`::before`, `::after`).
- **SourceMap v3 Line Mapping**:
  - Full Base64-VLQ decoder mapping generated CSS positions back to original source files and lines.
  - Error diagnostics point developers directly to their authoring `.fsx` or `.css` source line.
- **Hot Style Reloading**:
  - `compute_hot_reload_patches` diffs mounted node styles against recompiled cascade rules.
  - Generates atomic `PatchBatch` (`RegisterStyles` followed by `SetStyle` for affected nodes).
  - Preserves 100% of application runtime state (reactive signals, text input, navigation, scroll position).
  - Benchmark: 50 mounted nodes updated in **1.649 ms** (< 50ms budget).

