//! Virtual test node representation and property accessors.

use ferrix_core::Element;
use ferrix_renderer_protocol::{NodeId, Property, ViewType, Visibility};

/// A snapshot of an element node in the headless test tree.
#[derive(Clone, Debug, PartialEq)]
pub struct TestNode {
    pub id: NodeId,
    pub view_type: ViewType,
    pub text: Option<String>,
    pub props: Vec<Property>,
    pub children: Vec<TestNode>,
    pub visibility: Visibility,
}

impl TestNode {
    /// Creates a TestNode from an Element reference.
    pub fn from_element(element: &Element) -> Self {
        Self {
            id: element.id,
            view_type: element.view_type,
            text: element.text.clone(),
            props: element.props.clone(),
            children: element.children.iter().map(Self::from_element).collect(),
            visibility: element.visibility,
        }
    }

    /// Stable node identifier.
    #[inline]
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Underlying native view type.
    #[inline]
    pub fn view_type(&self) -> ViewType {
        self.view_type
    }

    /// Text content if present.
    #[inline]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// True if visibility is Visible.
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.visibility == Visibility::Visible
    }

    /// Accessibility role string if set.
    pub fn role(&self) -> Option<&str> {
        self.props.iter().find_map(|p| match p {
            Property::AccessibilityRole(role) => Some(role.as_str()),
            _ => None,
        })
    }

    /// Accessibility label or content description if set.
    pub fn label(&self) -> Option<&str> {
        self.props.iter().find_map(|p| match p {
            Property::AccessibilityLabel(label) => Some(label.as_str()),
            Property::ContentDescription(desc) => Some(desc.as_str()),
            _ => None,
        })
    }

    /// Test ID string if set.
    pub fn test_id(&self) -> Option<&str> {
        self.props.iter().find_map(|p| match p {
            Property::TestId(id) => Some(id.as_str()),
            _ => None,
        })
    }

    /// Checks if a property matching predicate `f` exists on the node.
    pub fn has_prop<F: Fn(&Property) -> bool>(&self, f: F) -> bool {
        self.props.iter().any(f)
    }

    /// Recursively flattens node and all descendants.
    pub fn flatten(&self) -> Vec<TestNode> {
        let mut list = vec![self.clone()];
        for child in &self.children {
            list.extend(child.flatten());
        }
        list
    }
}
