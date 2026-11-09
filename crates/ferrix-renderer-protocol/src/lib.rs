//! # ferrix-renderer-protocol
//!
//! Typed binary UI patch protocol between the Ferrix Rust runtime and platform renderers.
//! This crate is strictly decoupled and has zero workspace dependencies.

pub mod platform;

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
    Pressable = 9,
    Fragment = 10,
    Switch = 11,
    ActivityIndicator = 12,
    Modal = 13,
    RecyclerView = 14,
    SafeAreaView = 15,
    RefreshControl = 16,
    Portal = 17,
    Stack = 18,
    Spacer = 19,
    Divider = 20,
    Center = 21,
    Grid = 22,
    Checkbox = 23,
    RadioGroup = 24,
    Slider = 25,
    Picker = 26,
    ProgressBar = 27,
    Badge = 28,
    LinearGradient = 29,
    Icon = 30,
    NavContainer = 31,
    Screen = 32,
    Header = 33,
    TabBar = 34,
    Drawer = 35,
}

impl ViewType {
    /// Alias for Container (standard flex/box ViewGroup).
    pub const VIEW: Self = Self::Container;
    /// Aliases for virtualized list
    pub const LAZY_LIST: Self = Self::RecyclerView;
    pub const FLAT_LIST: Self = Self::RecyclerView;
    /// Aliases for Stack
    pub const Z_STACK: Self = Self::Stack;

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
            9 => Some(Self::Pressable),
            10 => Some(Self::Fragment),
            11 => Some(Self::Switch),
            12 => Some(Self::ActivityIndicator),
            13 => Some(Self::Modal),
            14 => Some(Self::RecyclerView),
            15 => Some(Self::SafeAreaView),
            16 => Some(Self::RefreshControl),
            17 => Some(Self::Portal),
            18 => Some(Self::Stack),
            19 => Some(Self::Spacer),
            20 => Some(Self::Divider),
            21 => Some(Self::Center),
            22 => Some(Self::Grid),
            23 => Some(Self::Checkbox),
            24 => Some(Self::RadioGroup),
            25 => Some(Self::Slider),
            26 => Some(Self::Picker),
            27 => Some(Self::ProgressBar),
            28 => Some(Self::Badge),
            29 => Some(Self::LinearGradient),
            30 => Some(Self::Icon),
            31 => Some(Self::NavContainer),
            32 => Some(Self::Screen),
            33 => Some(Self::Header),
            34 => Some(Self::TabBar),
            35 => Some(Self::Drawer),
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
    ImageUrl(String),
    AnimateAlpha {
        target: f32,
        duration_ms: u32,
    },
    AnimateScale {
        target: f32,
        duration_ms: u32,
    },
    AnimateTranslationY {
        target: f32,
        duration_ms: u32,
    },
    ContentDescription(String),
    AccessibilityRole(String),
    AccessibilityHeading(bool),
    Hint(String),
    // Wave 0 Extensions: Visual & Borders
    Opacity(f32),
    BorderWidth {
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    },
    BorderColor(u32),
    BorderRadius {
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    },
    Elevation(f32),
    Transform {
        translate_x: f32,
        translate_y: f32,
        scale_x: f32,
        scale_y: f32,
        rotate_deg: f32,
    },
    // Typography
    FontWeight(u16),
    FontStyle(u8),
    LineHeight(f32),
    LetterSpacing(f32),
    TextAlign(u8),
    TextDecoration(u8),
    NumberOfLines(u32),
    // Accessibility & Testing
    TestId(String),
    AccessibilityLabel(String),
    AccessibilityHint(String),
    AccessibilityState {
        disabled: bool,
        selected: bool,
        checked: u8,
        busy: bool,
        expanded: u8,
    },
    ImportantForAccessibility(u8),
    // Interactive & Gestures
    RippleColor(u32),
    HitSlop {
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    },
    // Wave 1 Extensions: Controlled Text & Inputs
    ControlledText {
        version: u32,
        text: String,
    },
    Placeholder(String),
    PlaceholderTextColor(u32),
    SecureTextEntry(bool),
    KeyboardType(u8),
    ReturnKeyType(u8),
    MaxLength(u32),
    Multiline(bool),
    AutoCapitalize(u8),
    AutoCorrect(bool),
    Editable(bool),
    Selection {
        start: u32,
        end: u32,
    },
    // Image
    ImageResizeMode(u8),
    BlurRadius(f32),
    TintColor(u32),
    // ScrollView & Lists
    ScrollHorizontal(bool),
    PagingEnabled(bool),
    ShowsScrollIndicator(bool),
    ContentOffset {
        x: f32,
        y: f32,
    },
    ItemCount(u32),
    ItemHeight(f32),
    // Switch
    SwitchValue(bool),
    TrackColor {
        active: u32,
        inactive: u32,
    },
    ThumbColor(u32),
    // ActivityIndicator
    IndicatorColor(u32),
    IndicatorSize(u8),
    Animating(bool),
    // Modal
    ModalVisible(bool),
    ModalAnimationType(u8),
    ModalTransparent(bool),
    // RefreshControl
    Refreshing(bool),
    ProgressBackgroundColor(u32),
    // Wave 2 Extensions: Inputs, Overlays, Gradients, Media
    Checked(bool),
    SliderValue(f32),
    SliderRange {
        min: f32,
        max: f32,
        step: f32,
    },
    Progress(f32),
    GradientColors {
        start_color: u32,
        end_color: u32,
        angle_deg: f32,
    },
    IconName(String),
    IconSize(f32),
    BadgeContent(String),
    GridColumns(u32),
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
            Self::ImageUrl(_) => 8,
            Self::AnimateAlpha { .. } => 9,
            Self::AnimateScale { .. } => 10,
            Self::AnimateTranslationY { .. } => 11,
            Self::ContentDescription(_) => 12,
            Self::AccessibilityRole(_) => 13,
            Self::AccessibilityHeading(_) => 14,
            Self::Hint(_) => 15,
            Self::Opacity(_) => 16,
            Self::BorderWidth { .. } => 17,
            Self::BorderColor(_) => 18,
            Self::BorderRadius { .. } => 19,
            Self::Elevation(_) => 20,
            Self::Transform { .. } => 21,
            Self::FontWeight(_) => 22,
            Self::FontStyle(_) => 23,
            Self::LineHeight(_) => 24,
            Self::LetterSpacing(_) => 25,
            Self::TextAlign(_) => 26,
            Self::TextDecoration(_) => 27,
            Self::NumberOfLines(_) => 28,
            Self::TestId(_) => 29,
            Self::AccessibilityLabel(_) => 30,
            Self::AccessibilityHint(_) => 31,
            Self::AccessibilityState { .. } => 32,
            Self::ImportantForAccessibility(_) => 33,
            Self::RippleColor(_) => 34,
            Self::HitSlop { .. } => 35,
            Self::ControlledText { .. } => 36,
            Self::Placeholder(_) => 37,
            Self::PlaceholderTextColor(_) => 38,
            Self::SecureTextEntry(_) => 39,
            Self::KeyboardType(_) => 40,
            Self::ReturnKeyType(_) => 41,
            Self::MaxLength(_) => 42,
            Self::Multiline(_) => 43,
            Self::AutoCapitalize(_) => 44,
            Self::AutoCorrect(_) => 45,
            Self::Editable(_) => 46,
            Self::Selection { .. } => 47,
            Self::ImageResizeMode(_) => 48,
            Self::BlurRadius(_) => 49,
            Self::TintColor(_) => 50,
            Self::ScrollHorizontal(_) => 51,
            Self::PagingEnabled(_) => 52,
            Self::ShowsScrollIndicator(_) => 53,
            Self::ContentOffset { .. } => 54,
            Self::ItemCount(_) => 55,
            Self::ItemHeight(_) => 56,
            Self::SwitchValue(_) => 57,
            Self::TrackColor { .. } => 58,
            Self::ThumbColor(_) => 59,
            Self::IndicatorColor(_) => 60,
            Self::IndicatorSize(_) => 61,
            Self::Animating(_) => 62,
            Self::ModalVisible(_) => 63,
            Self::ModalAnimationType(_) => 64,
            Self::ModalTransparent(_) => 65,
            Self::Refreshing(_) => 66,
            Self::ProgressBackgroundColor(_) => 67,
            Self::Checked(_) => 68,
            Self::SliderValue(_) => 69,
            Self::SliderRange { .. } => 70,
            Self::Progress(_) => 71,
            Self::GradientColors { .. } => 72,
            Self::IconName(_) => 73,
            Self::IconSize(_) => 74,
            Self::BadgeContent(_) => 75,
            Self::GridColumns(_) => 76,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventType {
    Click = 1,
    TextChanged = 2,
    FocusChanged = 3,
    Scroll = 4,
    PressIn = 5,
    PressOut = 6,
    LongPress = 7,
    Layout = 8,
    EndReached = 9,
    ValueChange = 10,
    SubmitEditing = 11,
    SelectionChange = 12,
    RequestClose = 13,
    Refresh = 14,
    SliderChange = 15,
}

impl EventType {
    pub fn from_u16(val: u16) -> Option<Self> {
        match val {
            1 => Some(Self::Click),
            2 => Some(Self::TextChanged),
            3 => Some(Self::FocusChanged),
            4 => Some(Self::Scroll),
            5 => Some(Self::PressIn),
            6 => Some(Self::PressOut),
            7 => Some(Self::LongPress),
            8 => Some(Self::Layout),
            9 => Some(Self::EndReached),
            10 => Some(Self::ValueChange),
            11 => Some(Self::SubmitEditing),
            12 => Some(Self::SelectionChange),
            13 => Some(Self::RequestClose),
            14 => Some(Self::Refresh),
            15 => Some(Self::SliderChange),
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
    #[error("Unknown platform message tag: {0}")]
    UnknownPlatformTag(u16),
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
                        Property::ImageUrl(url) => {
                            let bytes = url.as_bytes();
                            payload.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                            payload.extend_from_slice(bytes);
                        }
                        Property::AnimateAlpha {
                            target,
                            duration_ms,
                        }
                        | Property::AnimateScale {
                            target,
                            duration_ms,
                        }
                        | Property::AnimateTranslationY {
                            target,
                            duration_ms,
                        } => {
                            payload.extend_from_slice(&(8u16).to_le_bytes());
                            payload.extend_from_slice(&target.to_le_bytes());
                            payload.extend_from_slice(&duration_ms.to_le_bytes());
                        }
                        Property::ContentDescription(desc)
                        | Property::AccessibilityRole(desc)
                        | Property::Hint(desc) => {
                            let bytes = desc.as_bytes();
                            payload.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                            payload.extend_from_slice(bytes);
                        }
                        Property::AccessibilityHeading(h) => {
                            payload.extend_from_slice(&(1u16).to_le_bytes());
                            payload.push(if *h { 1 } else { 0 });
                        }
                        Property::Opacity(op)
                        | Property::Elevation(op)
                        | Property::LineHeight(op)
                        | Property::LetterSpacing(op) => {
                            payload.extend_from_slice(&(4u16).to_le_bytes());
                            payload.extend_from_slice(&op.to_le_bytes());
                        }
                        Property::BorderWidth {
                            left,
                            top,
                            right,
                            bottom,
                        }
                        | Property::BorderRadius {
                            top_left: left,
                            top_right: top,
                            bottom_right: right,
                            bottom_left: bottom,
                        }
                        | Property::HitSlop {
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
                        Property::BorderColor(c) | Property::RippleColor(c) => {
                            payload.extend_from_slice(&(4u16).to_le_bytes());
                            payload.extend_from_slice(&c.to_le_bytes());
                        }
                        Property::Transform {
                            translate_x,
                            translate_y,
                            scale_x,
                            scale_y,
                            rotate_deg,
                        } => {
                            payload.extend_from_slice(&(20u16).to_le_bytes());
                            payload.extend_from_slice(&translate_x.to_le_bytes());
                            payload.extend_from_slice(&translate_y.to_le_bytes());
                            payload.extend_from_slice(&scale_x.to_le_bytes());
                            payload.extend_from_slice(&scale_y.to_le_bytes());
                            payload.extend_from_slice(&rotate_deg.to_le_bytes());
                        }
                        Property::FontWeight(w) => {
                            payload.extend_from_slice(&(2u16).to_le_bytes());
                            payload.extend_from_slice(&w.to_le_bytes());
                        }
                        Property::FontStyle(s)
                        | Property::TextAlign(s)
                        | Property::TextDecoration(s)
                        | Property::ImportantForAccessibility(s) => {
                            payload.extend_from_slice(&(1u16).to_le_bytes());
                            payload.push(*s);
                        }
                        Property::NumberOfLines(n) => {
                            payload.extend_from_slice(&(4u16).to_le_bytes());
                            payload.extend_from_slice(&n.to_le_bytes());
                        }
                        Property::TestId(s)
                        | Property::AccessibilityLabel(s)
                        | Property::AccessibilityHint(s) => {
                            let bytes = s.as_bytes();
                            payload.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                            payload.extend_from_slice(bytes);
                        }
                        Property::AccessibilityState {
                            disabled,
                            selected,
                            checked,
                            busy,
                            expanded,
                        } => {
                            payload.extend_from_slice(&(5u16).to_le_bytes());
                            payload.push(if *disabled { 1 } else { 0 });
                            payload.push(if *selected { 1 } else { 0 });
                            payload.push(*checked);
                            payload.push(if *busy { 1 } else { 0 });
                            payload.push(*expanded);
                        }
                        Property::ControlledText { version, text } => {
                            let text_bytes = text.as_bytes();
                            let len = 4 + text_bytes.len();
                            payload.extend_from_slice(&(len as u16).to_le_bytes());
                            payload.extend_from_slice(&version.to_le_bytes());
                            payload.extend_from_slice(text_bytes);
                        }
                        Property::Placeholder(s) => {
                            let bytes = s.as_bytes();
                            payload.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                            payload.extend_from_slice(bytes);
                        }
                        Property::PlaceholderTextColor(c)
                        | Property::TintColor(c)
                        | Property::ThumbColor(c)
                        | Property::IndicatorColor(c)
                        | Property::ProgressBackgroundColor(c) => {
                            payload.extend_from_slice(&(4u16).to_le_bytes());
                            payload.extend_from_slice(&c.to_le_bytes());
                        }
                        Property::SecureTextEntry(b)
                        | Property::Multiline(b)
                        | Property::AutoCorrect(b)
                        | Property::Editable(b)
                        | Property::ScrollHorizontal(b)
                        | Property::PagingEnabled(b)
                        | Property::ShowsScrollIndicator(b)
                        | Property::SwitchValue(b)
                        | Property::Animating(b)
                        | Property::ModalVisible(b)
                        | Property::ModalTransparent(b)
                        | Property::Refreshing(b) => {
                            payload.extend_from_slice(&(1u16).to_le_bytes());
                            payload.push(if *b { 1 } else { 0 });
                        }
                        Property::KeyboardType(u)
                        | Property::ReturnKeyType(u)
                        | Property::AutoCapitalize(u)
                        | Property::ImageResizeMode(u)
                        | Property::IndicatorSize(u)
                        | Property::ModalAnimationType(u) => {
                            payload.extend_from_slice(&(1u16).to_le_bytes());
                            payload.push(*u);
                        }
                        Property::MaxLength(u) | Property::ItemCount(u) => {
                            payload.extend_from_slice(&(4u16).to_le_bytes());
                            payload.extend_from_slice(&u.to_le_bytes());
                        }
                        Property::Selection { start, end } => {
                            payload.extend_from_slice(&(8u16).to_le_bytes());
                            payload.extend_from_slice(&start.to_le_bytes());
                            payload.extend_from_slice(&end.to_le_bytes());
                        }
                        Property::BlurRadius(f) | Property::ItemHeight(f) => {
                            payload.extend_from_slice(&(4u16).to_le_bytes());
                            payload.extend_from_slice(&f.to_le_bytes());
                        }
                        Property::ContentOffset { x, y } => {
                            payload.extend_from_slice(&(8u16).to_le_bytes());
                            payload.extend_from_slice(&x.to_le_bytes());
                            payload.extend_from_slice(&y.to_le_bytes());
                        }
                        Property::TrackColor { active, inactive } => {
                            payload.extend_from_slice(&(8u16).to_le_bytes());
                            payload.extend_from_slice(&active.to_le_bytes());
                            payload.extend_from_slice(&inactive.to_le_bytes());
                        }
                        Property::Checked(b) => {
                            payload.extend_from_slice(&(1u16).to_le_bytes());
                            payload.push(if *b { 1 } else { 0 });
                        }
                        Property::SliderValue(f)
                        | Property::Progress(f)
                        | Property::IconSize(f) => {
                            payload.extend_from_slice(&(4u16).to_le_bytes());
                            payload.extend_from_slice(&f.to_le_bytes());
                        }
                        Property::SliderRange { min, max, step } => {
                            payload.extend_from_slice(&(12u16).to_le_bytes());
                            payload.extend_from_slice(&min.to_le_bytes());
                            payload.extend_from_slice(&max.to_le_bytes());
                            payload.extend_from_slice(&step.to_le_bytes());
                        }
                        Property::GradientColors {
                            start_color,
                            end_color,
                            angle_deg,
                        } => {
                            payload.extend_from_slice(&(12u16).to_le_bytes());
                            payload.extend_from_slice(&start_color.to_le_bytes());
                            payload.extend_from_slice(&end_color.to_le_bytes());
                            payload.extend_from_slice(&angle_deg.to_le_bytes());
                        }
                        Property::IconName(s) | Property::BadgeContent(s) => {
                            let bytes = s.as_bytes();
                            payload.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                            payload.extend_from_slice(bytes);
                        }
                        Property::GridColumns(u) => {
                            payload.extend_from_slice(&(4u16).to_le_bytes());
                            payload.extend_from_slice(&u.to_le_bytes());
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
                        8 => Property::ImageUrl(
                            std::str::from_utf8(prop_slice)
                                .map_err(|_| ProtocolError::InvalidUtf8)?
                                .to_string(),
                        ),
                        9 => {
                            if prop_slice.len() < 8 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 8,
                                });
                            }
                            let target = f32::from_le_bytes(prop_slice[0..4].try_into().unwrap());
                            let duration_ms =
                                u32::from_le_bytes(prop_slice[4..8].try_into().unwrap());
                            Property::AnimateAlpha {
                                target,
                                duration_ms,
                            }
                        }
                        10 => {
                            if prop_slice.len() < 8 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 8,
                                });
                            }
                            let target = f32::from_le_bytes(prop_slice[0..4].try_into().unwrap());
                            let duration_ms =
                                u32::from_le_bytes(prop_slice[4..8].try_into().unwrap());
                            Property::AnimateScale {
                                target,
                                duration_ms,
                            }
                        }
                        11 => {
                            if prop_slice.len() < 8 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 8,
                                });
                            }
                            let target = f32::from_le_bytes(prop_slice[0..4].try_into().unwrap());
                            let duration_ms =
                                u32::from_le_bytes(prop_slice[4..8].try_into().unwrap());
                            Property::AnimateTranslationY {
                                target,
                                duration_ms,
                            }
                        }
                        12 => Property::ContentDescription(
                            std::str::from_utf8(prop_slice)
                                .map_err(|_| ProtocolError::InvalidUtf8)?
                                .to_string(),
                        ),
                        13 => Property::AccessibilityRole(
                            std::str::from_utf8(prop_slice)
                                .map_err(|_| ProtocolError::InvalidUtf8)?
                                .to_string(),
                        ),
                        14 => Property::AccessibilityHeading(
                            prop_slice.first().copied().unwrap_or(0) != 0,
                        ),
                        15 => Property::Hint(
                            std::str::from_utf8(prop_slice)
                                .map_err(|_| ProtocolError::InvalidUtf8)?
                                .to_string(),
                        ),
                        16 => {
                            Property::Opacity(f32::from_le_bytes(prop_slice.try_into().map_err(
                                |_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                },
                            )?))
                        }
                        17 => {
                            if prop_slice.len() < 16 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 16,
                                });
                            }
                            Property::BorderWidth {
                                left: f32::from_le_bytes(prop_slice[0..4].try_into().unwrap()),
                                top: f32::from_le_bytes(prop_slice[4..8].try_into().unwrap()),
                                right: f32::from_le_bytes(prop_slice[8..12].try_into().unwrap()),
                                bottom: f32::from_le_bytes(prop_slice[12..16].try_into().unwrap()),
                            }
                        }
                        18 => Property::BorderColor(u32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        19 => {
                            if prop_slice.len() < 16 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 16,
                                });
                            }
                            Property::BorderRadius {
                                top_left: f32::from_le_bytes(prop_slice[0..4].try_into().unwrap()),
                                top_right: f32::from_le_bytes(prop_slice[4..8].try_into().unwrap()),
                                bottom_right: f32::from_le_bytes(
                                    prop_slice[8..12].try_into().unwrap(),
                                ),
                                bottom_left: f32::from_le_bytes(
                                    prop_slice[12..16].try_into().unwrap(),
                                ),
                            }
                        }
                        20 => {
                            Property::Elevation(f32::from_le_bytes(prop_slice.try_into().map_err(
                                |_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                },
                            )?))
                        }
                        21 => {
                            if prop_slice.len() < 20 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 20,
                                });
                            }
                            Property::Transform {
                                translate_x: f32::from_le_bytes(
                                    prop_slice[0..4].try_into().unwrap(),
                                ),
                                translate_y: f32::from_le_bytes(
                                    prop_slice[4..8].try_into().unwrap(),
                                ),
                                scale_x: f32::from_le_bytes(prop_slice[8..12].try_into().unwrap()),
                                scale_y: f32::from_le_bytes(prop_slice[12..16].try_into().unwrap()),
                                rotate_deg: f32::from_le_bytes(
                                    prop_slice[16..20].try_into().unwrap(),
                                ),
                            }
                        }
                        22 => Property::FontWeight(u16::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 2,
                                })?,
                        )),
                        23 => Property::FontStyle(prop_slice.first().copied().unwrap_or(0)),
                        24 => Property::LineHeight(f32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        25 => Property::LetterSpacing(f32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        26 => Property::TextAlign(prop_slice.first().copied().unwrap_or(0)),
                        27 => Property::TextDecoration(prop_slice.first().copied().unwrap_or(0)),
                        28 => Property::NumberOfLines(u32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        29 => Property::TestId(
                            std::str::from_utf8(prop_slice)
                                .map_err(|_| ProtocolError::InvalidUtf8)?
                                .to_string(),
                        ),
                        30 => Property::AccessibilityLabel(
                            std::str::from_utf8(prop_slice)
                                .map_err(|_| ProtocolError::InvalidUtf8)?
                                .to_string(),
                        ),
                        31 => Property::AccessibilityHint(
                            std::str::from_utf8(prop_slice)
                                .map_err(|_| ProtocolError::InvalidUtf8)?
                                .to_string(),
                        ),
                        32 => {
                            if prop_slice.len() < 5 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 5,
                                });
                            }
                            Property::AccessibilityState {
                                disabled: prop_slice[0] != 0,
                                selected: prop_slice[1] != 0,
                                checked: prop_slice[2],
                                busy: prop_slice[3] != 0,
                                expanded: prop_slice[4],
                            }
                        }
                        33 => Property::ImportantForAccessibility(
                            prop_slice.first().copied().unwrap_or(0),
                        ),
                        34 => Property::RippleColor(u32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        35 => {
                            if prop_slice.len() < 16 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 16,
                                });
                            }
                            Property::HitSlop {
                                left: f32::from_le_bytes(prop_slice[0..4].try_into().unwrap()),
                                top: f32::from_le_bytes(prop_slice[4..8].try_into().unwrap()),
                                right: f32::from_le_bytes(prop_slice[8..12].try_into().unwrap()),
                                bottom: f32::from_le_bytes(prop_slice[12..16].try_into().unwrap()),
                            }
                        }
                        36 => {
                            if prop_slice.len() < 4 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                });
                            }
                            let version = u32::from_le_bytes(prop_slice[0..4].try_into().unwrap());
                            let text = std::str::from_utf8(&prop_slice[4..])
                                .map_err(|_| ProtocolError::InvalidUtf8)?
                                .to_string();
                            Property::ControlledText { version, text }
                        }
                        37 => Property::Placeholder(
                            std::str::from_utf8(prop_slice)
                                .map_err(|_| ProtocolError::InvalidUtf8)?
                                .to_string(),
                        ),
                        38 => Property::PlaceholderTextColor(u32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        39 => {
                            Property::SecureTextEntry(prop_slice.first().copied().unwrap_or(0) != 0)
                        }
                        40 => Property::KeyboardType(prop_slice.first().copied().unwrap_or(0)),
                        41 => Property::ReturnKeyType(prop_slice.first().copied().unwrap_or(0)),
                        42 => {
                            Property::MaxLength(u32::from_le_bytes(prop_slice.try_into().map_err(
                                |_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                },
                            )?))
                        }
                        43 => Property::Multiline(prop_slice.first().copied().unwrap_or(0) != 0),
                        44 => Property::AutoCapitalize(prop_slice.first().copied().unwrap_or(0)),
                        45 => Property::AutoCorrect(prop_slice.first().copied().unwrap_or(0) != 0),
                        46 => Property::Editable(prop_slice.first().copied().unwrap_or(0) != 0),
                        47 => {
                            if prop_slice.len() < 8 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 8,
                                });
                            }
                            let start = u32::from_le_bytes(prop_slice[0..4].try_into().unwrap());
                            let end = u32::from_le_bytes(prop_slice[4..8].try_into().unwrap());
                            Property::Selection { start, end }
                        }
                        48 => Property::ImageResizeMode(prop_slice.first().copied().unwrap_or(0)),
                        49 => Property::BlurRadius(f32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        50 => {
                            Property::TintColor(u32::from_le_bytes(prop_slice.try_into().map_err(
                                |_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                },
                            )?))
                        }
                        51 => Property::ScrollHorizontal(
                            prop_slice.first().copied().unwrap_or(0) != 0,
                        ),
                        52 => {
                            Property::PagingEnabled(prop_slice.first().copied().unwrap_or(0) != 0)
                        }
                        53 => Property::ShowsScrollIndicator(
                            prop_slice.first().copied().unwrap_or(0) != 0,
                        ),
                        54 => {
                            if prop_slice.len() < 8 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 8,
                                });
                            }
                            let x = f32::from_le_bytes(prop_slice[0..4].try_into().unwrap());
                            let y = f32::from_le_bytes(prop_slice[4..8].try_into().unwrap());
                            Property::ContentOffset { x, y }
                        }
                        55 => {
                            Property::ItemCount(u32::from_le_bytes(prop_slice.try_into().map_err(
                                |_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                },
                            )?))
                        }
                        56 => Property::ItemHeight(f32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        57 => Property::SwitchValue(prop_slice.first().copied().unwrap_or(0) != 0),
                        58 => {
                            if prop_slice.len() < 8 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 8,
                                });
                            }
                            let active = u32::from_le_bytes(prop_slice[0..4].try_into().unwrap());
                            let inactive = u32::from_le_bytes(prop_slice[4..8].try_into().unwrap());
                            Property::TrackColor { active, inactive }
                        }
                        59 => Property::ThumbColor(u32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        60 => Property::IndicatorColor(u32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        61 => Property::IndicatorSize(prop_slice.first().copied().unwrap_or(0)),
                        62 => Property::Animating(prop_slice.first().copied().unwrap_or(0) != 0),
                        63 => Property::ModalVisible(prop_slice.first().copied().unwrap_or(0) != 0),
                        64 => {
                            Property::ModalAnimationType(prop_slice.first().copied().unwrap_or(0))
                        }
                        65 => Property::ModalTransparent(
                            prop_slice.first().copied().unwrap_or(0) != 0,
                        ),
                        66 => Property::Refreshing(prop_slice.first().copied().unwrap_or(0) != 0),
                        67 => Property::ProgressBackgroundColor(u32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        68 => Property::Checked(prop_slice.first().copied().unwrap_or(0) != 0),
                        69 => Property::SliderValue(f32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
                        70 => {
                            if prop_slice.len() < 12 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 12,
                                });
                            }
                            let min = f32::from_le_bytes(prop_slice[0..4].try_into().unwrap());
                            let max = f32::from_le_bytes(prop_slice[4..8].try_into().unwrap());
                            let step = f32::from_le_bytes(prop_slice[8..12].try_into().unwrap());
                            Property::SliderRange { min, max, step }
                        }
                        71 => {
                            Property::Progress(f32::from_le_bytes(prop_slice.try_into().map_err(
                                |_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                },
                            )?))
                        }
                        72 => {
                            if prop_slice.len() < 12 {
                                return Err(ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 12,
                                });
                            }
                            let start_color =
                                u32::from_le_bytes(prop_slice[0..4].try_into().unwrap());
                            let end_color =
                                u32::from_le_bytes(prop_slice[4..8].try_into().unwrap());
                            let angle_deg =
                                f32::from_le_bytes(prop_slice[8..12].try_into().unwrap());
                            Property::GradientColors {
                                start_color,
                                end_color,
                                angle_deg,
                            }
                        }
                        73 => Property::IconName(
                            std::str::from_utf8(prop_slice)
                                .map_err(|_| ProtocolError::InvalidUtf8)?
                                .to_string(),
                        ),
                        74 => {
                            Property::IconSize(f32::from_le_bytes(prop_slice.try_into().map_err(
                                |_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                },
                            )?))
                        }
                        75 => Property::BadgeContent(
                            std::str::from_utf8(prop_slice)
                                .map_err(|_| ProtocolError::InvalidUtf8)?
                                .to_string(),
                        ),
                        76 => Property::GridColumns(u32::from_le_bytes(
                            prop_slice
                                .try_into()
                                .map_err(|_| ProtocolError::UnexpectedEof {
                                    offset,
                                    expected: 4,
                                })?,
                        )),
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

