//! Headless test renderer and execution harness.

use std::collections::HashMap;

use ferrix_core::prelude::*;
use ferrix_renderer_protocol::{EventType, NodeId, Patch, PatchBatch};

use crate::node::TestNode;
use crate::query::{QueryEngine, QueryError};
use crate::snapshot::format_snapshot;

type EventHandler = Box<dyn FnMut()>;
type TextChangeHandler = Box<dyn FnMut(String)>;
type ValueChangeHandler = Box<dyn FnMut(bool)>;
type RenderFn = Box<dyn FnMut(&mut ComponentContext) -> Element>;

/// Headless component test harness.
pub struct TestRenderer {
    cx: ComponentContext,
    render_fn: Option<RenderFn>,
    current_tree: Element,
    mount_patches: PatchBatch,
    recent_patches: PatchBatch,
    patch_history: Vec<Patch>,
    event_handlers: HashMap<(NodeId, EventType), EventHandler>,
    text_handlers: HashMap<NodeId, TextChangeHandler>,
    value_handlers: HashMap<NodeId, ValueChangeHandler>,
}

impl TestRenderer {
    /// Creates a headless renderer from an initial element.
    pub fn from_element(element: Element) -> Self {
        let mount_patches = generate_mount_patches(&element);
        Self {
            cx: ComponentContext::with_start_id(element.id.raw() + 1),
            render_fn: None,
            current_tree: element,
            mount_patches: mount_patches.clone(),
            recent_patches: mount_patches.clone(),
            patch_history: mount_patches.patches,
            event_handlers: HashMap::new(),
            text_handlers: HashMap::new(),
            value_handlers: HashMap::new(),
        }
    }

    /// Creates a headless renderer from a functional component render closure.
    pub fn new<F: FnMut(&mut ComponentContext) -> Element + 'static>(mut render_fn: F) -> Self {
        let mut cx = ComponentContext::with_start_id(2);
        cx.prepare_render();
        let initial_tree = render_fn(&mut cx);
        let mount_patches = generate_mount_patches(&initial_tree);

