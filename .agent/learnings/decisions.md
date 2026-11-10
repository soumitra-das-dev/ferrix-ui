# Architecture Decisions (ADRs)

### 2026-11-09 ADR-0001: Three Independent Contracts
- **Context**: The framework requires clear separation of concerns so that compiler, runtime, and platform renderer can evolve or be swapped independently.
- **Options**:
  1. Monolithic crate where compiler emits direct calls to runtime and renderer.
  2. Independent contracts: FSX Language Specification (`docs/fsx-spec.md`), Runtime Component API (`ferrix-core`), and Renderer Protocol (`ferrix-renderer-protocol`).
- **Decision**: Option 2. `ferrix-renderer-protocol` has zero workspace dependencies and models the pure binary/structured wire protocol between Rust and Kotlin.
- **Consequences**: Enables swapping Android Views renderer for Jetpack Compose or desktop without changing compiler or runtime. Ensures protocol can be unit tested in isolation.

### 2026-11-09 ADR-0002: DirectByteBuffer & Packed Binary Encoding for JNI Batches
- **Context**: JNI overhead is dominated by per-call crossing costs and object instantiation. Calling JNI per UI property is explicitly forbidden. Ad-hoc JSON parsing incurs runtime and allocation overhead.
- **Options**:
  1. JSON string over JNI string.
  2. Protobuf / FlatBuffers serialization.
  3. Custom compact binary packed buffer transmitted via `java.nio.ByteBuffer.allocateDirect` / raw memory pointer with batched opcodes and payloads.
- **Decision**: Option 3. Packed binary command buffer (opcode `u8`, target `u64`, length-prefixed values) decoded in Kotlin using zero-copy `ByteBuffer`.
- **Consequences**: Zero JNI overhead per property, zero string serialization overhead, minimal memory allocations, deterministic decoding order.

### 2026-11-09 ADR-0003: Text Measurement Contract
- **Context**: Taffy computes layout geometry ahead of view creation in Rust. Calculating flexbox layout for text nodes requires intrinsic content size `(width, height)` given width constraints.
- **Options**:
  1. *Rust-side text shaping (e.g. cosmic-text, fontdue, rustybuzz)*: Does layout completely within Rust without cross-language calls, but cannot access OEM fonts, dynamic system font scale, Android typography fallback chains, or locale-specific line-breaking without bundling heavy font files and duplicating Android typography logic.
  2. *Synchronous per-node JNI calls to Android `StaticLayout`*: Guarantees 100% Android typography fidelity, but causes unacceptable per-node JNI chatter and overhead.
  3. *Hybrid Batched Native Measure Cache*: Rust maintains an in-memory LRU cache of text measurement results keyed by `(hash(text), font_size_sp, font_weight, font_style, max_width, font_scale)`. Uncached text metrics are collected in a single batch query sent to Kotlin's `StaticLayout` / `TextPaint` engine via JNI before running the Taffy layout pass, or predicted using calibrated font metrics with lazy verification.
- **Decision**: Option 3. Hybrid Batched Native Measure Cache. Zero per-node JNI chatter. Rust caches intrinsic dimensions; whenever text content or typography styling changes, unmeasured text entries are batched together and measured via Kotlin's `StaticLayout`.
- **Consequences**:
  - Perfect native fidelity: exact line breaking, OEM fonts, and user font scaling respected.
  - Zero per-node JNI chatter: measure requests are aggregated into a single batch.
  - Sub-microsecond cached lookups during reactive re-renders when text content is unchanged.


