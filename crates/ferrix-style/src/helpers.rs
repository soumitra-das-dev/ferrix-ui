//! Styling helper utilities: `cx` class merging and CVA-style variant composers.

use std::collections::HashMap;

use crate::ir::StyleIr;

pub trait ToOptionStr<'a> {
    fn to_opt_str(&self) -> Option<&'a str>;
}

impl<'a> ToOptionStr<'a> for &'a str {
    fn to_opt_str(&self) -> Option<&'a str> {
        Some(*self)
    }
}

impl<'a> ToOptionStr<'a> for Option<&'a str> {
    fn to_opt_str(&self) -> Option<&'a str> {
        *self
    }
}

/// Merges an iterator of optional class names together, filtering out empties and None.
pub fn cx<'a>(classes: impl IntoIterator<Item = Option<&'a str>>) -> String {
    let mut result = String::new();
    for s in classes.into_iter().flatten() {
        let s = s.trim();
        if !s.is_empty() {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(s);
        }
    }
    result
}

/// Macro for merging mixed class name types (e.g. `cx!("btn", Some("active"), None)`).
#[macro_export]
macro_rules! cx {
    ($($expr:expr),* $(,)?) => {{
        let items: &[Option<&str>] = &[
            $( $crate::helpers::ToOptionStr::to_opt_str(&$expr) ),*
        ];
        $crate::helpers::cx(items.iter().copied())
    }};
}

/// A CVA-style variant definition for composing multi-dimensional component styles.
#[derive(Clone, Debug, Default)]
pub struct VariantConfig {
    pub base: StyleIr,
    pub variants: HashMap<String, HashMap<String, StyleIr>>,
    pub default_variants: HashMap<String, String>,
}

impl VariantConfig {
    pub fn new(base: StyleIr) -> Self {
        Self {
            base,
            variants: HashMap::new(),
            default_variants: HashMap::new(),
        }
    }

    /// Adds a variant dimension (e.g. "intent" -> {"primary" => StyleIr, "secondary" => StyleIr}).
    pub fn with_variant(
        mut self,
        dimension: impl Into<String>,
        options: impl IntoIterator<Item = (impl Into<String>, StyleIr)>,
    ) -> Self {
        let mut map = HashMap::new();
        for (k, v) in options {
            map.insert(k.into(), v);
        }
        self.variants.insert(dimension.into(), map);
        self
    }

    /// Sets default variant selections (e.g. "intent" -> "primary", "size" -> "md").
    pub fn with_default(mut self, dimension: impl Into<String>, option: impl Into<String>) -> Self {
        self.default_variants
            .insert(dimension.into(), option.into());
        self
    }

    /// Resolves the final `StyleIr` given explicit variant selections.
    pub fn resolve<'a>(&self, selections: impl IntoIterator<Item = (&'a str, &'a str)>) -> StyleIr {
        let mut resolved = self.base.clone();

        let mut final_selections = self.default_variants.clone();
        for (dim, opt) in selections {
            final_selections.insert(dim.to_string(), opt.to_string());
        }

        for (dim, opt) in final_selections {
            if let Some(dim_map) = self.variants.get(&dim) {
                if let Some(style) = dim_map.get(&opt) {
                    resolved = resolved.merge(style);
                }
            }
        }

        resolved
    }
}
