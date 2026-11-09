//! Ergonomic declarative UI component builders matching the FSX specification.

use crate::element::ElementDef;
use crate::node_id::NodeIdAllocator;
use crate::style::Style;
use crate::Element;
use ferrix_renderer_protocol::{EventType, NodeId, Property, ViewType};

/// The fundamental flexbox/box container primitive (maps to native ViewGroup / FrameLayout).
#[derive(Clone, Debug)]
pub struct View {
    def: ElementDef,
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}

impl View {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::Container),
        }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.def.props.push(Property::Padding {
            left: p,
            top: p,
            right: p,
            bottom: p,
        });
        self
    }

    pub fn background_color(mut self, color: u32) -> Self {
        self.def.props.push(Property::BackgroundColor(color));
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
        self
    }

    pub fn accessible(self, _accessible: bool) -> Self {
        self
    }

    pub fn accessibility_label(mut self, label: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::AccessibilityLabel(label.into()));
        self
    }

    pub fn accessibility_hint(mut self, hint: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::AccessibilityHint(hint.into()));
        self
    }

    pub fn accessibility_role(mut self, role: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::AccessibilityRole(role.into()));
        self
    }

    pub fn on_layout(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::Layout) {
            self.def.event_listeners.push(EventType::Layout);
        }
        self
    }

    pub fn child(mut self, child: impl Into<ElementDef>) -> Self {
        self.def.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ElementDef>>) -> Self {
        for child in children {
            self.def.children.push(child.into());
        }
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.def.key = Some(key.into());
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl From<View> for ElementDef {
    fn from(val: View) -> Self {
        val.def
    }
}

/// Core touch gesture responder component.
#[derive(Clone, Debug)]
pub struct Pressable {
    def: ElementDef,
}

impl Default for Pressable {
    fn default() -> Self {
        Self::new()
    }
}

impl Pressable {
    pub fn new() -> Self {
        let mut def = ElementDef::new(ViewType::Pressable);
        def.event_listeners.push(EventType::Click);
        Self { def }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
        self
    }

    pub fn ripple_color(mut self, color: u32) -> Self {
        self.def.props.push(Property::RippleColor(color));
        self
    }

    pub fn hit_slop(mut self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        self.def.props.push(Property::HitSlop {
            left,
            top,
            right,
            bottom,
        });
        self
    }

    pub fn disabled(mut self, dis: bool) -> Self {
        self.def.props.push(Property::Enabled(!dis));
        self
    }

    pub fn on_press(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::Click) {
            self.def.event_listeners.push(EventType::Click);
        }
        self
    }

    pub fn on_press_in(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::PressIn) {
            self.def.event_listeners.push(EventType::PressIn);
        }
        self
    }

    pub fn on_press_out(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::PressOut) {
            self.def.event_listeners.push(EventType::PressOut);
        }
        self
    }

    pub fn on_long_press(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::LongPress) {
            self.def.event_listeners.push(EventType::LongPress);
        }
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
        self
    }

    pub fn accessibility_label(mut self, label: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::AccessibilityLabel(label.into()));
        self
    }

    pub fn accessibility_role(mut self, role: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::AccessibilityRole(role.into()));
        self
    }

    pub fn child(mut self, child: impl Into<ElementDef>) -> Self {
        self.def.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ElementDef>>) -> Self {
        for child in children {
            self.def.children.push(child.into());
        }
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.def.key = Some(key.into());
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl From<Pressable> for ElementDef {
    fn from(val: Pressable) -> Self {
        val.def
    }
}

/// Transparent container component that groups elements without creating a backing native View.
#[derive(Clone, Debug, Default)]
pub struct Fragment {
    pub children: Vec<ElementDef>,
}

impl Fragment {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl Into<ElementDef>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ElementDef>>) -> Self {
        for child in children {
            self.children.push(child.into());
        }
        self
    }
}

impl From<Fragment> for ElementDef {
    fn from(val: Fragment) -> Self {
        let mut def = ElementDef::new(ViewType::Fragment);
        def.children = val.children;
        def
    }
}

/// Column layout container (vertical flex-like layout).
#[derive(Clone, Debug)]
pub struct Column {
    def: ElementDef,
}

