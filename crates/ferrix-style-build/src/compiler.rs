//! Main CSS Compiler driver coordinating parsing, CSS modules, and Rust code generation.

use std::collections::HashMap;
use std::path::Path;

use ferrix_style::{CascadeRule, RuleSource};

use crate::diagnostics::StyleDiagnostic;
use crate::modules::scope_class_name;
use crate::parser::CssParser;

/// Output of a compiled CSS stylesheet.
#[derive(Clone, Debug)]
pub struct CompiledStylesheet {
    pub rules: Vec<CascadeRule>,
    pub class_map: HashMap<String, String>,
    pub variables: HashMap<String, u32>,
    pub diagnostics: Vec<StyleDiagnostic>,
}

/// Compiler transforming CSS and CSS Modules into Ferrix Native Style IR rules.
#[derive(Clone, Debug, Default)]
pub struct StyleCompiler;

impl StyleCompiler {
    pub fn new() -> Self {
        Self
    }

    /// Compiles CSS text. Detects CSS module by file name ending with `.module.css`.
    pub fn compile(
        &self,
        source: &str,
        file_path: &Path,
    ) -> Result<CompiledStylesheet, Vec<StyleDiagnostic>> {
        let file_name = file_path.to_string_lossy().to_string();
        let is_module = file_name.ends_with(".module.css");

        let mut class_map = HashMap::new();
        let mut processed_css = source.to_string();

        if is_module {
            // Find all class selectors (e.g. .card, .btn) and replace with scoped names
            let mut words: Vec<&str> = source
                .split(|c: char| c.is_ascii_whitespace() || c == '{' || c == '}' || c == ';')
                .collect();
            words.sort();
            words.dedup();

            for word in words {
                let word = word.trim();
                if let Some(class_name) = word.strip_prefix('.') {
                    let clean_class = class_name
                        .split([':', '.', '>', ','])
                        .next()
                        .unwrap_or("")
                        .trim();

                    if !clean_class.is_empty() && !class_map.contains_key(clean_class) {
                        let scoped = scope_class_name(file_path, clean_class);
                        class_map.insert(clean_class.to_string(), scoped.clone());
                    }
                }
            }

            // Replace class selectors in processed CSS
            for (local, scoped) in &class_map {
                let target = format!(".{}", local);
                let replacement = format!(".{}", scoped);
                processed_css = processed_css.replace(&target, &replacement);
            }
        }

        let default_source = if is_module {
            RuleSource::ModuleCss
        } else {
            RuleSource::GlobalCss
        };

        let parser = CssParser::new(&processed_css, &file_name);
        let (rules, variables) = parser.parse(default_source)?;

        Ok(CompiledStylesheet {
            rules,
            class_map,
            variables,
            diagnostics: Vec::new(),
        })
    }
}
