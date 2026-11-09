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
