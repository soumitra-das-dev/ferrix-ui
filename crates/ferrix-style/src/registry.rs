//! Intern pool and style registry allocating stable `u32` style IDs.

use std::collections::HashMap;

use ferrix_renderer_protocol::Patch;

use crate::ir::StyleIr;

/// Intern pool and style registry allocating stable `u32` style IDs.
#[derive(Clone, Debug, Default)]
pub struct StyleRegistry {
    styles: HashMap<u32, StyleIr>,
    intern_map: HashMap<StyleIr, u32>,
    uncommitted: Vec<u32>,
    next_id: u32,
}

impl StyleRegistry {
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
            intern_map: HashMap::new(),
            uncommitted: Vec::new(),
            next_id: 1,
        }
    }

    /// Interns a `StyleIr` into the registry.
    /// If an identical style has already been registered, returns its existing ID (deduplication).
    /// Otherwise allocates a new monotonic ID and marks it as uncommitted.
    pub fn intern(&mut self, style: StyleIr) -> u32 {
        if let Some(&id) = self.intern_map.get(&style) {
            return id;
        }

        let id = self.next_id;
        self.next_id += 1;

        self.styles.insert(id, style.clone());
        self.intern_map.insert(style, id);
        self.uncommitted.push(id);
        id
    }

    /// Explicitly registers a style with a predetermined ID.
    pub fn register(&mut self, id: u32, style: StyleIr) {
        if id >= self.next_id {
            self.next_id = id + 1;
        }
        self.styles.insert(id, style.clone());
        self.intern_map.insert(style, id);
        self.uncommitted.push(id);
    }

    /// Looks up a style by ID.
    pub fn get(&self, id: u32) -> Option<&StyleIr> {
        self.styles.get(&id)
    }

    /// Returns the number of registered styles.
    pub fn len(&self) -> usize {
        self.styles.len()
    }

    /// Returns true if no styles have been registered.
    pub fn is_empty(&self) -> bool {
        self.styles.is_empty()
    }

    /// Emits a `Patch::RegisterStyles` for all styles interned since the last flush.
    pub fn take_registration_patch(&mut self) -> Option<Patch> {
        if self.uncommitted.is_empty() {
            return None;
        }

        let mut entries = Vec::with_capacity(self.uncommitted.len());
        for &id in &self.uncommitted {
            if let Some(style) = self.styles.get(&id) {
                entries.push((id, style.to_properties()));
            }
        }
        self.uncommitted.clear();

        Some(Patch::RegisterStyles { styles: entries })
    }

    /// Emits a `Patch::RegisterStyles` containing all currently registered styles.
    pub fn emit_all_patches(&self) -> Patch {
        let mut entries = Vec::with_capacity(self.styles.len());
        for (&id, style) in &self.styles {
            entries.push((id, style.to_properties()));
        }
        Patch::RegisterStyles { styles: entries }
    }
}