### 2026-11-09 ADR-0004: Style Registry and Interned Style IDs in Renderer Protocol
- **Context**: Serializing identical property sets across thousands of UI nodes (e.g., list items with identical background, padding, fonts, and elevation) multiplies JNI buffer sizes and wastes CPU cycles in decoding.
- **Options**:
  1. *Per-Node Property Inlining*: Send every property on every node in `SetProperty` commands.
  2. *Ad-hoc Class Name Strings*: Send string class names over JNI and parse them Kotlin-side.
  3. *Pre-Authorized Style Registry with Interned Style IDs*: Rust registers static/computed styles once via `Patch::RegisterStyles { styles: [(u32, Vec<Property>)] }`. Nodes bind styles using `Patch::SetStyle { node_id, style_id }` or `Patch::SetStyleList { node_id, style_ids }`.
- **Decision**: Option 3. Additive opcodes `RegisterStyles` (0x0C), `SetStyle` (0x0D), and `SetStyleList` (0x0E). Kotlin maintains a `styleRegistry` map and applies properties directly to views.
- **Consequences**:
  - Dramatic reduction in patch buffer sizes (from $O(\text{nodes} \times \text{props})$ to $O(\text{styles} \times \text{props} + \text{nodes})$).
  - Fine-grained reactive theme changes (e.g. dark mode) update registry entries once, triggering instant view updates.
  - Full backwards compatibility with existing `SetProperty` commands.

### 2026-11-09 ADR-0005: React Native Layout Defaults over Web CSS Defaults in Native Style IR
- **Context**: Web CSS specifies `display: inline`, `flex-direction: row`, `position: static`, and `box-sizing: content-box` by default. Mobile UI frameworks (React Native, Flutter) universally use column flexbox and relative positioning as defaults.
- **Options**:
  1. *Web CSS defaults*: Requires explicit `display: flex; flex-direction: column;` on virtually every container.
  2. *React Native defaults*: `display: flex`, `flex-direction: column`, `position: relative`, `box-sizing: border-box`, `min-width: 0`, `min-height: 0`.
- **Decision**: Option 2. Native Style IR applies React Native layout defaults when properties are unspecified.
- **Consequences**:
  - Declarative developer ergonomics match React Native and mobile native expectations.
  - Clean mapping to Taffy's default layout styles.

### 2026-11-09 ADR-0006: CSS Combinators and Parent-Context Matching Policy
- **Context**: Full CSS descendant and direct-child combinators (`.parent > .child`, `.card .title`) require DOM traversal. In Architecture A, Rust constructs the element tree and resolves styles prior to layout.
- **Options**:
  1. *Reject all combinators*: Only allow simple class selectors `.card_title`.
  2. *Full runtime DOM query engine*: Heavy memory and execution overhead.
  3. *Parent-Context Matching during Element Build*: Direct-child (`>`) and contextual descendant rules are resolved during element tree assembly by matching parent class context stored in `ComponentContext`.
- **Decision**: Option 3. Support direct-child (`>`) and scoped descendant rules via parent class context matching; reject unsupported web pseudo-elements (`::before/::after`) with clear diagnostics.
- **Consequences**:
  - Clean component encapsulation while allowing structured styling patterns (`.card > .header`).
  - No DOM traversal at runtime.

### 2026-11-09 ADR-0007: LightningCSS and Build-Time PostCSS Tooling
- **Context**: CSS normalization, at-rule processing, color conversion (`oklch`, `hsl`), and shorthand expansion must be fast and zero-runtime-cost.
- **Options**:
  1. *Hand-written CSS tokenizer and parser*: Minimal build footprint, but high maintenance for modern CSS syntax.
  2. *LightningCSS Rust crate*: Extremely fast Rust-based CSS parser/transformer, handles nesting, media queries, CSS variables, and minification.
  3. *Node-only PostCSS runner*: Requires Node.js installed on every machine, slowing build times.
- **Decision**: Hybrid approach: LightningCSS is the default Rust-only fast path (no Node required). If `postcss.config.*` exists, `tools/postcss-runner` runs as a build-time subprocess before feeding into the native style pipeline.
- **Consequences**:
  - Zero-runtime overhead in the APK.
  - Zero-dependency fast path for developers without Node.js.
  - Full compatibility with PostCSS and Tailwind plugins for advanced web workflows.
