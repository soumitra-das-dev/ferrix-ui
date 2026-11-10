//! Core Element model representing the declarative UI tree.

use crate::node_id::NodeIdAllocator;
use ferrix_renderer_protocol::{EventType, NodeId, Property, ViewType, Visibility};

/// Declarative, immutable representation of a UI element in Ferrix.
#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    /// Stable identifier for this node.
    pub id: NodeId,
    /// Native view type (Column, Row, Text, Button, etc.).
    pub view_type: ViewType,
    /// Visual and styling properties.
    pub props: Vec<Property>,
    /// Text content (for Text and Button nodes).
    pub text: Option<String>,
    /// Child elements (for container nodes like Column, Row).
    pub children: Vec<Element>,
    /// Subscribed event types (e.g. Click).
    pub event_listeners: Vec<EventType>,
    /// View visibility (Visible, Invisible, Gone).
    pub visibility: Visibility,
    /// Optional key for keyed list reconciliation.
    pub key: Option<String>,
}

impl Element {
    /// Creates a new element with allocated ID.
    pub fn new(id: NodeId, view_type: ViewType) -> Self {
        Self {
            id,
            view_type,
            props: Vec::new(),
            text: None,
            children: Vec::new(),
            event_listeners: Vec::new(),
            visibility: Visibility::Visible,
            key: None,
        }
    }

    /// Attaches child elements.
    pub fn with_children(mut self, children: Vec<Element>) -> Self {
        self.children = children;
        self
    }

    /// Sets text content.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Adds a property.
    pub fn with_prop(mut self, prop: Property) -> Self {
        self.props.push(prop);
        self
    }

    /// Adds multiple properties.
    pub fn with_props(mut self, props: Vec<Property>) -> Self {
        self.props.extend(props);
        self
    }

    /// Subscribes to an event type.
    pub fn with_event(mut self, event: EventType) -> Self {
        if !self.event_listeners.contains(&event) {
            self.event_listeners.push(event);
        }
        self
    }

    /// Sets the reconciliation key.
    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Sets node visibility.
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Finds an element by its NodeId in this subtree.
    pub fn find(&self, target_id: NodeId) -> Option<&Element> {
        if self.id == target_id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find(target_id) {
                return Some(found);
            }
        }
        None
    }

    /// Counts all nodes in this subtree including self.
    pub fn node_count(&self) -> usize {
        1 + self.children.iter().map(|c| c.node_count()).sum::<usize>()
    }
}

/// Unresolved element definition before NodeId allocation.
#[derive(Clone, Debug, PartialEq)]
pub struct ElementDef {
    pub view_type: ViewType,
    pub props: Vec<Property>,
    pub text: Option<String>,
    pub children: Vec<ElementDef>,
    pub event_listeners: Vec<EventType>,
    pub visibility: Visibility,
    pub key: Option<String>,
}

impl ElementDef {
    pub fn new(view_type: ViewType) -> Self {
        Self {
            view_type,
            props: Vec::new(),
            text: None,
            children: Vec::new(),
            event_listeners: Vec::new(),
            visibility: Visibility::Visible,
            key: None,
        }
    }

    /// Resolves this definition into a concrete Element tree with allocated NodeIds.
    pub fn build(self, allocator: &NodeIdAllocator) -> Element {
        let id = allocator.allocate();
        self.build_with_id(id, allocator)
    }

    /// Resolves this definition into a concrete Element tree with a designated root ID.
    pub fn build_with_id(self, id: NodeId, allocator: &NodeIdAllocator) -> Element {
        let children = self
            .children
            .into_iter()
            .map(|child| child.build(allocator))
            .collect();

        Element {
            id,
            view_type: self.view_type,
            props: self.props,
            text: self.text,
            children,
            event_listeners: self.event_listeners,
            visibility: self.visibility,
            key: self.key,
        }
    }
}
