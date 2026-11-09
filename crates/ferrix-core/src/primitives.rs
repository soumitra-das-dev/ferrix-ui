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
    pub(crate) def: ElementDef,
}

impl Default for Image {
    fn default() -> Self {
        Self::new("")
    }
}

impl Image {
    pub fn new(url: impl Into<String>) -> Self {
        let mut def = ElementDef::new(ViewType::Image);
        def.props.push(Property::ImageUrl(url.into()));
        Self { def }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.def.props.push(Property::ImageUrl(url.into()));
        self
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
        self
    }

    pub fn resize_mode(mut self, mode: u8) -> Self {
        self.def.props.push(Property::ImageResizeMode(mode));
        self
    }

    pub fn resize_mode_str(self, mode: &str) -> Self {
        let m = match mode {
            "cover" => 1,
            "stretch" => 2,
            "center" => 3,
            _ => 0, // contain
        };
        self.resize_mode(m)
    }

    pub fn blur_radius(mut self, radius: f32) -> Self {
        self.def.props.push(Property::BlurRadius(radius));
        self
    }

    pub fn tint_color(mut self, color: u32) -> Self {
        self.def.props.push(Property::TintColor(color));
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

/// ImageBackground container wrapping an image and nested children.
#[derive(Clone, Debug)]
pub struct ImageBackground {
    image: Image,
    container: View,
}

impl ImageBackground {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            image: Image::new(url),
            container: View::new(),
        }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.container = self.container.style(style);
        self
    }

    pub fn resize_mode(mut self, mode: u8) -> Self {
        self.image = self.image.resize_mode(mode);
        self
    }

    pub fn child(mut self, child: impl Into<ElementDef>) -> Self {
        self.container = self.container.child(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ElementDef>>) -> Self {
        self.container = self.container.children(children);
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.container = self.container.key(key);
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        let mut def: ElementDef = self.container.into();
        def.children.insert(0, self.image.into());
        def.build(allocator)
    }
}

impl From<ImageBackground> for ElementDef {
    fn from(val: ImageBackground) -> Self {
        let mut def: ElementDef = val.container.into();
        def.children.insert(0, val.image.into());
        def
    }
}

/// Scrollable container element.
#[derive(Clone, Debug)]
pub struct ScrollView {
    pub(crate) def: ElementDef,
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

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
        self
    }

    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.def.props.push(Property::ScrollHorizontal(horizontal));
        self
    }

    pub fn paging_enabled(mut self, paging_enabled: bool) -> Self {
        self.def.props.push(Property::PagingEnabled(paging_enabled));
        self
    }

    pub fn shows_scroll_indicator(mut self, shows: bool) -> Self {
        self.def.props.push(Property::ShowsScrollIndicator(shows));
        self
    }

    pub fn content_offset(mut self, x: f32, y: f32) -> Self {
        self.def.props.push(Property::ContentOffset { x, y });
        self
    }

    pub fn on_scroll(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::Scroll) {
            self.def.event_listeners.push(EventType::Scroll);
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

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
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
    pub(crate) def: ElementDef,
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

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.def.text = Some(text.into());
        self
    }

    pub fn controlled_text(mut self, version: u32, text: impl Into<String>) -> Self {
        self.def.props.push(Property::ControlledText {
            version,
            text: text.into(),
        });
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.def
            .props
            .push(Property::Placeholder(placeholder.into()));
        self
    }

    pub fn placeholder_text_color(mut self, color: u32) -> Self {
        self.def.props.push(Property::PlaceholderTextColor(color));
        self
    }

    pub fn secure_text_entry(mut self, secure: bool) -> Self {
        self.def.props.push(Property::SecureTextEntry(secure));
        self
    }

    pub fn keyboard_type(mut self, keyboard_type: u8) -> Self {
        self.def.props.push(Property::KeyboardType(keyboard_type));
        self
    }

    pub fn return_key_type(mut self, return_key_type: u8) -> Self {
        self.def
            .props
            .push(Property::ReturnKeyType(return_key_type));
        self
    }

    pub fn max_length(mut self, max_length: u32) -> Self {
        self.def.props.push(Property::MaxLength(max_length));
        self
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.def.props.push(Property::Multiline(multiline));
        self
    }

    pub fn auto_capitalize(mut self, mode: u8) -> Self {
        self.def.props.push(Property::AutoCapitalize(mode));
        self
    }

    pub fn auto_correct(mut self, auto_correct: bool) -> Self {
        self.def.props.push(Property::AutoCorrect(auto_correct));
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.def.props.push(Property::Editable(editable));
        self
    }

    pub fn selection(mut self, start: u32, end: u32) -> Self {
        self.def.props.push(Property::Selection { start, end });
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

    pub fn on_change_text(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::TextChanged) {
            self.def.event_listeners.push(EventType::TextChanged);
        }
        self
    }

    pub fn on_submit_editing(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::SubmitEditing) {
            self.def.event_listeners.push(EventType::SubmitEditing);
        }
        self
    }

    pub fn on_focus(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::FocusChanged) {
            self.def.event_listeners.push(EventType::FocusChanged);
        }
        self
    }

    pub fn on_blur(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::FocusChanged) {
            self.def.event_listeners.push(EventType::FocusChanged);
        }
        self
    }

    pub fn on_selection_change(mut self) -> Self {
        if !self
            .def
            .event_listeners
            .contains(&EventType::SelectionChange)
        {
            self.def.event_listeners.push(EventType::SelectionChange);
        }
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

/// High performance virtualized list primitive (maps to Android RecyclerView).
#[derive(Clone, Debug)]
pub struct LazyList {
    pub(crate) def: ElementDef,
}

impl Default for LazyList {
    fn default() -> Self {
        Self::new()
    }
}

impl LazyList {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::RecyclerView),
        }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
        self
    }

    pub fn item_count(mut self, count: u32) -> Self {
        self.def.props.push(Property::ItemCount(count));
        self
    }

    pub fn item_height(mut self, height: f32) -> Self {
        self.def.props.push(Property::ItemHeight(height));
        self
    }

    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.def.props.push(Property::ScrollHorizontal(horizontal));
        self
    }

    pub fn paging_enabled(mut self, paging: bool) -> Self {
        self.def.props.push(Property::PagingEnabled(paging));
        self
    }

    pub fn on_end_reached(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::EndReached) {
            self.def.event_listeners.push(EventType::EndReached);
        }
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.def.key = Some(key.into());
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

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl From<LazyList> for ElementDef {
    fn from(val: LazyList) -> Self {
        val.def
    }
}

