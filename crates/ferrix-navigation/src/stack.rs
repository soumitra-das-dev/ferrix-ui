//! Native stack navigator state machine and command generator.

use crate::options::{ScreenOptions, TransitionAnimation};
use ferrix_renderer_protocol::{Command, CommandBatch, NodeId};

/// Entry in the navigation stack.
#[derive(Clone, Debug, PartialEq)]
pub struct ScreenEntry {
    pub screen_id: NodeId,
    pub route_name: String,
    pub options: ScreenOptions,
}

/// Native stack navigator managing screen hierarchy and generating platform commands.
#[derive(Clone, Debug)]
pub struct StackNavigator {
    pub container_id: NodeId,
    stack: Vec<ScreenEntry>,
}

impl StackNavigator {
    /// Creates a new `StackNavigator` bound to a native container NodeId.
    pub fn new(container_id: NodeId) -> Self {
        Self {
            container_id,
            stack: Vec::new(),
        }
    }

    /// Pushes a new screen onto the stack and generates commands for native transition.
    pub fn push(
        &mut self,
        screen_id: NodeId,
        route_name: &str,
        options: ScreenOptions,
    ) -> CommandBatch {
        let mut batch = CommandBatch::new();

        // 1. Configure options on incoming screen
        batch.push(Command::SetScreenOptions {
            screen_id,
            title: options.title.clone(),
            header_shown: options.header_shown,
            presentation: options.presentation.to_u8(),
        });

        // 2. Dispatch navigate command to native container
        batch.push(Command::Navigate {
            container_id: self.container_id,
            screen_id,
            animation: options.animation.to_u8(),
        });

        self.stack.push(ScreenEntry {
            screen_id,
            route_name: route_name.to_string(),
            options,
        });

        batch
    }

    /// Pops the active screen from the stack.
    pub fn pop(&mut self, animation: Option<TransitionAnimation>) -> (bool, CommandBatch) {
        if self.stack.len() <= 1 {
            return (false, CommandBatch::new());
        }

        self.stack.pop();
        let anim = animation.unwrap_or(TransitionAnimation::Default).to_u8();

        let mut batch = CommandBatch::new();
        batch.push(Command::GoBack {
            container_id: self.container_id,
            animation: anim,
        });

        (true, batch)
    }

    /// Replaces the active screen on the stack with a new screen.
    pub fn replace(
        &mut self,
        screen_id: NodeId,
        route_name: &str,
        options: ScreenOptions,
    ) -> CommandBatch {
        if !self.stack.is_empty() {
            self.stack.pop();
        }
        self.push(screen_id, route_name, options)
    }

    /// Updates options for an existing screen and generates a command update.
    pub fn set_options(&mut self, screen_id: NodeId, options: ScreenOptions) -> CommandBatch {
        let mut batch = CommandBatch::new();
        if let Some(entry) = self.stack.iter_mut().find(|e| e.screen_id == screen_id) {
            entry.options = options.clone();
            batch.push(Command::SetScreenOptions {
                screen_id,
                title: options.title,
                header_shown: options.header_shown,
                presentation: options.presentation.to_u8(),
            });
        }
        batch
    }

    /// Returns whether the stack has previous screens to pop to.
    pub fn can_go_back(&self) -> bool {
        self.stack.len() > 1
    }

    /// Currently active screen entry.
    pub fn active_screen(&self) -> Option<&ScreenEntry> {
        self.stack.last()
    }

    /// Current depth of the stack.
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
