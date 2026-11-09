//! Ergonomic declarative UI component builders matching the FSX specification.

use crate::element::ElementDef;
use crate::node_id::NodeIdAllocator;
use crate::Element;
use ferrix_renderer_protocol::{EventType, NodeId, Property, ViewType};

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
