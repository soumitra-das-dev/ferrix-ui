//! Router state management, navigation stack, history, and serialization.

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::manifest::{RouteManifest, RouteMatch};
use ferrix_reactivity::Signal;

/// Serializable snapshot of a single navigation stack entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StackEntry {
    pub id: u64,
    pub pathname: String,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub pattern: String,
    pub component_name: String,
}

/// Serializable state snapshot for surviving Android Activity recreation and process death.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouterSnapshot {
    pub stack: Vec<StackEntry>,
    pub history: Vec<String>,
    pub active_index: usize,
}

struct InnerRouterState {
    manifest: RouteManifest,
    stack: Vec<RouteMatch>,
    history: Vec<String>,
}

/// Reactive router state owning the navigation stack and reactive inspection signals.
#[derive(Clone)]
pub struct RouterState {
    inner: Rc<RefCell<InnerRouterState>>,
    pathname_signal: Signal<String>,
    segments_signal: Signal<Vec<String>>,
    local_params_signal: Signal<HashMap<String, String>>,
    global_params_signal: Signal<HashMap<String, String>>,
    can_go_back_signal: Signal<bool>,
    is_focused_signal: Signal<bool>,
}

impl RouterState {
    /// Creates a new `RouterState` initialized to the given `initial_url` using the provided manifest.
    pub fn new(manifest: RouteManifest, initial_url: &str) -> Self {
        let initial_match = manifest.match_url(initial_url);
        let pathname = initial_match.pathname.clone();
        let segments = Self::extract_segments(&pathname);
        let params = initial_match.params.clone();
        let query = initial_match.query.clone();

        let inner = InnerRouterState {
            manifest,
            stack: vec![initial_match],
            history: vec![pathname.clone()],
        };

        Self {
            inner: Rc::new(RefCell::new(inner)),
            pathname_signal: Signal::new(pathname),
            segments_signal: Signal::new(segments),
            local_params_signal: Signal::new(params),
            global_params_signal: Signal::new(query),
            can_go_back_signal: Signal::new(false),
            is_focused_signal: Signal::new(true),
        }
    }

    fn extract_segments(pathname: &str) -> Vec<String> {
        let clean = pathname.trim_matches('/');
        if clean.is_empty() {
            Vec::new()
        } else {
            clean.split('/').map(|s| s.to_string()).collect()
        }
    }

    fn sync_signals(&self) {
        let inner = self.inner.borrow();
        if let Some(active) = inner.stack.last() {
            let pathname = active.pathname.clone();
            let segments = Self::extract_segments(&pathname);
            let params = active.params.clone();
            let query = active.query.clone();
            let can_back = inner.stack.len() > 1;

            self.pathname_signal.set(pathname);
            self.segments_signal.set(segments);
            self.local_params_signal.set(params);
            self.global_params_signal.set(query);
            self.can_go_back_signal.set(can_back);
            self.is_focused_signal.set(true);
        }
    }

    /// Pushes a new route onto the stack.
    pub fn push(&self, href: &str) {
        let mut inner = self.inner.borrow_mut();
        let matched = inner.manifest.match_url(href);
        inner.history.push(matched.pathname.clone());
        inner.stack.push(matched);
        drop(inner);
        self.sync_signals();
    }

    /// Replaces the current top of the stack with the new route.
    pub fn replace(&self, href: &str) {
        let mut inner = self.inner.borrow_mut();
        let matched = inner.manifest.match_url(href);
        if !inner.stack.is_empty() {
            inner.stack.pop();
        }
        inner.history.push(matched.pathname.clone());
        inner.stack.push(matched);
        drop(inner);
        self.sync_signals();
    }

    /// Pops the current route from the stack if possible. Returns `true` if popped.
    pub fn back(&self) -> bool {
        let mut inner = self.inner.borrow_mut();
        if inner.stack.len() > 1 {
            inner.stack.pop();
            drop(inner);
            self.sync_signals();
            true
        } else {
            false
        }
    }

    /// Returns whether the stack has previous screens to pop to.
    pub fn can_go_back(&self) -> bool {
        self.can_go_back_signal.get()
    }

