//! CSS Modules deterministic scoping generator.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::Path;

/// Generates a deterministic scoped class name for CSS Modules.
pub fn scope_class_name(file_path: &Path, class_name: &str) -> String {
    let file_stem = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module")
        .trim_end_matches(".module");

    let mut hasher = DefaultHasher::new();
    file_path.to_string_lossy().hash(&mut hasher);
    class_name.hash(&mut hasher);
    let hash = hasher.finish();
    let short_hash = (hash & 0xFFFFFF) as u32;

    format!("{}_{}_{:06x}", file_stem, class_name, short_hash)
}

/// A CSS Module containing scoped styles and its export mapping.
#[derive(Clone, Debug, Default)]
pub struct ScopedModule {
    /// Maps original local class name (e.g. "header") to scoped class name (e.g. "card_header_7a8b9c")
    pub class_map: HashMap<String, String>,
}

impl ScopedModule {
    pub fn new() -> Self {
        Self {
            class_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, local_name: impl Into<String>, scoped_name: impl Into<String>) {
        self.class_map.insert(local_name.into(), scoped_name.into());
    }

    pub fn get(&self, local_name: &str) -> Option<&str> {
        self.class_map.get(local_name).map(|s| s.as_str())
    }
}