/// Magic header bytes for imperative commands: "FXCM"
pub const COMMAND_MAGIC: [u8; 4] = [0x46, 0x58, 0x43, 0x4D];
/// Magic header bytes for text measure requests: "FXTM"
pub const TEXT_MEASURE_REQ_MAGIC: [u8; 4] = [0x46, 0x58, 0x54, 0x4D];
/// Magic header bytes for text measure responses: "FXTR"
pub const TEXT_MEASURE_RESP_MAGIC: [u8; 4] = [0x46, 0x58, 0x54, 0x52];

/// Imperative UI command sent from Rust runtime to platform renderer.
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    FocusNode {
        node_id: NodeId,
    },
    BlurNode {
        node_id: NodeId,
    },
    ScrollTo {
        node_id: NodeId,
        x: f32,
        y: f32,
        animated: bool,
    },
    MeasureNode {
        node_id: NodeId,
    },
    Navigate {
        container_id: NodeId,
        screen_id: NodeId,
        animation: u8,
    },
    GoBack {
        container_id: NodeId,
        animation: u8,
    },
    SetScreenOptions {
        screen_id: NodeId,
        title: String,
        header_shown: bool,
        presentation: u8,
    },
}

/// A batch of imperative commands.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandBatch {
    pub commands: Vec<Command>,
}