    /// Reactive signal indicating whether the router can go back.
    pub fn can_go_back_signal(&self) -> Signal<bool> {
        self.can_go_back_signal.clone()
    }

    /// Navigates to a route: pushes if different, otherwise updates query.
    pub fn navigate(&self, href: &str) {
        if href == self.pathname_signal.get() {
            return;
        }
        self.push(href);
    }

    /// Triggers an error escalation, pushing the matching `+error` screen onto the stack.
    pub fn trigger_error(&self, error_message: &str) {
        let mut inner = self.inner.borrow_mut();
        let current_path = inner
            .stack
            .last()
            .map(|m| m.pathname.clone())
            .unwrap_or_else(|| "/".to_string());
        let matched = inner
            .manifest
            .match_error_route(&current_path, error_message);
        inner.stack.push(matched);
        drop(inner);
        self.sync_signals();
    }

    /// Returns whether the active top screen is an error screen.
    pub fn is_error(&self) -> bool {
        self.inner
            .borrow()
            .stack
            .last()
            .map(|m| m.route.is_error)
            .unwrap_or(false)
    }

    /// Reactive signal for the current pathname.
    pub fn pathname_signal(&self) -> Signal<String> {
        self.pathname_signal.clone()
    }

    /// Reactive signal for the active segments.
    pub fn segments_signal(&self) -> Signal<Vec<String>> {
        self.segments_signal.clone()
    }

    /// Reactive signal for local search params (from dynamic [id] segments).
    pub fn local_params_signal(&self) -> Signal<HashMap<String, String>> {
        self.local_params_signal.clone()
    }

    /// Convenience snapshot of current local route params.
    pub fn local_params(&self) -> HashMap<String, String> {
        self.local_params_signal.get()
    }

    /// Reactive signal for global query parameters.
    pub fn global_params_signal(&self) -> Signal<HashMap<String, String>> {
        self.global_params_signal.clone()
    }

    /// Convenience snapshot of current global query params.
    pub fn global_params(&self) -> HashMap<String, String> {
        self.global_params_signal.get()
    }

    /// Reactive signal indicating whether the current screen is focused.
    pub fn is_focused_signal(&self) -> Signal<bool> {
        self.is_focused_signal.clone()
    }

    /// Sets the focus state of the router.
    pub fn set_focused(&self, focused: bool) {
        self.is_focused_signal.set(focused);
    }

    /// Current active pathname value.
    pub fn current_pathname(&self) -> String {
        self.pathname_signal.get()
    }

    /// Active route match reference.
    pub fn current_match(&self) -> RouteMatch {
        self.inner.borrow().stack.last().cloned().unwrap()
    }

    /// Current navigation stack depth.
    pub fn stack_len(&self) -> usize {
        self.inner.borrow().stack.len()
    }

    /// Serializes the active router state into a snapshot.
    pub fn snapshot(&self) -> RouterSnapshot {
        let inner = self.inner.borrow();
        let stack = inner
            .stack
            .iter()
            .enumerate()
            .map(|(i, m)| StackEntry {
                id: i as u64,
                pathname: m.pathname.clone(),
                params: m.params.clone(),
                query: m.query.clone(),
                pattern: m.route.pattern.clone(),
                component_name: m.route.component_name.clone(),
            })
            .collect();

        RouterSnapshot {
            stack,
            history: inner.history.clone(),
            active_index: inner.stack.len().saturating_sub(1),
        }
    }

    /// Restores router state from a snapshot.
    pub fn restore(&self, snapshot: RouterSnapshot) {
        let mut inner = self.inner.borrow_mut();
        inner.stack.clear();
        for entry in &snapshot.stack {
            let matched = inner.manifest.match_url(&entry.pathname);
            inner.stack.push(matched);
        }
        if inner.stack.is_empty() {
            let fallback = inner.manifest.match_url("/");
            inner.stack.push(fallback);
        }
        inner.history = snapshot.history;
        drop(inner);
        self.sync_signals();
    }
}