pub type FlatList = LazyList;

/// Binary switch primitive (maps to native SwitchCompat).
#[derive(Clone, Debug)]
pub struct Switch {
    pub(crate) def: ElementDef,
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

impl Switch {
    pub fn new() -> Self {
        let mut def = ElementDef::new(ViewType::Switch);
        def.event_listeners.push(EventType::ValueChange);
        Self { def }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
        self
    }

    pub fn value(mut self, value: bool) -> Self {
        self.def.props.push(Property::SwitchValue(value));
        self
    }

    pub fn track_color(mut self, active: u32, inactive: u32) -> Self {
        self.def
            .props
            .push(Property::TrackColor { active, inactive });
        self
    }

    pub fn thumb_color(mut self, color: u32) -> Self {
        self.def.props.push(Property::ThumbColor(color));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.def.props.push(Property::Enabled(!disabled));
        self
    }

    pub fn on_value_change(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::ValueChange) {
            self.def.event_listeners.push(EventType::ValueChange);
        }
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
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

impl From<Switch> for ElementDef {
    fn from(val: Switch) -> Self {
        val.def
    }
}

/// Activity spinner indicator (maps to native ProgressBar indeterminate).
#[derive(Clone, Debug)]
pub struct ActivityIndicator {
    pub(crate) def: ElementDef,
}

impl Default for ActivityIndicator {
    fn default() -> Self {
        Self::new()
    }
}

impl ActivityIndicator {
    pub fn new() -> Self {
        let mut def = ElementDef::new(ViewType::ActivityIndicator);
        def.props.push(Property::Animating(true));
        Self { def }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.def.props.push(Property::IndicatorColor(color));
        self
    }

    pub fn size(mut self, size: u8) -> Self {
        self.def.props.push(Property::IndicatorSize(size));
        self
    }