impl CommandBatch {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn push(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + self.commands.len() * 17);
        buf.extend_from_slice(&COMMAND_MAGIC);
        buf.extend_from_slice(&(self.commands.len() as u32).to_le_bytes());
        for cmd in &self.commands {
            match cmd {
                Command::FocusNode { node_id } => {
                    buf.push(1u8);
                    buf.extend_from_slice(&node_id.raw().to_le_bytes());
                }
                Command::BlurNode { node_id } => {
                    buf.push(2u8);
                    buf.extend_from_slice(&node_id.raw().to_le_bytes());
                }
                Command::ScrollTo {
                    node_id,
                    x,
                    y,
                    animated,
                } => {
                    buf.push(3u8);
                    buf.extend_from_slice(&node_id.raw().to_le_bytes());
                    buf.extend_from_slice(&x.to_le_bytes());
                    buf.extend_from_slice(&y.to_le_bytes());
                    buf.push(if *animated { 1 } else { 0 });
                }
                Command::MeasureNode { node_id } => {
                    buf.push(4u8);
                    buf.extend_from_slice(&node_id.raw().to_le_bytes());
                }
                Command::Navigate {
                    container_id,
                    screen_id,
                    animation,
                } => {
                    buf.push(5u8);
                    buf.extend_from_slice(&container_id.raw().to_le_bytes());
                    buf.extend_from_slice(&screen_id.raw().to_le_bytes());
                    buf.push(*animation);
                }
                Command::GoBack {
                    container_id,
                    animation,
                } => {
                    buf.push(6u8);
                    buf.extend_from_slice(&container_id.raw().to_le_bytes());
                    buf.push(*animation);
                }
                Command::SetScreenOptions {
                    screen_id,
                    title,
                    header_shown,
                    presentation,
                } => {
                    buf.push(7u8);
                    buf.extend_from_slice(&screen_id.raw().to_le_bytes());
                    let title_bytes = title.as_bytes();
                    buf.extend_from_slice(&(title_bytes.len() as u32).to_le_bytes());
                    buf.extend_from_slice(title_bytes);
                    buf.push(if *header_shown { 1 } else { 0 });
                    buf.push(*presentation);
                }
            }
        }
        buf
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() < 8 {
            return Err(ProtocolError::UnexpectedEof {
                offset: bytes.len(),
                expected: 8,
            });
        }
        if bytes[0..4] != COMMAND_MAGIC {
            return Err(ProtocolError::InvalidMagic);
        }
        let count = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
        let mut offset = 8;
        let mut commands = Vec::with_capacity(count);
        for _ in 0..count {
            if offset >= bytes.len() {
                return Err(ProtocolError::UnexpectedEof {
                    offset,
                    expected: offset + 1,
                });
            }
            let op = bytes[offset];
            offset += 1;
            match op {
                1 => {
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
                    commands.push(Command::FocusNode { node_id });
                }
                2 => {
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
                    commands.push(Command::BlurNode { node_id });
                }
                3 => {
                    if offset + 17 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 17,
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
                    let animated = bytes[offset] != 0;
                    offset += 1;
                    commands.push(Command::ScrollTo {
                        node_id,
                        x,
                        y,
                        animated,
                    });
                }
                4 => {
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
                    commands.push(Command::MeasureNode { node_id });
                }
                5 => {
                    if offset + 17 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 17,
                        });
                    }
                    let container_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let screen_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let animation = bytes[offset];
                    offset += 1;
                    commands.push(Command::Navigate {
                        container_id,
                        screen_id,
                        animation,
                    });
                }
                6 => {
                    if offset + 9 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 9,
                        });
                    }
                    let container_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let animation = bytes[offset];
                    offset += 1;
                    commands.push(Command::GoBack {
                        container_id,
                        animation,
                    });
                }
                7 => {
                    if offset + 14 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + 14,
                        });
                    }
                    let screen_id = NodeId(u64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    ));
                    offset += 8;
                    let title_len =
                        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
                    offset += 4;
                    if offset + title_len + 2 > bytes.len() {
                        return Err(ProtocolError::UnexpectedEof {
                            offset,
                            expected: offset + title_len + 2,
                        });
                    }
                    let title = String::from_utf8(bytes[offset..offset + title_len].to_vec())
                        .map_err(|_| ProtocolError::InvalidUtf8)?;
                    offset += title_len;
                    let header_shown = bytes[offset] != 0;
                    offset += 1;
                    let presentation = bytes[offset];
                    offset += 1;
                    commands.push(Command::SetScreenOptions {
                        screen_id,
                        title,
                        header_shown,
                        presentation,
                    });
                }
                _ => return Err(ProtocolError::UnknownOpcode(op)),
            }
        }
        Ok(Self { commands })
    }
}