        Self {
            cx,
            render_fn: Some(Box::new(render_fn)),
            current_tree: initial_tree,
            mount_patches: mount_patches.clone(),
            recent_patches: mount_patches.clone(),
            patch_history: mount_patches.patches,
            event_handlers: HashMap::new(),
            text_handlers: HashMap::new(),
            value_handlers: HashMap::new(),
        }
    }

    /// Mutable reference to the component context (e.g. for inspection).
    pub fn context_mut(&mut self) -> &mut ComponentContext {
        &mut self.cx
    }

    /// Registers a custom event handler for a given node ID and event type.
    pub fn on_event(
        &mut self,
        node_id: NodeId,
        event_type: EventType,
        handler: impl FnMut() + 'static,
    ) {
        self.event_handlers
            .insert((node_id, event_type), Box::new(handler));
    }

    /// Registers a Click/press event handler on a node.
    pub fn on_click(&mut self, node_id: NodeId, handler: impl FnMut() + 'static) {
        self.on_event(node_id, EventType::Click, handler);
    }

    /// Registers a text change handler on a TextInput node.
    pub fn on_text_change(&mut self, node_id: NodeId, handler: impl FnMut(String) + 'static) {
        self.text_handlers.insert(node_id, Box::new(handler));
    }

    /// Registers a value change handler on a Switch/Checkbox node.
    pub fn on_value_change(&mut self, node_id: NodeId, handler: impl FnMut(bool) + 'static) {
        self.value_handlers.insert(node_id, Box::new(handler));
    }

    /// Dispatches a synthetic event on a node, triggers registered handlers, and re-renders if a render function is provided.
    pub fn fire_event(&mut self, node_id: NodeId, event_type: EventType) {
        if let Some(mut handler) = self.event_handlers.remove(&(node_id, event_type)) {
            handler();
            self.event_handlers.insert((node_id, event_type), handler);
        }

        self.rerender();
    }

    /// Simulates a press/click event on a node.
    pub fn press(&mut self, node_id: NodeId) {
        self.fire_event(node_id, EventType::Click);
    }

    /// Simulates changing text on a TextInput node.
    pub fn change_text(&mut self, node_id: NodeId, new_text: impl Into<String>) {
        let text = new_text.into();
        if let Some(mut handler) = self.text_handlers.remove(&node_id) {
            handler(text.clone());
            self.text_handlers.insert(node_id, handler);
        }

        self.fire_event(node_id, EventType::TextChanged);
    }

    /// Simulates toggling a boolean value on a Switch or Checkbox node.
    pub fn toggle_value(&mut self, node_id: NodeId, value: bool) {
        if let Some(mut handler) = self.value_handlers.remove(&node_id) {
            handler(value);
            self.value_handlers.insert(node_id, handler);
        }

        self.fire_event(node_id, EventType::ValueChange);
    }

    /// Re-renders the component, performs tree diffing, and records emitted patches.
    pub fn rerender(&mut self) {
        if let Some(mut render_fn) = self.render_fn.take() {
            self.cx.prepare_render();
            let next_tree = render_fn(&mut self.cx);
            let diff = diff_elements(&self.current_tree, &next_tree);

            self.recent_patches = diff.clone();
            self.patch_history.extend(diff.patches);
            self.current_tree = next_tree;
            self.render_fn = Some(render_fn);
        }
    }

    /// Root virtual TestNode of the current tree.
    pub fn root_node(&self) -> TestNode {
        TestNode::from_element(&self.current_tree)
    }

    /// Current Element tree.
    pub fn tree(&self) -> &Element {
        &self.current_tree
    }

    /// Initial mount patches emitted when component was first rendered.
    pub fn mount_patches(&self) -> &PatchBatch {
        &self.mount_patches
    }

    /// Patches emitted during the most recent render or event cycle.
    pub fn recent_patches(&self) -> &PatchBatch {
        &self.recent_patches
    }

    /// Complete history of all patches emitted by this renderer.
    pub fn all_patches(&self) -> &[Patch] {
        &self.patch_history
    }

    /// Number of patches emitted during the most recent update.
    pub fn patch_count(&self) -> usize {
        self.recent_patches.len()
    }

    /// Queries exactly one node containing `text`.
    pub fn get_by_text(&self, text: &str) -> Result<TestNode, QueryError> {
        QueryEngine::get_by_text(&self.root_node(), text)
    }

    /// Queries an optional node containing `text`.
    pub fn query_by_text(&self, text: &str) -> Option<TestNode> {
        QueryEngine::find_all_by_text(&self.root_node(), text)
            .into_iter()
            .next()
    }

    /// Queries all nodes containing `text`.
    pub fn find_all_by_text(&self, text: &str) -> Vec<TestNode> {
        QueryEngine::find_all_by_text(&self.root_node(), text)
    }

    /// Queries exactly one node matching `test_id`.
    pub fn get_by_test_id(&self, test_id: &str) -> Result<TestNode, QueryError> {
        QueryEngine::get_by_test_id(&self.root_node(), test_id)
    }

    /// Queries an optional node matching `test_id`.
    pub fn query_by_test_id(&self, test_id: &str) -> Option<TestNode> {
        QueryEngine::find_all_by_test_id(&self.root_node(), test_id)
            .into_iter()
            .next()
    }

    /// Queries exactly one node matching accessibility `role`.
    pub fn get_by_role(&self, role: &str) -> Result<TestNode, QueryError> {
        QueryEngine::get_by_role(&self.root_node(), role)
    }

    /// Queries all nodes matching accessibility `role`.
    pub fn find_all_by_role(&self, role: &str) -> Vec<TestNode> {
        QueryEngine::find_all_by_role(&self.root_node(), role)
    }

    /// Queries exactly one node matching accessibility `label`.
    pub fn get_by_label(&self, label: &str) -> Result<TestNode, QueryError> {
        QueryEngine::get_by_label(&self.root_node(), label)
    }

    /// Converts the current tree into an XML/JSX snapshot string.
    pub fn to_snapshot(&self) -> String {
        format_snapshot(&self.current_tree)
    }

    /// Asserts that exactly one patch was produced in the recent update, and verifies it.
    pub fn assert_single_patch<F: FnOnce(&Patch)>(&self, f: F) {
        assert_eq!(
            self.recent_patches.len(),
            1,
            "Expected exactly 1 patch, but found {}",
            self.recent_patches.len()
        );
        f(&self.recent_patches.patches[0]);
    }
}
