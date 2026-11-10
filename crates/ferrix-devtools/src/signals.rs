//! Reactive signal graph debugging and inspection.

use std::collections::HashMap;

/// Metadata and state snapshot for a registered reactive signal.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SignalDebugInfo {
    pub name: String,
    pub current_value: String,
    pub update_count: usize,
}

/// In-memory inspector tracking active reactive signals.
#[derive(Clone, Debug, Default)]
pub struct SignalInspector {
    signals: HashMap<String, SignalDebugInfo>,
}

impl SignalInspector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Records or updates the current value of a named signal.
    pub fn record_value(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name_str = name.into();
        let val_str = value.into();

        let entry = self
            .signals
            .entry(name_str.clone())
            .or_insert_with(|| SignalDebugInfo {
                name: name_str,
                current_value: String::new(),
                update_count: 0,
            });

        entry.current_value = val_str;
        entry.update_count += 1;
    }

    /// Returns debug information for all tracked signals.
    pub fn all_signals(&self) -> Vec<SignalDebugInfo> {
        let mut list: Vec<SignalDebugInfo> = self.signals.values().cloned().collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }

    /// Total count of registered signals.
    pub fn count(&self) -> usize {
        self.signals.len()
    }

    /// Retrieves debug info for a specific signal by name.
    pub fn get(&self, name: &str) -> Option<&SignalDebugInfo> {
        self.signals.get(name)
    }
}