/// Single text measurement request sent from Rust to Kotlin.
#[derive(Clone, Debug, PartialEq)]
pub struct TextMeasureRequest {
    pub request_id: u32,
    pub text: String,
    pub font_size_sp: f32,
    pub font_weight: u16,
    pub font_style: u8,
    pub max_width: f32,
}

/// Single text measurement response returned from Kotlin to Rust.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextMeasureResponse {
    pub request_id: u32,
    pub width: f32,
    pub height: f32,
}

/// Encodes a batch of text measurement requests.
pub fn encode_text_measure_requests(requests: &[TextMeasureRequest]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&TEXT_MEASURE_REQ_MAGIC);
    buf.extend_from_slice(&(requests.len() as u32).to_le_bytes());
    for req in requests {
        buf.extend_from_slice(&req.request_id.to_le_bytes());
        buf.extend_from_slice(&req.font_size_sp.to_le_bytes());
        buf.extend_from_slice(&req.font_weight.to_le_bytes());
        buf.push(req.font_style);
        buf.extend_from_slice(&req.max_width.to_le_bytes());
        let text_bytes = req.text.as_bytes();
        buf.extend_from_slice(&(text_bytes.len() as u32).to_le_bytes());
        buf.extend_from_slice(text_bytes);
    }
    buf
}

