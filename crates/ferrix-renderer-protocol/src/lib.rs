//! # ferrix-renderer-protocol
//!
//! Typed binary UI patch protocol between the Ferrix Rust runtime and platform renderers.
//! This crate is strictly decoupled and has zero workspace dependencies.

use thiserror::Error;

/// Protocol magic header bytes: "FXUI"
pub const PROTOCOL_MAGIC: [u8; 4] = [0x46, 0x58, 0x55, 0x49];
/// Current protocol version
pub const PROTOCOL_VERSION: u16 = 1;

/// Stable identifier for a UI node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u64);

impl NodeId {
    pub const ROOT: Self = Self(1);

    #[inline]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    #[inline]
    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// Native view types recognized by the platform renderer.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewType {
    Container = 1,
    Column = 2,
    Row = 3,
    Text = 4,
    Button = 5,
    Image = 6,
    TextInput = 7,
    ScrollView = 8,
}

impl ViewType {
    pub fn from_u16(val: u16) -> Option<Self> {
        match val {
            1 => Some(Self::Container),
            2 => Some(Self::Column),
            3 => Some(Self::Row),
            4 => Some(Self::Text),
            5 => Some(Self::Button),
            6 => Some(Self::Image),
            7 => Some(Self::TextInput),
            8 => Some(Self::ScrollView),
            _ => None,
        }
    }
}

/// View visibility state.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    Visible = 0,
    Invisible = 1,
    Gone = 2,
}

impl Visibility {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Visible),
            1 => Some(Self::Invisible),
            2 => Some(Self::Gone),
            _ => None,
        }
    }
}

/// View property keys and values.
#[derive(Clone, Debug, PartialEq)]
pub enum Property {
    BackgroundColor(u32),
    TextColor(u32),
    FontSize(f32),
    Padding {
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    },
    Spacing(f32),
    CornerRadius(f32),
    Enabled(bool),
}

impl Property {
    pub fn prop_id(&self) -> u16 {
        match self {
            Self::BackgroundColor(_) => 1,
            Self::TextColor(_) => 2,
            Self::FontSize(_) => 3,
            Self::Padding { .. } => 4,
            Self::Spacing(_) => 5,
            Self::CornerRadius(_) => 6,
            Self::Enabled(_) => 7,
        }
    }
}

/// Protocol patch opcodes.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    CreateNode = 1,
    AppendChild = 2,
    InsertBefore = 3,
    RemoveChild = 4,
    DestroyNode = 5,
    SetText = 6,
    SetLayout = 7,
    SetProperty = 8,
    SetVisibility = 9,
    AttachEventListener = 10,
    DetachEventListener = 11,
}

impl Opcode {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            1 => Some(Self::CreateNode),
            2 => Some(Self::AppendChild),
            3 => Some(Self::InsertBefore),
            4 => Some(Self::RemoveChild),
            5 => Some(Self::DestroyNode),
            6 => Some(Self::SetText),
            7 => Some(Self::SetLayout),
            8 => Some(Self::SetProperty),
            9 => Some(Self::SetVisibility),
            10 => Some(Self::AttachEventListener),
            11 => Some(Self::DetachEventListener),
            _ => None,
        }
    }
}

/// Event types dispatched from Kotlin to Rust.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventType {
    Click = 1,
    TextChanged = 2,
    FocusChanged = 3,
    Scroll = 4,
}

impl EventType {
    pub fn from_u16(val: u16) -> Option<Self> {
        match val {
            1 => Some(Self::Click),
            2 => Some(Self::TextChanged),
            3 => Some(Self::FocusChanged),
            4 => Some(Self::Scroll),
            _ => None,
        }
    }
}

