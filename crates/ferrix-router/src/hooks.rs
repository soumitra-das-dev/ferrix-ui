//! Reactive routing hooks and handle.

use crate::state::RouterState;
use ferrix_reactivity::Signal;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static CURRENT_ROUTER: RefCell<Option<RouterState>> = const { RefCell::new(None) };
}

/// Sets the active router instance for the current thread runtime.
pub fn set_current_router(router: RouterState) {
    CURRENT_ROUTER.with(|cell| {
        *cell.borrow_mut() = Some(router);
    });
}

/// Retrieves the active router instance for the current thread runtime.
pub fn get_current_router() -> Option<RouterState> {
    CURRENT_ROUTER.with(|cell| cell.borrow().clone())
}

/// Handle exposing imperative navigation actions to components.
#[derive(Clone)]
pub struct RouterHandle {
    state: RouterState,
}

impl RouterHandle {
    pub fn new(state: RouterState) -> Self {
        Self { state }
    }

    /// Pushes a new screen onto the stack.
    pub fn push(&self, href: &str) {
        self.state.push(href);
    }

    /// Replaces the current top of the stack.
    pub fn replace(&self, href: &str) {
        self.state.replace(href);
    }

    /// Pops the active screen, returning true if back navigation succeeded.
    pub fn back(&self) -> bool {
        self.state.back()
    }

    /// Navigates to a target URL.
    pub fn navigate(&self, href: &str) {
        self.state.navigate(href);
    }

    /// Checks if back navigation is available.
    pub fn can_go_back(&self) -> bool {
        self.state.can_go_back()
    }
}

/// Hook returning the imperative `RouterHandle`.
pub fn use_router() -> RouterHandle {
    let state =
        get_current_router().expect("use_router must be called within an active Router context");
    RouterHandle::new(state)
}

/// Hook returning a reactive signal for the current pathname (e.g. `"/users/42"`).
pub fn use_pathname() -> Signal<String> {
    let state =
        get_current_router().expect("use_pathname must be called within an active Router context");
    state.pathname_signal()
}

/// Hook returning a reactive signal for the active route segments (e.g. `["users", "42"]`).
pub fn use_segments() -> Signal<Vec<String>> {
    let state =
        get_current_router().expect("use_segments must be called within an active Router context");
    state.segments_signal()
}

/// Hook returning a reactive signal for local parameters extracted from dynamic segments (e.g. `{ "id": "42" }`).
pub fn use_local_search_params() -> Signal<HashMap<String, String>> {
    let state = get_current_router()
        .expect("use_local_search_params must be called within an active Router context");
    state.local_params_signal()
}

/// Hook returning a reactive signal for global query string parameters.
pub fn use_global_search_params() -> Signal<HashMap<String, String>> {
    let state = get_current_router()
        .expect("use_global_search_params must be called within an active Router context");
    state.global_params_signal()
}

/// Hook returning a reactive signal indicating whether this screen is currently focused.
pub fn use_is_focused() -> Signal<bool> {
    let state = get_current_router()
        .expect("use_is_focused must be called within an active Router context");
    state.is_focused_signal()
}

/// Hook returning a reactive signal indicating whether back navigation is possible.
pub fn use_can_go_back() -> Signal<bool> {
    let state = get_current_router()
        .expect("use_can_go_back must be called within an active Router context");
    state.can_go_back_signal()
}

/// Executes a focus effect whenever the screen gains focus.
pub fn use_focus_effect<F>(effect: F)
where
    F: Fn() -> Option<Box<dyn FnOnce()>> + 'static,
{
    let is_focused = use_is_focused();
    // In reactive runtime, an effect tracks `is_focused.get()`
    ferrix_reactivity::Effect::new(move || {
        if is_focused.get() {
            let cleanup = effect();
            Some(Box::new(move || {
                if let Some(c) = cleanup {
                    c();
                }
            }))
        } else {
            None
        }
    });
}
