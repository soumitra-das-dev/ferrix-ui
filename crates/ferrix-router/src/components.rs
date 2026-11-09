//! Declarative router components and screen options.

use crate::hooks::{use_router, RouterHandle};
use ferrix_core::element::Element;
use ferrix_renderer_protocol::{NodeId, ViewType};

/// Presentation style for stack navigation screens.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StackPresentation {
    #[default]
    Card,
    Modal,
    TransparentModal,
}

/// Native transition animation for screen transitions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StackAnimation {
    #[default]
    Default,
    Fade,
    SlideFromRight,
    SlideFromBottom,
    None,
}

/// Navigation options configuring the native header and presentation of a screen.
#[derive(Clone, Debug, PartialEq)]
pub struct StackScreenOptions {
    pub title: Option<String>,
    pub header_shown: bool,
    pub presentation: StackPresentation,
    pub animation: StackAnimation,
}

impl Default for StackScreenOptions {
    fn default() -> Self {
        Self {
            title: None,
            header_shown: true,
            presentation: StackPresentation::Card,
            animation: StackAnimation::Default,
        }
    }
}

/// Declarative link component properties.
#[derive(Clone, Debug)]
pub struct LinkProps {
    pub href: String,
    pub replace: bool,
    pub children: Vec<Element>,
}

/// Creates a declarative `<Link>` element wrapping child components.
pub fn link(id: NodeId, props: LinkProps, _router: &RouterHandle) -> Element {
    let mut elem = Element::new(id, ViewType::Pressable);
    elem.children = props.children;
    // In Ferrix event dispatch, Press triggers navigation
    elem.event_listeners
        .push(ferrix_renderer_protocol::EventType::Click);
    elem
}

/// Immediately redirects navigation to `href` when rendered (e.g. in route guards).
pub fn redirect(href: &str) {
    let router = use_router();
    router.replace(href);
}