/// Decodes a batch of text measurement requests.
pub fn decode_text_measure_requests(
    bytes: &[u8],
) -> Result<Vec<TextMeasureRequest>, ProtocolError> {
    if bytes.len() < 8 {
        return Err(ProtocolError::UnexpectedEof {
            offset: bytes.len(),
            expected: 8,
        });
    }
    if bytes[0..4] != TEXT_MEASURE_REQ_MAGIC {
        return Err(ProtocolError::InvalidMagic);
    }
    let count = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
    let mut offset = 8;
    let mut requests = Vec::with_capacity(count);
    for _ in 0..count {
        if offset + 19 > bytes.len() {
            return Err(ProtocolError::UnexpectedEof {
                offset,
                expected: offset + 19,
            });
        }
        let request_id = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let font_size_sp = f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let font_weight = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
        offset += 2;
        let font_style = bytes[offset];
        offset += 1;
        let max_width = f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let text_len = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
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
        requests.push(TextMeasureRequest {
            request_id,
            text,
            font_size_sp,
            font_weight,
            font_style,
            max_width,
        });
    }
    Ok(requests)
}

/// Encodes a batch of text measurement responses.
pub fn encode_text_measure_responses(responses: &[TextMeasureResponse]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&TEXT_MEASURE_RESP_MAGIC);
    buf.extend_from_slice(&(responses.len() as u32).to_le_bytes());
    for resp in responses {
        buf.extend_from_slice(&resp.request_id.to_le_bytes());
        buf.extend_from_slice(&resp.width.to_le_bytes());
        buf.extend_from_slice(&resp.height.to_le_bytes());
    }
    buf
}

