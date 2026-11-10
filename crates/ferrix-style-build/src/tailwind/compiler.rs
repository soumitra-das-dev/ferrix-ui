//! Tailwind compiler generating ahead-of-time `CascadeRule`s for native execution.
//! In production APKs, zero CSS parsing or string interpretation happens at runtime.

use std::collections::HashSet;

use ferrix_layout::{Display, FlexDirection, Position};
use ferrix_style::{CascadeRule, RuleSource, Specificity, StyleIr};

use super::matrix::map_utility;
use super::scanner::scan_candidates;
use crate::diagnostics::StyleDiagnostic;

/// Result of ahead-of-time Tailwind compilation.
#[derive(Clone, Debug, Default)]
pub struct CompiledTailwind {
    pub rules: Vec<CascadeRule>,
    pub diagnostics: Vec<StyleDiagnostic>,
}

/// Ahead-of-time Tailwind compiler for Ferrix native mobile apps.
#[derive(Clone, Debug, Default)]
pub struct TailwindCompiler {
    safelist: Vec<String>,
}

impl TailwindCompiler {
    pub fn new() -> Self {
        Self {
            safelist: Vec::new(),
        }
    }

    pub fn with_safelist(mut self, safelist: &[&str]) -> Self {
        self.safelist = safelist.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Compiles Tailwind utility rules for an entire set of `.fsx` sources and explicit classes.
    pub fn compile(
        &self,
        sources: &[&str],
        explicit_classes: &[&str],
    ) -> Result<CompiledTailwind, Vec<StyleDiagnostic>> {
        let mut candidates = HashSet::new();

        // 1. Scan `.fsx` sources
        let safelist_refs: Vec<&str> = self.safelist.iter().map(String::as_str).collect();
        for src in sources {
            let scanned = scan_candidates(src, &safelist_refs);
            candidates.extend(scanned);
        }

        // 2. Add explicit classes
        for &cls in explicit_classes {
            for word in cls.split_whitespace() {
                if !word.is_empty() {
                    candidates.insert(word.to_string());
                }
            }
        }

        let mut rules = Vec::new();
        let mut diagnostics = Vec::new();

        // 3. Inject React Native / Ferrix mobile preflight defaults:
        // Universal selector: display: flex, flex-direction: column, position: relative
        let mut preflight_style = StyleIr::new();
        preflight_style.display = Some(Display::Flex);
        preflight_style.flex_direction = Some(FlexDirection::Column);
        preflight_style.position = Some(Position::Relative);

        rules.push(CascadeRule::new(
            "*",
            RuleSource::Universal,
            Specificity::UNIVERSAL,
            0,
            preflight_style,
        ));

        // 4. Deterministically sort candidate classes for reproducible builds
        let mut sorted_candidates: Vec<String> = candidates.into_iter().collect();
        sorted_candidates.sort();

        let mut source_order = 100u32;

        for candidate in sorted_candidates {
            match map_utility(&candidate) {
                Ok(Some(style)) => {
                    let selector = format!(".{}", candidate);
                    rules.push(CascadeRule::new(
                        selector,
                        RuleSource::GlobalCss,
                        Specificity::CLASS,
                        source_order,
                        style,
                    ));
                    source_order += 1;
                }
                Ok(None) => {
                    // Non-tailwind or unrecognized utility; ignored
                }
                Err(diag) => {
                    diagnostics.push(diag);
                }
            }
        }

        if diagnostics.iter().any(|d| d.is_error()) {
            return Err(diagnostics);
        }

        Ok(CompiledTailwind { rules, diagnostics })
    }
}
