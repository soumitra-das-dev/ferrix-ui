//! Reactive Style Resolver evaluating dynamic media queries, state pseudo-classes, and CSS variables.

use std::collections::HashMap;

use ferrix_renderer_protocol::{NodeId, Patch};

use crate::color::Color;
use crate::ir::StyleIr;
use crate::registry::StyleRegistry;

/// System color scheme preference.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ColorScheme {
    #[default]
    Light,
    Dark,
}

/// Window dimensions in density-independent pixels (dp).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowDimensions {
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub font_scale: f32,
}

impl Default for WindowDimensions {
    fn default() -> Self {
        Self {
            width: 360.0,
            height: 800.0,
            scale: 2.0,
            font_scale: 1.0,
        }
    }
}

/// Dynamic environment state influencing reactive styling.
#[derive(Clone, Debug, Default)]
pub struct EnvironmentState {
    pub color_scheme: ColorScheme,
    pub dimensions: WindowDimensions,
    pub reduced_motion: bool,
    pub variables: HashMap<String, u32>,
}

/// Ephemeral interaction state of an individual element.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ElementState {
    pub active: bool,
    pub focus: bool,
    pub disabled: bool,
    pub checked: bool,
}

/// Manages dynamic reactive style evaluation for mounted UI elements.
#[derive(Debug)]
pub struct ReactiveStyleResolver {
    registry: StyleRegistry,
    environment: EnvironmentState,
    node_styles: HashMap<NodeId, StyleIr>,
    node_states: HashMap<NodeId, ElementState>,
    applied_style_ids: HashMap<NodeId, u32>,
}

impl ReactiveStyleResolver {
    pub fn new() -> Self {
        Self {
            registry: StyleRegistry::new(),
            environment: EnvironmentState::default(),
            node_styles: HashMap::new(),
            node_states: HashMap::new(),
            applied_style_ids: HashMap::new(),
        }
    }

    pub fn with_environment(environment: EnvironmentState) -> Self {
        Self {
            registry: StyleRegistry::new(),
            environment,
            node_styles: HashMap::new(),
            node_states: HashMap::new(),
            applied_style_ids: HashMap::new(),
        }
    }

    pub fn registry(&self) -> &StyleRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut StyleRegistry {
        &mut self.registry
    }

    pub fn environment(&self) -> &EnvironmentState {
        &self.environment
    }

    /// Sets or updates a global CSS variable in the environment.
    pub fn set_variable(&mut self, name: impl Into<String>, color: u32) {
        self.environment.variables.insert(name.into(), color);
    }

    /// Registers a node with its declared `StyleIr`.
    /// Evaluates the initial effective style and returns the assigned `style_id`.
    pub fn register_node(&mut self, node_id: NodeId, style: StyleIr) -> (u32, Option<Patch>) {
        let element_state = self.node_states.get(&node_id).copied().unwrap_or_default();
        let resolved = self.resolve_effective_from_base(&style, element_state);
        self.node_styles.insert(node_id, style);

        let style_id = self.registry.intern(resolved);
        self.applied_style_ids.insert(node_id, style_id);

        let reg_patch = self.registry.take_registration_patch();
        (style_id, reg_patch)
    }

    /// Unregisters a node when destroyed.
    pub fn unregister_node(&mut self, node_id: NodeId) {
        self.node_styles.remove(&node_id);
        self.node_states.remove(&node_id);
        self.applied_style_ids.remove(&node_id);
    }

    /// Updates the interaction state of an element (e.g. `:active`, `:focus`, `:disabled`, `:checked`).
    /// Emits `Patch::SetStyle` only if the effective style changed; otherwise returns `None`.
    pub fn update_element_state(&mut self, node_id: NodeId, state: ElementState) -> Option<Patch> {
        let prev_state = self.node_states.insert(node_id, state);
        if prev_state == Some(state) {
            return None;
        }

        let resolved = self.resolve_effective_style(node_id, state);
        let new_style_id = self.registry.intern(resolved);

        let prev_style_id = self.applied_style_ids.get(&node_id).copied();
        if prev_style_id == Some(new_style_id) {
            return None;
        }

        self.applied_style_ids.insert(node_id, new_style_id);
        Some(Patch::SetStyle {
            node_id,
            style_id: new_style_id,
        })
    }