impl Column {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::Column),
        }
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.def.props.push(Property::Padding {
            left: p,
            top: p,
            right: p,
            bottom: p,
        });
        self
    }

    pub fn padding_rect(mut self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        self.def.props.push(Property::Padding {
            left,
            top,
            right,
            bottom,
        });
        self
    }

    pub fn spacing(mut self, s: f32) -> Self {
        self.def.props.push(Property::Spacing(s));
        self
    }

    pub fn background_color(mut self, color: u32) -> Self {
        self.def.props.push(Property::BackgroundColor(color));
        self
    }

    pub fn child(mut self, child: impl Into<ElementDef>) -> Self {
        self.def.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ElementDef>>) -> Self {
        for child in children {
            self.def.children.push(child.into());
        }
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.def.key = Some(key.into());
        self
    }

    pub fn content_description(mut self, desc: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::ContentDescription(desc.into()));
        self
    }

    pub fn accessibility_heading(mut self, heading: bool) -> Self {
        self.def.props.push(Property::AccessibilityHeading(heading));
        self
    }

    pub fn animate_alpha(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateAlpha {
            target,
            duration_ms,
        });
        self
    }

    pub fn animate_scale(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateScale {
            target,
            duration_ms,
        });
        self
    }

    pub fn animate_translation_y(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateTranslationY {
            target,
            duration_ms,
        });
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Column> for ElementDef {
    fn from(val: Column) -> Self {
        val.def
    }
}

/// Row layout container (horizontal flex-like layout).
#[derive(Clone, Debug)]
pub struct Row {
    def: ElementDef,
}

impl Row {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::Row),
        }
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.def.props.push(Property::Padding {
            left: p,
            top: p,
            right: p,
            bottom: p,
        });
        self
    }

    pub fn padding_rect(mut self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        self.def.props.push(Property::Padding {
            left,
            top,
            right,
            bottom,
        });
        self
    }

    pub fn spacing(mut self, s: f32) -> Self {
        self.def.props.push(Property::Spacing(s));
        self
    }

    pub fn background_color(mut self, color: u32) -> Self {
        self.def.props.push(Property::BackgroundColor(color));
        self
    }

    pub fn child(mut self, child: impl Into<ElementDef>) -> Self {
        self.def.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ElementDef>>) -> Self {
        for child in children {
            self.def.children.push(child.into());
        }
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.def.key = Some(key.into());
        self
    }

    pub fn content_description(mut self, desc: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::ContentDescription(desc.into()));
        self
    }

    pub fn accessibility_heading(mut self, heading: bool) -> Self {
        self.def.props.push(Property::AccessibilityHeading(heading));
        self
    }

    pub fn animate_alpha(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateAlpha {
            target,
            duration_ms,
        });
        self
    }

    pub fn animate_scale(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateScale {
            target,
            duration_ms,
        });
        self
    }

    pub fn animate_translation_y(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateTranslationY {
            target,
            duration_ms,
        });
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Row> for ElementDef {
    fn from(val: Row) -> Self {
        val.def
    }
}

/// Text display element.
#[derive(Clone, Debug)]
pub struct Text {
    def: ElementDef,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        let mut def = ElementDef::new(ViewType::Text);
        def.text = Some(text.into());
        Self { def }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
        self
    }

    pub fn number_of_lines(mut self, lines: u32) -> Self {
        self.def.props.push(Property::NumberOfLines(lines));
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
        self
    }

    pub fn on_press(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::Click) {
            self.def.event_listeners.push(EventType::Click);
        }
        self
    }

    pub fn size(mut self, sp: f32) -> Self {
        self.def.props.push(Property::FontSize(sp));
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.def.props.push(Property::TextColor(color));
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.def.props.push(Property::Padding {
            left: p,
            top: p,
            right: p,
            bottom: p,
        });
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.def.key = Some(key.into());
        self
    }

    pub fn content_description(mut self, desc: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::ContentDescription(desc.into()));
        self
    }

    pub fn accessibility_heading(mut self, heading: bool) -> Self {
        self.def.props.push(Property::AccessibilityHeading(heading));
        self
    }

    pub fn animate_alpha(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateAlpha {
            target,
            duration_ms,
        });
        self
    }

    pub fn animate_scale(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateScale {
            target,
            duration_ms,
        });
        self
    }

    pub fn animate_translation_y(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateTranslationY {
            target,
            duration_ms,
        });
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl From<Text> for ElementDef {
    fn from(val: Text) -> Self {
        val.def
    }
}

