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

