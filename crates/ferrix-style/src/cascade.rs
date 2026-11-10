//! CSS Cascade Engine and Specificity calculator.

use crate::ir::StyleIr;

/// CSS Specificity tuple `(a, b, c)`.
/// - `a`: ID selectors (always 0 in Ferrix)
/// - `b`: Class selectors, attribute selectors, and pseudo-classes
/// - `c`: Type selectors and pseudo-elements
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Specificity {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

impl Specificity {
    pub const fn new(a: u32, b: u32, c: u32) -> Self {
        Self { a, b, c }
    }

    /// Universal selector `*` specificity: (0, 0, 0).
    pub const UNIVERSAL: Self = Self::new(0, 0, 0);

    /// Single class selector `.card` specificity: (0, 1, 0).
    pub const CLASS: Self = Self::new(0, 1, 0);

    /// Single element type selector `Text` specificity: (0, 0, 1).
    pub const ELEMENT: Self = Self::new(0, 0, 1);
}

/// Source tier of a style rule according to the Ferrix merge hierarchy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuleSource {
    Universal = 0,
    GlobalCss = 1,
    ModuleCss = 2,
    Inline = 3,
}

/// Selector kind pre-parsed for fast matching without allocations.
#[derive(Clone, Debug, PartialEq)]
pub enum ParsedSelector {
    Universal,
    Tag(String),
    Classes(Vec<String>),
    Other,
}

/// A parsed style rule participating in cascade resolution.
#[derive(Clone, Debug, PartialEq)]
pub struct CascadeRule {
    pub selector: String,
    pub parsed: ParsedSelector,
    pub source: RuleSource,
    pub layer: u32,
    pub specificity: Specificity,
    pub source_order: u32,
    pub style: StyleIr,
}

impl CascadeRule {
    pub fn new(
        selector: impl Into<String>,
        source: RuleSource,
        specificity: Specificity,
        source_order: u32,
        style: StyleIr,
    ) -> Self {
        let selector = selector.into();
        let sel = selector.trim();
        let parsed = if sel == "*" {
            ParsedSelector::Universal
        } else if sel.starts_with('.') {
            let classes: Vec<String> = sel
                .split('.')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();
            ParsedSelector::Classes(classes)
        } else if !sel.starts_with(':') {
            ParsedSelector::Tag(sel.to_string())
        } else {
            ParsedSelector::Other
        };

        Self {
            selector,
            parsed,
            source,
            layer: 0,
            specificity,
            source_order,
            style,
        }
    }

    pub fn with_layer(mut self, layer: u32) -> Self {
        self.layer = layer;
        self
    }

    /// Evaluates if this rule matches an element with the given classes and element type.
    pub fn matches(&self, class_names: &[&str], element_type: Option<&str>) -> bool {
        match &self.parsed {
            ParsedSelector::Universal => true,
            ParsedSelector::Tag(tag) => {
                if let Some(el) = element_type {
                    tag.eq_ignore_ascii_case(el)
                } else {
                    false
                }
            }
            ParsedSelector::Classes(req_classes) => req_classes
                .iter()
                .all(|c| class_names.contains(&c.as_str())),
            ParsedSelector::Other => false,
        }
    }
}

/// Resolves style rules according to standard CSS specificity and source hierarchy.
#[derive(Clone, Debug, Default)]
pub struct CascadeEngine {
    rules: Vec<CascadeRule>,
}

impl CascadeEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: CascadeRule) {
        self.rules.push(rule);
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Resolves the effective `StyleIr` for an element.
    /// Merge order:
    /// Universal (*) < Global CSS < Module Rules (by layer, specificity, source_order) < Inline style.
    pub fn resolve(
        &self,
        class_names: &[&str],
        element_type: Option<&str>,
        inline: Option<&StyleIr>,
    ) -> StyleIr {
        // Collect all matching rules
        let mut matching: Vec<&CascadeRule> = self
            .rules
            .iter()
            .filter(|r| r.matches(class_names, element_type))
            .collect();

        // Sort by cascade precedence:
        // 1. RuleSource (Universal < Global < Module)
        // 2. layer (0 < 1 < 2 ...)
        // 3. specificity (a, b, c)
        // 4. source_order (earlier < later)
        matching.sort_by(|r1, r2| {
            r1.source
                .cmp(&r2.source)
                .then_with(|| r1.layer.cmp(&r2.layer))
                .then_with(|| r1.specificity.cmp(&r2.specificity))
                .then_with(|| r1.source_order.cmp(&r2.source_order))
        });

        // Merge matching rules in ascending precedence
        let mut resolved = StyleIr::new();
        for rule in matching {
            resolved = resolved.merge(&rule.style);
        }

        // Inline styles always take final precedence
        if let Some(inline_style) = inline {
            resolved = resolved.merge(inline_style);
        }

        resolved
    }
}
