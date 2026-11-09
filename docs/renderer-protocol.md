# Ferrix Renderer Protocol Specification

## 1. Scope & Isolation
The **Renderer Protocol** defines the exact binary wire contract between the Ferrix Rust runtime and platform renderers (such as the Android Kotlin runtime).

**Strict Isolation Rule**:
The `ferrix-renderer-protocol` crate depends on **no other crate** in the workspace. It contains only pure data structures, serialization/deserialization logic, and binary buffer definitions. Both Rust and Kotlin must adhere strictly to this specification.

---

## 2. Wire Protocol Architecture

### 2.1 Transport Mechanism
- Patches are grouped and serialized into a **Batched Command Buffer**.
- Transport across JNI utilizes `java.nio.ByteBuffer.allocateDirect` (DirectByteBuffer) with explicit little-endian byte ordering (`LITTLE_ENDIAN`).
- Communication is batched: Rust accumulates dirty node mutations during a frame/tick and transfers a single buffer to Kotlin for execution on the Android UI Main Thread.
- Kotlin sends events (e.g., user clicks, gestures, lifecycle callbacks) back to Rust using an equivalent event buffer or compact event struct.

### 2.2 Buffer Header Format
Every patch buffer begins with a fixed 16-byte header:

| Offset | Type | Field | Description |
|---|---|---|---|
| 0..4 | `[u8; 4]` | `magic` | Magic header bytes: `FXUI` (`0x46, 0x58, 0x55, 0x49`) |
| 4..6 | `u16` | `version` | Protocol version (currently `1`) |
| 6..8 | `u16` | `flags` | Bitflags (e.g., `0x01` = includes layout pass) |
| 8..12 | `u32` | `command_count` | Number of patch commands serialized in the payload |
| 12..16 | `u32` | `payload_len` | Total size of command payload in bytes |

### 2.3 Stable Node Identifiers
- Every virtual and native element is identified by a stable `u64` **Node ID**.
- The root view container has ID `1`.
- Node IDs are allocated monotonically by the Rust runtime.
- Kotlin maintains a lookup map `LongSparseArray<View>` or `HashMap<Long, View>` mapping `NodeId` to the native Android `View`.

---

## 3. Patch Command Opcodes & Payloads

Commands are serialized sequentially immediately following the header. Each command starts with a 1-byte opcode (`u8`).

### 0x01: `CreateNode`
Instructs the platform renderer to instantiate a native view.
- `opcode`: `0x01` (`u8`)
- `node_id`: `u64`
- `view_type`: `u16`
  - `0x0001`: `Container` (FrameLayout / ViewGroup)
  - `0x0002`: `Column` (Vertical LinearLayout / Custom FerrixLayout)
  - `0x0003`: `Row` (Horizontal LinearLayout / Custom FerrixLayout)
  - `0x0004`: `Text` (`TextView`)
  - `0x0005`: `Button` (`MaterialButton` / `Button`)
  - `0x0006`: `Image` (`ImageView`)
  - `0x0007`: `TextInput` (`EditText`)
  - `0x0008`: `ScrollView` (`NestedScrollView`)

### 0x02: `AppendChild`
Appends a child node to a parent node.
- `opcode`: `0x02` (`u8`)
- `parent_id`: `u64`
- `child_id`: `u64`

### 0x03: `InsertBefore`
Inserts a child node before an existing sibling.
- `opcode`: `0x03` (`u8`)
- `parent_id`: `u64`
- `child_id`: `u64`
- `before_id`: `u64`

### 0x04: `RemoveChild`
Detaches a child node from its parent.
- `opcode`: `0x04` (`u8`)
- `parent_id`: `u64`
- `child_id`: `u64`

### 0x05: `DestroyNode`
Deallocates the node mapping and cleans up any native resources.
- `opcode`: `0x05` (`u8`)
- `node_id`: `u64`

### 0x06: `SetText`
Updates the text content of a text-bearing node (`TextView`, `Button`).
- `opcode`: `0x06` (`u8`)
- `node_id`: `u64`
- `text_len`: `u32`
- `text_bytes`: `[u8; text_len]` (valid UTF-8)