/// Decodes a batch of text measurement responses.
pub fn decode_text_measure_responses(
    bytes: &[u8],
) -> Result<Vec<TextMeasureResponse>, ProtocolError> {
    if bytes.len() < 8 {
        return Err(ProtocolError::UnexpectedEof {
            offset: bytes.len(),
            expected: 8,
        });
    }
    if bytes[0..4] != TEXT_MEASURE_RESP_MAGIC {
        return Err(ProtocolError::InvalidMagic);
    }
    let count = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
    let mut offset = 8;
    let mut responses = Vec::with_capacity(count);
    for _ in 0..count {
        if offset + 12 > bytes.len() {
            return Err(ProtocolError::UnexpectedEof {
                offset,
                expected: offset + 12,
            });
        }
        let request_id = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let width = f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let height = f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;
        responses.push(TextMeasureResponse {
            request_id,
            width,
            height,
        });
    }
    Ok(responses)
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

    #[test]
    fn test_extended_properties_round_trip() {
        let mut batch = PatchBatch::new();
        batch.push(Patch::CreateNode {
            node_id: NodeId(20),
            view_type: ViewType::Pressable,
        });
        batch.push(Patch::SetProperty {
            node_id: NodeId(20),
            property: Property::Opacity(0.85),
        });
        batch.push(Patch::SetProperty {
            node_id: NodeId(20),
            property: Property::BorderWidth {
                left: 1.0,
                top: 2.0,
                right: 3.0,
                bottom: 4.0,
            },
        });
        batch.push(Patch::SetProperty {
            node_id: NodeId(20),
            property: Property::BorderColor(0xFF00FF00),
        });
        batch.push(Patch::SetProperty {
            node_id: NodeId(20),
            property: Property::BorderRadius {
                top_left: 8.0,
                top_right: 8.0,
                bottom_right: 4.0,
                bottom_left: 4.0,
            },
        });
        batch.push(Patch::SetProperty {
            node_id: NodeId(20),
            property: Property::Transform {
                translate_x: 10.0,
                translate_y: -5.0,
                scale_x: 1.2,
                scale_y: 1.2,
                rotate_deg: 45.0,
            },
        });
        batch.push(Patch::SetProperty {
            node_id: NodeId(20),
            property: Property::TestId("test_pressable_btn".to_string()),
        });
        batch.push(Patch::SetProperty {
            node_id: NodeId(20),
            property: Property::AccessibilityState {
                disabled: false,
                selected: true,
                checked: 1,
                busy: false,
                expanded: 0,
            },
        });
        batch.push(Patch::AttachEventListener {
            node_id: NodeId(20),
            event_type: EventType::PressIn,
        });

        let encoded = batch.encode();
        let decoded = PatchBatch::decode(&encoded).expect("failed to decode extended batch");
        assert_eq!(batch, decoded);
    }

    #[test]
    fn test_command_batch_round_trip() {
        let mut batch = CommandBatch::new();
        batch.push(Command::FocusNode {
            node_id: NodeId(100),
        });
        batch.push(Command::ScrollTo {
            node_id: NodeId(101),
            x: 0.0,
            y: 350.0,
            animated: true,
        });
        batch.push(Command::BlurNode {
            node_id: NodeId(100),
        });
        batch.push(Command::MeasureNode {
            node_id: NodeId(101),
        });
        batch.push(Command::Navigate {
            container_id: NodeId(1),
            screen_id: NodeId(2),
            animation: 1,
        });
        batch.push(Command::GoBack {
            container_id: NodeId(1),
            animation: 2,
        });
        batch.push(Command::SetScreenOptions {
            screen_id: NodeId(2),
            title: "Screen Title".to_string(),
            header_shown: true,
            presentation: 0,
        });

        let encoded = batch.encode();
        let decoded = CommandBatch::decode(&encoded).expect("failed to decode command batch");
        assert_eq!(batch, decoded);
    }

    #[test]
    fn test_text_measure_batch_round_trip() {
        let requests = vec![
            TextMeasureRequest {
                request_id: 1,
                text: "Hello World".to_string(),
                font_size_sp: 16.0,
                font_weight: 400,
                font_style: 0,
                max_width: 320.0,
            },
            TextMeasureRequest {
                request_id: 2,
                text: "Ferrix High Performance UI".to_string(),
                font_size_sp: 24.0,
                font_weight: 700,
                font_style: 1,
                max_width: 280.0,
            },
        ];

        let req_encoded = encode_text_measure_requests(&requests);
        let req_decoded =
            decode_text_measure_requests(&req_encoded).expect("failed to decode requests");
        assert_eq!(requests, req_decoded);

        let responses = vec![
            TextMeasureResponse {
                request_id: 1,
                width: 95.0,
                height: 22.0,
            },
            TextMeasureResponse {
                request_id: 2,
                width: 260.0,
                height: 64.0,
            },
        ];

        let resp_encoded = encode_text_measure_responses(&responses);
        let resp_decoded =
            decode_text_measure_responses(&resp_encoded).expect("failed to decode responses");
        assert_eq!(responses, resp_decoded);
    }

    #[test]
    fn test_wave1_properties_round_trip() {
        let mut batch = PatchBatch::new();
        let node = NodeId(500);

        batch.push(Patch::CreateNode {
            node_id: node,
            view_type: ViewType::RecyclerView,
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ControlledText {
                version: 42,
                text: "Controlled Content".to_string(),
            },
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::Placeholder("Type here...".to_string()),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::PlaceholderTextColor(0x88888888),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::SecureTextEntry(true),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::KeyboardType(1),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ReturnKeyType(2),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::MaxLength(100),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::Multiline(true),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::AutoCapitalize(1),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::AutoCorrect(false),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::Editable(true),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::Selection { start: 2, end: 5 },
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ImageResizeMode(1),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::BlurRadius(5.5),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::TintColor(0xFF112233),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ScrollHorizontal(true),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::PagingEnabled(true),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ShowsScrollIndicator(false),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ContentOffset { x: 120.0, y: 340.0 },
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ItemCount(10000),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ItemHeight(64.0),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::SwitchValue(true),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::TrackColor {
                active: 0xFF00FF00,
                inactive: 0xFF888888,
            },
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ThumbColor(0xFFFFFFFF),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::IndicatorColor(0xFF0000FF),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::IndicatorSize(1),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::Animating(true),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ModalVisible(true),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ModalAnimationType(2),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ModalTransparent(true),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::Refreshing(false),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::ProgressBackgroundColor(0xFFEEEEEE),
        });
        batch.push(Patch::AttachEventListener {
            node_id: node,
            event_type: EventType::EndReached,
        });
        batch.push(Patch::AttachEventListener {
            node_id: node,
            event_type: EventType::Refresh,
        });

        let encoded = batch.encode();
        let decoded = PatchBatch::decode(&encoded).expect("failed to decode wave 1 patch batch");
        assert_eq!(batch, decoded);
    }

    #[test]
    fn test_wave2_properties_round_trip() {
        let mut batch = PatchBatch::new();
        let node = NodeId(600);

        batch.push(Patch::CreateNode {
            node_id: node,
            view_type: ViewType::Slider,
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::Checked(true),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::SliderValue(0.75),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::SliderRange {
                min: 0.0,
                max: 1.0,
                step: 0.05,
            },
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::Progress(0.65),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::GradientColors {
                start_color: 0xFF2563EB,
                end_color: 0xFF9333EA,
                angle_deg: 90.0,
            },
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::IconName("check-circle".to_string()),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::IconSize(24.0),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::BadgeContent("NEW".to_string()),
        });
        batch.push(Patch::SetProperty {
            node_id: node,
            property: Property::GridColumns(3),
        });
        batch.push(Patch::AttachEventListener {
            node_id: node,
            event_type: EventType::SliderChange,
        });

        let encoded = batch.encode();
        let decoded = PatchBatch::decode(&encoded).expect("failed to decode wave 2 patch batch");
        assert_eq!(batch, decoded);
    }
}