    /// Updates the global environment (e.g. theme switch, orientation, window resize).
    /// Re-evaluates all registered nodes and returns a batched list of `Patch`es.
    /// If no nodes changed style, returns an empty list.
    pub fn update_environment(&mut self, environment: EnvironmentState) -> Vec<Patch> {
        self.environment = environment;
        let mut patches = Vec::new();

        let node_ids: Vec<NodeId> = self.node_styles.keys().copied().collect();
        for node_id in node_ids {
            let state = self.node_states.get(&node_id).copied().unwrap_or_default();
            let resolved = self.resolve_effective_style(node_id, state);
            let new_style_id = self.registry.intern(resolved);

            let prev_style_id = self.applied_style_ids.get(&node_id).copied();
            if prev_style_id != Some(new_style_id) {
                self.applied_style_ids.insert(node_id, new_style_id);
                patches.push(Patch::SetStyle {
                    node_id,
                    style_id: new_style_id,
                });
            }
        }

        // Prepend any newly registered style definitions if interned
        if let Some(reg_patch) = self.registry.take_registration_patch() {
            patches.insert(0, reg_patch);
        }

        patches
    }

    /// Resolves the concrete effective `StyleIr` given the current environment and interaction state.
    pub fn resolve_effective_style(&self, node_id: NodeId, state: ElementState) -> StyleIr {
        match self.node_styles.get(&node_id) {
            Some(b) => self.resolve_effective_from_base(b, state),
            None => StyleIr::new(),
        }
    }

    /// Resolves effective style given a base `StyleIr` reference.
    pub fn resolve_effective_from_base(&self, base: &StyleIr, state: ElementState) -> StyleIr {
        let has_variants = base.sm.is_some()
            || base.md.is_some()
            || base.lg.is_some()
            || (base.dark.is_some() && self.environment.color_scheme == ColorScheme::Dark)
            || (state.focus && base.focus.is_some())
            || (state.active && base.active.is_some())
            || (state.checked && base.checked.is_some())
            || (state.disabled && base.disabled.is_some())
            || !base.variables.is_empty();

        if !has_variants {
            return base.clone();
        }

        let mut effective = base.clone();

        // 1. Responsive breakpoints
        let w = self.environment.dimensions.width;
        if w >= 640.0 {
            if let Some(ref sm) = base.sm {
                effective = effective.merge(sm);
            }
        }
        if w >= 768.0 {
            if let Some(ref md) = base.md {
                effective = effective.merge(md);
            }
        }
        if w >= 1024.0 {
            if let Some(ref lg) = base.lg {
                effective = effective.merge(lg);
            }
        }

        // 2. Dark mode variant
        if self.environment.color_scheme == ColorScheme::Dark {
            if let Some(ref dark) = base.dark {
                effective = effective.merge(dark);
            }
        }

        // 3. Focus state
        if state.focus {
            if let Some(ref focus) = base.focus {
                effective = effective.merge(focus);
            }
        }

        // 4. Active (pressed) state
        if state.active {
            if let Some(ref active) = base.active {
                effective = effective.merge(active);
            }
        }

        // 5. Checked state
        if state.checked {
            if let Some(ref checked) = base.checked {
                effective = effective.merge(checked);
            }
        }

        // 6. Disabled state
        if state.disabled {
            if let Some(ref disabled) = base.disabled {
                effective = effective.merge(disabled);
            }
        }

        // 7. Resolve CSS variables
        for (var_name, var_value) in &effective.variables {
            if let Some(&env_color) = self.environment.variables.get(var_name) {
                if var_name.contains("bg") || var_name.contains("background") {
                    effective.background_color = Some(env_color);
                } else if var_name.contains("text") || var_name.contains("color") {
                    effective.color = Some(env_color);
                } else if var_name.contains("border") {
                    effective.border_color = Some(env_color);
                }
            } else if let Some(parsed) = Color::parse(var_value) {
                if var_name.contains("bg") || var_name.contains("background") {
                    effective.background_color = Some(parsed);
                } else if var_name.contains("text") || var_name.contains("color") {
                    effective.color = Some(parsed);
                } else if var_name.contains("border") {
                    effective.border_color = Some(parsed);
                }
            }
        }

        effective
    }
}

impl Default for ReactiveStyleResolver {
    fn default() -> Self {
        Self::new()
    }
}
