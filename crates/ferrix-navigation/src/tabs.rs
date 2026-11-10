//! Native tab navigator state machine and tab switching.

use crate::options::TransitionAnimation;
use ferrix_renderer_protocol::{Command, CommandBatch, NodeId};

/// Tab configuration entry.
#[derive(Clone, Debug, PartialEq)]
pub struct TabEntry {
    pub screen_id: NodeId,
    pub title: String,
    pub icon: Option<String>,
}

/// Native tab navigator managing tab bar state and screen activations.
#[derive(Clone, Debug)]
pub struct TabNavigator {
    pub container_id: NodeId,
    tabs: Vec<TabEntry>,
    active_index: usize,
}

impl TabNavigator {
    /// Creates a new `TabNavigator` bound to a native container NodeId.
    pub fn new(container_id: NodeId) -> Self {
        Self {
            container_id,
            tabs: Vec::new(),
            active_index: 0,
        }
    }

    /// Adds a tab screen to the navigator.
    pub fn add_tab(&mut self, screen_id: NodeId, title: &str, icon: Option<&str>) {
        self.tabs.push(TabEntry {
            screen_id,
            title: title.to_string(),
            icon: icon.map(|s| s.to_string()),
        });
    }

    /// Switches the active tab, generating commands to activate the target screen.
    pub fn switch_tab(&mut self, index: usize) -> Option<CommandBatch> {
        if index >= self.tabs.len() || index == self.active_index {
            return None;
        }

        self.active_index = index;
        let target = &self.tabs[index];

        let mut batch = CommandBatch::new();
        batch.push(Command::Navigate {
            container_id: self.container_id,
            screen_id: target.screen_id,
            animation: TransitionAnimation::None.to_u8(),
        });

        Some(batch)
    }

    /// Returns the currently active tab index.
    pub fn active_index(&self) -> usize {
        self.active_index
    }

    /// Returns the currently active tab entry.
    pub fn active_tab(&self) -> Option<&TabEntry> {
        self.tabs.get(self.active_index)
    }

    /// Returns total number of tabs.
    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    /// Whether there are tabs registered.
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
}