    pub fn animating(mut self, animating: bool) -> Self {
        self.def.props.push(Property::Animating(animating));
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
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

impl From<ActivityIndicator> for ElementDef {
    fn from(val: ActivityIndicator) -> Self {
        val.def
    }
}

/// Modal overlay dialog container.
#[derive(Clone, Debug)]
pub struct Modal {
    pub(crate) def: ElementDef,
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

impl Modal {
    pub fn new() -> Self {
        let mut def = ElementDef::new(ViewType::Modal);
        def.props.push(Property::ModalVisible(true));
        Self { def }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.def.props.push(Property::ModalVisible(visible));
        self
    }

    pub fn animation_type(mut self, animation_type: u8) -> Self {
        self.def
            .props
            .push(Property::ModalAnimationType(animation_type));
        self
    }

    pub fn transparent(mut self, transparent: bool) -> Self {
        self.def.props.push(Property::ModalTransparent(transparent));
        self
    }

    pub fn on_request_close(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::RequestClose) {
            self.def.event_listeners.push(EventType::RequestClose);
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

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
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

impl From<Modal> for ElementDef {
    fn from(val: Modal) -> Self {
        val.def
    }
}

/// Container that automatically handles device safe area insets.
#[derive(Clone, Debug)]
pub struct SafeAreaView {
    pub(crate) def: ElementDef,
}

impl Default for SafeAreaView {
    fn default() -> Self {
        Self::new()
    }
}

impl SafeAreaView {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::SafeAreaView),
        }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
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

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
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

impl From<SafeAreaView> for ElementDef {
    fn from(val: SafeAreaView) -> Self {
        val.def
    }
}

/// Pull-to-refresh control container.
#[derive(Clone, Debug)]
pub struct RefreshControl {
    pub(crate) def: ElementDef,
}

impl Default for RefreshControl {
    fn default() -> Self {
        Self::new()
    }
}

impl RefreshControl {
    pub fn new() -> Self {
        let mut def = ElementDef::new(ViewType::RefreshControl);
        def.event_listeners.push(EventType::Refresh);
        Self { def }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
        self
    }

    pub fn refreshing(mut self, refreshing: bool) -> Self {
        self.def.props.push(Property::Refreshing(refreshing));
        self
    }

    pub fn progress_background_color(mut self, color: u32) -> Self {
        self.def
            .props
            .push(Property::ProgressBackgroundColor(color));
        self
    }

    pub fn on_refresh(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::Refresh) {
            self.def.event_listeners.push(EventType::Refresh);
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

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
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

impl From<RefreshControl> for ElementDef {
    fn from(val: RefreshControl) -> Self {
        val.def
    }
}

/// Out-of-tree portal root component.
#[derive(Clone, Debug)]
pub struct Portal {
    pub(crate) def: ElementDef,
}

impl Default for Portal {
    fn default() -> Self {
        Self::new()
    }
}

impl Portal {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::Portal),
        }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
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

impl From<Portal> for ElementDef {
    fn from(val: Portal) -> Self {
        val.def
    }
}

/// React Native compatible TouchableOpacity component.
#[derive(Clone, Debug, Default)]
pub struct TouchableOpacity {
    pub(crate) pressable: Pressable,
}

impl TouchableOpacity {
    pub fn new() -> Self {
        let mut pressable = Pressable::new();
        pressable.def.props.push(Property::Opacity(1.0));
        Self { pressable }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.pressable = self.pressable.style(style);
        self
    }

    pub fn active_opacity(mut self, opacity: f32) -> Self {
        self.pressable.def.props.push(Property::Opacity(opacity));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.pressable = self.pressable.disabled(disabled);
        self
    }

    pub fn on_press(mut self) -> Self {
        self.pressable = self.pressable.on_press();
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.pressable = self.pressable.test_id(id);
        self
    }

    pub fn child(mut self, child: impl Into<ElementDef>) -> Self {
        self.pressable = self.pressable.child(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ElementDef>>) -> Self {
        self.pressable = self.pressable.children(children);
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.pressable = self.pressable.key(key);
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.pressable.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.pressable.build_with_id(id, allocator)
    }
}

impl From<TouchableOpacity> for ElementDef {
    fn from(val: TouchableOpacity) -> Self {
        val.pressable.def
    }
}

/// React Native compatible TouchableHighlight component.
#[derive(Clone, Debug, Default)]
pub struct TouchableHighlight {
    pub(crate) pressable: Pressable,
}

impl TouchableHighlight {
    pub fn new() -> Self {
        Self {
            pressable: Pressable::new(),
        }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.pressable = self.pressable.style(style);
        self
    }

    pub fn underlay_color(mut self, color: u32) -> Self {
        self.pressable = self.pressable.ripple_color(color);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.pressable = self.pressable.disabled(disabled);
        self
    }

    pub fn on_press(mut self) -> Self {
        self.pressable = self.pressable.on_press();
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.pressable = self.pressable.test_id(id);
        self
    }

    pub fn child(mut self, child: impl Into<ElementDef>) -> Self {
        self.pressable = self.pressable.child(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ElementDef>>) -> Self {
        self.pressable = self.pressable.children(children);
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.pressable = self.pressable.key(key);
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.pressable.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.pressable.build_with_id(id, allocator)
    }
}

impl From<TouchableHighlight> for ElementDef {
    fn from(val: TouchableHighlight) -> Self {
        val.pressable.def
    }
}

/// React Native compatible TouchableWithoutFeedback component.
#[derive(Clone, Debug, Default)]
pub struct TouchableWithoutFeedback {
    pub(crate) pressable: Pressable,
}

impl TouchableWithoutFeedback {
    pub fn new() -> Self {
        Self {
            pressable: Pressable::new(),
        }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.pressable = self.pressable.style(style);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.pressable = self.pressable.disabled(disabled);
        self
    }

    pub fn on_press(mut self) -> Self {
        self.pressable = self.pressable.on_press();
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.pressable = self.pressable.test_id(id);
        self
    }

    pub fn child(mut self, child: impl Into<ElementDef>) -> Self {
        self.pressable = self.pressable.child(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ElementDef>>) -> Self {
        self.pressable = self.pressable.children(children);
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.pressable = self.pressable.key(key);
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.pressable.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.pressable.build_with_id(id, allocator)
    }
}

impl From<TouchableWithoutFeedback> for ElementDef {
    fn from(val: TouchableWithoutFeedback) -> Self {
        val.pressable.def
    }
}

/// Typical safe area insets structure.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EdgeInsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// Hook returning system safe area insets (status bar, navigation bar, display cutouts).
pub fn use_safe_area_insets() -> EdgeInsets {
    EdgeInsets {
        top: 24.0,
        right: 0.0,
        bottom: 16.0,
        left: 0.0,
    }
}

/// Overlay layout container stacking children along the Z-axis.
#[derive(Clone, Debug)]
pub struct Stack {
    pub(crate) def: ElementDef,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::Stack),
        }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.def.props.extend(style.to_properties());
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

impl From<Stack> for ElementDef {
    fn from(val: Stack) -> Self {
        val.def
    }
}

pub type ZStack = Stack;

/// Flexible spacer expanding to fill available space in flex containers.
#[derive(Clone, Debug)]
pub struct Spacer {
    pub(crate) def: ElementDef,
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::Spacer),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.def.props.push(Property::Spacing(size));
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl From<Spacer> for ElementDef {
    fn from(val: Spacer) -> Self {
        val.def
    }
}

/// Visual line separator between elements.
#[derive(Clone, Debug)]
pub struct Divider {
    pub(crate) def: ElementDef,
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl Divider {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::Divider),
        }
    }

    pub fn color(mut self, color: u32) -> Self {
        self.def.props.push(Property::BackgroundColor(color));
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.def.props.push(Property::BorderWidth {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: thickness,
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

impl From<Divider> for ElementDef {
    fn from(val: Divider) -> Self {
        val.def
    }
}

/// Center layout container that centers its child along both axes.
#[derive(Clone, Debug)]
pub struct Center {
    pub(crate) def: ElementDef,
}

impl Default for Center {
    fn default() -> Self {
        Self::new()
    }
}

impl Center {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::Center),
        }
    }

    pub fn child(mut self, child: impl Into<ElementDef>) -> Self {
        self.def.children.push(child.into());
        self
    }

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl From<Center> for ElementDef {
    fn from(val: Center) -> Self {
        val.def
    }
}

/// Multi-column grid container.
#[derive(Clone, Debug)]
pub struct Grid {
    pub(crate) def: ElementDef,
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Grid {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::Grid),
        }
    }

    pub fn columns(mut self, cols: u32) -> Self {
        self.def.props.push(Property::GridColumns(cols));
        self
    }

    pub fn spacing(mut self, s: f32) -> Self {
        self.def.props.push(Property::Spacing(s));
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

    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        self.def.build(allocator)
    }

    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        self.def.build_with_id(id, allocator)
    }
}

impl From<Grid> for ElementDef {
    fn from(val: Grid) -> Self {
        val.def
    }
}

/// Binary checkbox control.
#[derive(Clone, Debug)]
pub struct Checkbox {
    pub(crate) def: ElementDef,
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Checkbox {
    pub fn new() -> Self {
        let mut def = ElementDef::new(ViewType::Checkbox);
        def.event_listeners.push(EventType::ValueChange);
        Self { def }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.def.props.push(Property::Checked(checked));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.def.props.push(Property::Enabled(!disabled));
        self
    }

    pub fn on_value_change(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::ValueChange) {
            self.def.event_listeners.push(EventType::ValueChange);
        }
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
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

impl From<Checkbox> for ElementDef {
    fn from(val: Checkbox) -> Self {
        val.def
    }
}

/// Continuous or discrete slider control.
#[derive(Clone, Debug)]
pub struct Slider {
    pub(crate) def: ElementDef,
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}

impl Slider {
    pub fn new() -> Self {
        let mut def = ElementDef::new(ViewType::Slider);
        def.event_listeners.push(EventType::SliderChange);
        Self { def }
    }

    pub fn value(mut self, val: f32) -> Self {
        self.def.props.push(Property::SliderValue(val));
        self
    }

    pub fn range(mut self, min: f32, max: f32, step: f32) -> Self {
        self.def
            .props
            .push(Property::SliderRange { min, max, step });
        self
    }

    pub fn on_slider_change(mut self) -> Self {
        if !self.def.event_listeners.contains(&EventType::SliderChange) {
            self.def.event_listeners.push(EventType::SliderChange);
        }
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
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

impl From<Slider> for ElementDef {
    fn from(val: Slider) -> Self {
        val.def
    }
}

/// Horizontal progress bar indicator.
#[derive(Clone, Debug)]
pub struct ProgressBar {
    pub(crate) def: ElementDef,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressBar {
    pub fn new() -> Self {
        Self {
            def: ElementDef::new(ViewType::ProgressBar),
        }
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.def.props.push(Property::Progress(progress));
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.def.props.push(Property::IndicatorColor(color));
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
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

impl From<ProgressBar> for ElementDef {
    fn from(val: ProgressBar) -> Self {
        val.def
    }
}

/// Small status or counter badge tag.
#[derive(Clone, Debug)]
pub struct Badge {
    pub(crate) def: ElementDef,
}

impl Default for Badge {
    fn default() -> Self {
        Self::new("")
    }
}

impl Badge {
    pub fn new(content: impl Into<String>) -> Self {
        let mut def = ElementDef::new(ViewType::Badge);
        def.props.push(Property::BadgeContent(content.into()));
        Self { def }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.def.props.push(Property::BadgeContent(content.into()));
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

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
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

impl From<Badge> for ElementDef {
    fn from(val: Badge) -> Self {
        val.def
    }
}

/// Smooth linear gradient background container.
#[derive(Clone, Debug)]
pub struct LinearGradient {
    pub(crate) def: ElementDef,
}

impl Default for LinearGradient {
    fn default() -> Self {
        Self::new(0xFF2563EB, 0xFF9333EA)
    }
}

impl LinearGradient {
    pub fn new(start_color: u32, end_color: u32) -> Self {
        let mut def = ElementDef::new(ViewType::LinearGradient);
        def.props.push(Property::GradientColors {
            start_color,
            end_color,
            angle_deg: 90.0,
        });
        Self { def }
    }

    pub fn colors(mut self, start_color: u32, end_color: u32) -> Self {
        self.def.props.push(Property::GradientColors {
            start_color,
            end_color,
            angle_deg: 90.0,
        });
        self
    }

    pub fn angle(mut self, angle_deg: f32) -> Self {
        if let Some(Property::GradientColors { angle_deg: a, .. }) = self
            .def
            .props
            .iter_mut()
            .find(|p| matches!(p, Property::GradientColors { .. }))
        {
            *a = angle_deg;
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

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
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

impl From<LinearGradient> for ElementDef {
    fn from(val: LinearGradient) -> Self {
        val.def
    }
}

/// Vector icon display element.
#[derive(Clone, Debug)]
pub struct Icon {
    pub(crate) def: ElementDef,
}

impl Default for Icon {
    fn default() -> Self {
        Self::new("help-circle")
    }
}

impl Icon {
    pub fn new(name: impl Into<String>) -> Self {
        let mut def = ElementDef::new(ViewType::Icon);
        def.props.push(Property::IconName(name.into()));
        def.props.push(Property::IconSize(24.0));
        Self { def }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.def.props.push(Property::IconSize(size));
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.def.props.push(Property::TintColor(color));
        self
    }

    pub fn test_id(mut self, id: impl Into<String>) -> Self {
        self.def.props.push(Property::TestId(id.into()));
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

impl From<Icon> for ElementDef {
    fn from(val: Icon) -> Self {
        val.def
    }
}
