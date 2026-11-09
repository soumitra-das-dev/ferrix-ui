# JNI & Android Runtime Knowledge

### 2026-11-09 JNI Bridge Lifecycle & DirectByteBuffer
- **Context**: Passing batched command buffers between Rust and Kotlin.
- **What happened / finding**: Using `env.new_direct_byte_buffer` allows zero-copy sharing of native buffers with Java `ByteBuffer`. Kotlin can read opcodes, IDs, and payloads directly without JNI copying.
- **Root cause**: Direct ByteBuffers map directly to unmanaged native memory regions.
- **Rule going forward**: Always use little-endian byte ordering for multi-byte integers (`u64`, `f32`, `u32`) in both Rust writer and Kotlin reader.
- **Evidence**: Android NDK documentation on DirectByteBuffer.