/// A UI patch instruction emitted by the Rust runtime.
#[derive(Clone, Debug, PartialEq)]
pub enum Patch {
    CreateNode {
        node_id: NodeId,
        view_type: ViewType,
    },
    AppendChild {
        parent_id: NodeId,
        child_id: NodeId,
    },
    InsertBefore {
        parent_id: NodeId,
        child_id: NodeId,
        before_id: NodeId,
    },
    RemoveChild {
        parent_id: NodeId,
        child_id: NodeId,
    },
    DestroyNode {
        node_id: NodeId,
    },
    SetText {
        node_id: NodeId,
        text: String,
    },
    SetLayout {
        node_id: NodeId,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    SetProperty {
        node_id: NodeId,
        property: Property,
    },
    SetVisibility {
        node_id: NodeId,
        visibility: Visibility,
    },
    AttachEventListener {
        node_id: NodeId,
        event_type: EventType,
    },
    DetachEventListener {
        node_id: NodeId,
        event_type: EventType,
    },
}

/// Errors occurring during patch encoding or decoding.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ProtocolError {
    #[error("Invalid magic bytes in buffer header")]
    InvalidMagic,
    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(u16),
    #[error("Unexpected end of buffer at offset {offset}, expected {expected} bytes")]
    UnexpectedEof { offset: usize, expected: usize },
    #[error("Unknown opcode: {0}")]
    UnknownOpcode(u8),
    #[error("Unknown view type: {0}")]
    UnknownViewType(u16),
    #[error("Unknown property ID: {0}")]
    UnknownPropertyId(u16),
    #[error("Unknown visibility value: {0}")]
    UnknownVisibility(u8),
    #[error("Unknown event type: {0}")]
    UnknownEventType(u16),
    #[error("Invalid UTF-8 text string")]
    InvalidUtf8,
}

/// A batch of patches to be applied atomically on the platform UI thread.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PatchBatch {
    pub patches: Vec<Patch>,
}

impl PatchBatch {
    pub fn new() -> Self {
        Self {
            patches: Vec::new(),
        }
    }

    pub fn push(&mut self, patch: Patch) {
        self.patches.push(patch);
    }

    pub fn is_empty(&self) -> bool {
        self.patches.is_empty()
    }

    pub fn len(&self) -> usize {
        self.patches.len()
    }