/// Interactive button element.
#[derive(Clone, Debug)]
pub struct Button {
    def: ElementDef,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        let mut def = ElementDef::new(ViewType::Button);
        def.text = Some(label.into());
        // Default interactive button listens to Click events
        def.event_listeners.push(EventType::Click);
        Self { def }
    }

    pub fn enabled(mut self, en: bool) -> Self {
        self.def.props.push(Property::Enabled(en));
        self
    }

    pub fn background_color(mut self, color: u32) -> Self {
        self.def.props.push(Property::BackgroundColor(color));
        self
    }

    pub fn text_color(mut self, color: u32) -> Self {
        self.def.props.push(Property::TextColor(color));
        self
    }

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.def.props.push(Property::CornerRadius(radius));
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.def.key = Some(key.into());
        self
    }

    pub fn content_description(mut self, desc: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::ContentDescription(desc.into()));
        self
    }

    pub fn accessibility_heading(mut self, heading: bool) -> Self {
        self.def.props.push(Property::AccessibilityHeading(heading));
        self
    }

    pub fn animate_alpha(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateAlpha {
            target,
            duration_ms,
        });
        self
    }

    pub fn animate_scale(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateScale {
            target,
            duration_ms,
        });
        self
    }

    pub fn animate_translation_y(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateTranslationY {
            target,
            duration_ms,
        });
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl From<Button> for ElementDef {
    fn from(val: Button) -> Self {
        val.def
    }
}

/// Image display element.
#[derive(Clone, Debug)]
pub struct Image {
    def: ElementDef,
}

impl Image {
    pub fn new(url: impl Into<String>) -> Self {
        let mut def = ElementDef::new(ViewType::Image);
        def.props.push(Property::ImageUrl(url.into()));
        Self { def }
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.def.key = Some(key.into());
        self
    }

    pub fn content_description(mut self, desc: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::ContentDescription(desc.into()));
        self
    }

    pub fn accessibility_heading(mut self, heading: bool) -> Self {
        self.def.props.push(Property::AccessibilityHeading(heading));
        self
    }

    pub fn animate_alpha(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateAlpha {
            target,
            duration_ms,
        });
        self
    }

    pub fn animate_scale(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateScale {
            target,
            duration_ms,
        });
        self
    }

    pub fn animate_translation_y(mut self, target: f32, duration_ms: u32) -> Self {
        self.def.props.push(Property::AnimateTranslationY {
            target,
            duration_ms,
        });
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl From<Image> for ElementDef {
    fn from(val: Image) -> Self {
        val.def
    }
}

/// Scrollable container element.
#[derive(Clone, Debug)]
pub struct ScrollView {
    def: ElementDef,
}

impl Default for ScrollView {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrollView {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::ScrollView),
        }
    }

    pub fn child(mut self, child: impl Into<ElementDef>) -> Self {
        self.def.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ElementDef>>) -> Self {
        for child in children {
            self.def.children.push(child.into());
        }
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.def.key = Some(key.into());
        self
    }

    pub fn content_description(mut self, desc: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::ContentDescription(desc.into()));
        self
    }

    pub fn accessibility_heading(mut self, heading: bool) -> Self {
        self.def.props.push(Property::AccessibilityHeading(heading));
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl From<ScrollView> for ElementDef {
    fn from(val: ScrollView) -> Self {
        val.def
    }
}

/// Interactive text input element (maps to native EditText).
#[derive(Clone, Debug)]
pub struct TextInput {
    def: ElementDef,
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

impl TextInput {
    pub fn new() -> Self {
        let mut def = ElementDef::new(ViewType::TextInput);
        def.event_listeners.push(EventType::TextChanged);
        Self { def }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.def.text = Some(text.into());
        self
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.def.props.push(Property::Hint(hint.into()));
        self
    }

    pub fn size(mut self, sp: f32) -> Self {
        self.def.props.push(Property::FontSize(sp));
        self
    }

    pub fn text_color(mut self, color: u32) -> Self {
        self.def.props.push(Property::TextColor(color));
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.def.props.push(Property::Padding {
            left: p,
            top: p,
            right: p,
            bottom: p,
        });
        self
    }

    pub fn content_description(mut self, desc: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::ContentDescription(desc.into()));
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.def.key = Some(key.into());
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl From<TextInput> for ElementDef {
    fn from(val: TextInput) -> Self {
        val.def
    }
}