### 0x07: `SetLayout`
Applies computed layout geometry from Taffy directly to the native View.
- `opcode`: `0x07` (`u8`)
- `node_id`: `u64`
- `x`: `f32` (left position in physical/scaled pixels)
- `y`: `f32` (top position in physical/scaled pixels)
- `width`: `f32` (width in physical/scaled pixels)
- `height`: `f32` (height in physical/scaled pixels)

### 0x08: `SetProperty`
Configures a typed style or behavior attribute on the target node.
- `opcode`: `0x08` (`u8`)
- `node_id`: `u64`
- `prop_id`: `u16`
  - `0x0001`: `BackgroundColor` (ARGB `u32`)
  - `0x0002`: `TextColor` (ARGB `u32`)
  - `0x0003`: `FontSize` (`f32` sp)
  - `0x0004`: `Padding` (`left: f32, top: f32, right: f32, bottom: f32`)
  - `0x0005`: `Spacing` (`f32` dp)
  - `0x0006`: `CornerRadius` (`f32` dp)
  - `0x0007`: `Enabled` (`u8`: 0 = false, 1 = true)
- `val_len`: `u16`
- `val_bytes`: `[u8; val_len]`

### 0x09: `SetVisibility`
Controls the Android View visibility.
- `opcode`: `0x09` (`u8`)
- `node_id`: `u64`
- `visibility`: `u8` (0 = `VISIBLE`, 1 = `INVISIBLE`, 2 = `GONE`)

### 0x0A: `AttachEventListener`
Instructs Kotlin to attach an event listener to the View.
- `opcode`: `0x0A` (`u8`)
- `node_id`: `u64`
- `event_type`: `u16` (1 = `Click`, 2 = `TextChanged`, 3 = `FocusChange`, 4 = `Scroll`)

### 0x0B: `DetachEventListener`
Removes an attached event listener.
- `opcode`: `0x0B` (`u8`)
- `node_id`: `u64`
- `event_type`: `u16`

### 0x0C: `RegisterStyles` (Style Registry)
Registers a batch of interned styles in the platform renderer style registry. Nodes reference these interned style IDs instead of re-transmitting repeated property sets.
- `opcode`: `0x0C` (`u8`)
- `style_count`: `u32`
- Followed by `style_count` entries:
  - `style_id`: `u32` (stable interned identifier)
  - `prop_count`: `u16`
  - Followed by `prop_count` serialized `Property` records (encoded with `prop_id` + value bytes).

### 0x0D: `SetStyle`
Applies an interned style from the style registry directly to a node.
- `opcode`: `0x0D` (`u8`)
- `node_id`: `u64`
- `style_id`: `u32`

### 0x0E: `SetStyleList`
Applies an ordered cascade list of interned style IDs to a node (merging properties from first to last).
- `opcode`: `0x0E` (`u8`)
- `node_id`: `u64`
- `style_count`: `u16`
- `style_ids`: `[u32; style_count]`

---

## 4. Event Protocol (Kotlin → Rust)

Events generated by user interaction are queued in Kotlin and delivered to Rust:

### Event Header & Item Structure
- `node_id`: `u64`
- `event_type`: `u16`
- `payload_len`: `u32`
- `payload_bytes`: `[u8; payload_len]`

### Common Event Types:
- `0x0001` `OnClick`: Empty payload or optional touch coordinate `(f32, f32)`.
- `0x0002` `OnTextChanged`: Length-prefixed UTF-8 string containing the new text.
- `0x0003` `OnFocusChanged`: 1 byte (`0` or `1`).

---

## 5. Protocol Verification & Error Handling
1. **Magic Header Validation**: Every incoming buffer must match `FXUI` or be rejected immediately with an error.
2. **Buffer Bounds Checking**: Decoders in both Rust and Kotlin verify offsets before reading; unexpected EOF raises a malformed buffer exception.
3. **Atomic Frame Application**: Patches within a single buffer must be applied as a coherent transaction on the Android Main Thread.