    /// Serializes the batch into a compact binary buffer with header.
    pub fn encode(&self) -> Vec<u8> {
        let mut payload = Vec::new();

        for patch in &self.patches {
            match patch {
                Patch::CreateNode { node_id, view_type } => {
                    payload.push(Opcode::CreateNode as u8);
                    payload.extend_from_slice(&node_id.raw().to_le_bytes());
                    payload.extend_from_slice(&(*view_type as u16).to_le_bytes());
                }
                Patch::AppendChild {
                    parent_id,
                    child_id,
                } => {
                    payload.push(Opcode::AppendChild as u8);
                    payload.extend_from_slice(&parent_id.raw().to_le_bytes());
                    payload.extend_from_slice(&child_id.raw().to_le_bytes());
                }
                Patch::InsertBefore {
                    parent_id,
                    child_id,
                    before_id,
                } => {
                    payload.push(Opcode::InsertBefore as u8);
                    payload.extend_from_slice(&parent_id.raw().to_le_bytes());
                    payload.extend_from_slice(&child_id.raw().to_le_bytes());
                    payload.extend_from_slice(&before_id.raw().to_le_bytes());
                }
                Patch::RemoveChild {
                    parent_id,
                    child_id,
                } => {
                    payload.push(Opcode::RemoveChild as u8);
                    payload.extend_from_slice(&parent_id.raw().to_le_bytes());
                    payload.extend_from_slice(&child_id.raw().to_le_bytes());
                }
                Patch::DestroyNode { node_id } => {
                    payload.push(Opcode::DestroyNode as u8);
                    payload.extend_from_slice(&node_id.raw().to_le_bytes());
                }
                Patch::SetText { node_id, text } => {
                    payload.push(Opcode::SetText as u8);
                    payload.extend_from_slice(&node_id.raw().to_le_bytes());
                    let bytes = text.as_bytes();
                    payload.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                    payload.extend_from_slice(bytes);
                }
                Patch::SetLayout {
                    node_id,
                    x,
                    y,
                    width,
                    height,
                } => {
                    payload.push(Opcode::SetLayout as u8);
                    payload.extend_from_slice(&node_id.raw().to_le_bytes());
                    payload.extend_from_slice(&x.to_le_bytes());
                    payload.extend_from_slice(&y.to_le_bytes());
                    payload.extend_from_slice(&width.to_le_bytes());
                    payload.extend_from_slice(&height.to_le_bytes());
                }
                Patch::SetProperty { node_id, property } => {
                    payload.push(Opcode::SetProperty as u8);
                    payload.extend_from_slice(&node_id.raw().to_le_bytes());
                    payload.extend_from_slice(&property.prop_id().to_le_bytes());
                    match property {
                        Property::BackgroundColor(c) | Property::TextColor(c) => {
                            payload.extend_from_slice(&(4u16).to_le_bytes());
                            payload.extend_from_slice(&c.to_le_bytes());
                        }
                        Property::FontSize(s)
                        | Property::Spacing(s)
                        | Property::CornerRadius(s) => {
                            payload.extend_from_slice(&(4u16).to_le_bytes());
                            payload.extend_from_slice(&s.to_le_bytes());
                        }
                        Property::Padding {
                            left,
                            top,
                            right,
                            bottom,
                        } => {
                            payload.extend_from_slice(&(16u16).to_le_bytes());
                            payload.extend_from_slice(&left.to_le_bytes());
                            payload.extend_from_slice(&top.to_le_bytes());
                            payload.extend_from_slice(&right.to_le_bytes());
                            payload.extend_from_slice(&bottom.to_le_bytes());
                        }
                        Property::Enabled(en) => {
                            payload.extend_from_slice(&(1u16).to_le_bytes());
                            payload.push(if *en { 1 } else { 0 });
                        }
                    }
                }
                Patch::SetVisibility {
                    node_id,
                    visibility,
                } => {
                    payload.push(Opcode::SetVisibility as u8);
                    payload.extend_from_slice(&node_id.raw().to_le_bytes());
                    payload.push(*visibility as u8);
                }
                Patch::AttachEventListener {
                    node_id,
                    event_type,
                } => {
                    payload.push(Opcode::AttachEventListener as u8);
                    payload.extend_from_slice(&node_id.raw().to_le_bytes());
                    payload.extend_from_slice(&(*event_type as u16).to_le_bytes());
                }
                Patch::DetachEventListener {
                    node_id,
                    event_type,
                } => {
                    payload.push(Opcode::DetachEventListener as u8);
                    payload.extend_from_slice(&node_id.raw().to_le_bytes());
                    payload.extend_from_slice(&(*event_type as u16).to_le_bytes());
                }
            }
        }

        let mut buffer = Vec::with_capacity(16 + payload.len());
        buffer.extend_from_slice(&PROTOCOL_MAGIC);
        buffer.extend_from_slice(&PROTOCOL_VERSION.to_le_bytes());
        buffer.extend_from_slice(&(0u16).to_le_bytes()); // flags
        buffer.extend_from_slice(&(self.patches.len() as u32).to_le_bytes());
        buffer.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        buffer.extend_from_slice(&payload);
        buffer
    }

    /// Deserializes a binary patch buffer back into a `PatchBatch`.
    pub fn decode(bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() < 16 {
            return Err(ProtocolError::UnexpectedEof {
                offset: bytes.len(),
                expected: 16,
            });
        }

        if bytes[0..4] != PROTOCOL_MAGIC {
            return Err(ProtocolError::InvalidMagic);
        }

        let version = u16::from_le_bytes([bytes[4], bytes[5]]);
        if version != PROTOCOL_VERSION {
            return Err(ProtocolError::UnsupportedVersion(version));
        }

        let command_count = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]) as usize;
        let payload_len = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]) as usize;

        if bytes.len() < 16 + payload_len {
            return Err(ProtocolError::UnexpectedEof {
                offset: bytes.len(),
                expected: 16 + payload_len,
            });
        }

        let mut offset = 16;
        let mut patches = Vec::with_capacity(command_count);

        for _ in 0..command_count {
            if offset >= bytes.len() {
                return Err(ProtocolError::UnexpectedEof {
                    offset,
                    expected: offset + 1,
                });
            }
            let opcode_byte = bytes[offset];
            offset += 1;

            let opcode =
                Opcode::from_u8(opcode_byte).ok_or(ProtocolError::UnknownOpcode(opcode_byte))?;

            match opcode {
                Opcode::CreateNode => {
                    if offset + 10 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 10,
                        });
                    }
                    let node_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let vt_raw = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
                    offset += 2;
                    let view_type =
                        ViewType::from_u16(vt_raw).ok_or(ProtocolError::UnknownViewType(vt_raw))?;
                    patches.push(Patch::CreateNode { node_id, view_type });
                }
                Opcode::AppendChild => {
                    if offset + 16 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 16,
                        });
                    }
                    let parent_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let child_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    patches.push(Patch::AppendChild {
                        parent_id,
                        child_id,
                    });
                }
                Opcode::InsertBefore => {
                    if offset + 24 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 24,
                        });
                    }
                    let parent_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let child_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let before_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    patches.push(Patch::InsertBefore {
                        parent_id,
                        child_id,
                        before_id,
                    });
                }
                Opcode::RemoveChild => {
                    if offset + 16 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 16,
                        });
                    }
                    let parent_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let child_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    patches.push(Patch::RemoveChild {
                        parent_id,
                        child_id,
                    });
                }
                Opcode::DestroyNode => {
                    if offset + 8 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 8,
                        });
                    }
                    let node_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    patches.push(Patch::DestroyNode { node_id });
                }
                Opcode::SetText => {
                    if offset + 12 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 12,
                        });
                    }
                    let node_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let text_len =
                        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
                    offset += 4;
                    if offset + text_len > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + text_len,
                        });
                    }
                    let text = std::str::from_utf8(&bytes[offset..offset + text_len])
                        .map_err(|_| ProtocolError::InvalidUtf8)?
                        .to_string();
                    offset += text_len;
                    patches.push(Patch::SetText { node_id, text });
                }
                Opcode::SetLayout => {
                    if offset + 24 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 24,
                        });
                    }
                    let node_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let x = f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
                    offset += 4;
                    let y = f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
                    offset += 4;
                    let width = f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
                    offset += 4;
                    let height = f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
                    offset += 4;
                    patches.push(Patch::SetLayout {
                        node_id,
                        x,
                        y,
                        width,
                        height,
                    });
                }
                Opcode::SetProperty => {
                    if offset + 12 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 12,
                        });
                    }
                    let node_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let prop_id = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
                    offset += 2;
                    let val_len =
                        u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as usize;
                    offset += 2;
                    if offset + val_len > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + val_len,
                        });
                    }
                    let prop_slice = &bytes[offset..offset + val_len];
                    offset += val_len;

                    let property = match prop_id {
                        1 => Property::BackgroundColor(u32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        2 => {
                            Property::TextColor(u32::from_le_bytes(prop_slice.try_into().map_err(
                                |_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                },
                            )?))
                        }
                        3 => {
                            Property::FontSize(f32::from_le_bytes(prop_slice.try_into().map_err(
                                |_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                },
                            )?))
                        }
                        4 => {
                            if prop_slice.len() < 16 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 16,
                                });
                            }
                            let left = f32::from_le_bytes(prop_slice[0..4].try_into().unwrap());
                            let top = f32::from_le_bytes(prop_slice[4..8].try_into().unwrap());
                            let right = f32::from_le_bytes(prop_slice[8..12].try_into().unwrap());
                            let bottom = f32::from_le_bytes(prop_slice[12..16].try_into().unwrap());
                            Property::Padding {
                                left,
                                top,
                                right,
                                bottom,
                            }
                        }
                        5 => Property::Spacing(f32::from_le_bytes(prop_slice.try_into().map_err(
                            |_| ProtocolError::UnexpectedEof {
                                offset,
                                expected: 4,
                            },
                        )?)),
                        6 => Property::CornerRadius(f32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        7 => Property::Enabled(prop_slice.first().copied().unwrap_or(0) != 0),
                        _ => return Err(ProtocolError::UnknownPropertyId(prop_id)),
                    };
                    patches.push(Patch::SetProperty { node_id, property });
                }
                Opcode::SetVisibility => {
                    if offset + 9 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 9,
                        });
                    }
                    let node_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let vis_byte = bytes[offset];
                    offset += 1;
                    let visibility = Visibility::from_u8(vis_byte)
                        .ok_or(ProtocolError::UnknownVisibility(vis_byte))?;
                    patches.push(Patch::SetVisibility {
                        node_id,
                        visibility,
                    });
                }
                Opcode::AttachEventListener => {
                    if offset + 10 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 10,
                        });
                    }
                    let node_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let et_raw = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
                    offset += 2;
                    let event_type = EventType::from_u16(et_raw)
                        .ok_or(ProtocolError::UnknownEventType(et_raw))?;
                    patches.push(Patch::AttachEventListener {
                        node_id,
                        event_type,
                    });
                }
                Opcode::DetachEventListener => {
                    if offset + 10 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 10,
                        });
                    }
                    let node_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let et_raw = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
                    offset += 2;
                    let event_type = EventType::from_u16(et_raw)
                        .ok_or(ProtocolError::UnknownEventType(et_raw))?;
                    patches.push(Patch::DetachEventListener {
                        node_id,
                        event_type,
                    });
                }
            }
        }

        Ok(PatchBatch { patches })
    }
}

/// An event delivered from Kotlin to Rust.
#[derive(Clone, Debug, PartialEq)]
pub struct UiEvent {
    pub node_id: NodeId,
    pub event_type: EventType,
    pub payload: Vec<u8>,
}

impl UiEvent {
    pub fn new(node_id: NodeId, event_type: EventType) -> Self {
        Self {
            node_id,
            event_type,
            payload: Vec::new(),
        }
    }

    pub fn with_payload(node_id: NodeId, event_type: EventType, payload: Vec<u8>) -> Self {
        Self {
            node_id,
            event_type,
            payload,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(14 + self.payload.len());
        buf.extend_from_slice(&self.node_id.raw().to_le_bytes());
        buf.extend_from_slice(&(self.event_type as u16).to_le_bytes());
        buf.extend_from_slice(&(self.payload.len() as u32).to_le_bytes());
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() < 14 {
            return Err(ProtocolError::UnexpectedEof {
                offset: bytes.len(),
                expected: 14,
            });
        }
        let node_id = NodeId(u64::from_le_bytes(bytes[0..8].try_into().unwrap()));
        let et_raw = u16::from_le_bytes(bytes[8..10].try_into().unwrap());
        let event_type =
            EventType::from_u16(et_raw).ok_or(ProtocolError::UnknownEventType(et_raw))?;
        let payload_len = u32::from_le_bytes(bytes[10..14].try_into().unwrap()) as usize;
        if bytes.len() < 14 + payload_len {
            return Err(ProtocolError::UnexpectedEof {
                offset: bytes.len(),
                expected: 14 + payload_len,
            });
        }
        let payload = bytes[14..14 + payload_len].to_vec();
        Ok(Self {
            node_id,
            event_type,
            payload,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_round_trip() {
        let mut batch = PatchBatch::new();
        batch.push(Patch::CreateNode {
            node_id: NodeId(10),
            view_type: ViewType::Text,
        });
        batch.push(Patch::SetText {
            node_id: NodeId(10),
            text: "Hello Ferrix!".to_string(),
        });
        batch.push(Patch::SetLayout {
            node_id: NodeId(10),
            x: 10.0,
            y: 20.0,
            width: 200.0,
            height: 48.0,
        });
        batch.push(Patch::SetProperty {
            node_id: NodeId(10),
            property: Property::FontSize(18.0),
        });
        batch.push(Patch::SetVisibility {
            node_id: NodeId(10),
            visibility: Visibility::Visible,
        });
        batch.push(Patch::AttachEventListener {
            node_id: NodeId(10),
            event_type: EventType::Click,
        });

        let encoded = batch.encode();
        assert!(encoded.starts_with(&PROTOCOL_MAGIC));

        let decoded = PatchBatch::decode(&encoded).expect("failed to decode valid batch");
        assert_eq!(batch, decoded);
    }

    #[test]
    fn test_event_round_trip() {
        let event =
            UiEvent::with_payload(NodeId(42), EventType::TextChanged, b"new value".to_vec());
        let encoded = event.encode();
        let decoded = UiEvent::decode(&encoded).expect("failed to decode valid event");
        assert_eq!(event, decoded);
    }
}
