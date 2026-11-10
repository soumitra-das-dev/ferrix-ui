//! Linter execution engine and diagnostics collection.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LintFix {
    pub replacement: String,
    pub target_line: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LintDiagnostic {
    pub rule: String,
    pub severity: Severity,
    pub message: String,
    pub line: usize,
    pub col: usize,
    pub fix: Option<LintFix>,
}

#[derive(Default)]
pub struct LintEngine;

impl LintEngine {
    pub fn new() -> Self {
        Self
    }

    /// Analyzes source code and produces diagnostics for rule violations.
    pub fn lint(&self, source: &str) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        let mut in_conditional_or_loop = false;
        let mut block_depth = 0;
        let mut conditional_depth = 0;

        for (idx, &line) in lines.iter().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();

            if trimmed.starts_with("//") {
                continue;
            }

            // Track block depths for rules of hooks
            if trimmed.starts_with("if ")
                || trimmed.starts_with("if(")
                || trimmed.starts_with("for ")
                || trimmed.starts_with("for(")
                || trimmed.starts_with("while ")
                || trimmed.starts_with("while(")
            {
                in_conditional_or_loop = true;
                conditional_depth = block_depth + 1;
            }

            if trimmed.contains('{') {
                block_depth += trimmed.matches('{').count();
            }
            if trimmed.contains('}') {
                let closes = trimmed.matches('}').count();
                if block_depth >= closes {
                    block_depth -= closes;
                } else {
                    block_depth = 0;
                }
                if block_depth < conditional_depth {
                    in_conditional_or_loop = false;
                }
            }

            // Rule 1: rules-of-hooks
            if in_conditional_or_loop {
                for hook in [
                    "useState",
                    "useSignal",
                    "useMemo",
                    "useEffect",
                    "useRouter",
                    "useStorage",
                ] {
                    if trimmed.contains(hook) {
                        diagnostics.push(LintDiagnostic {
                            rule: "rules-of-hooks".to_string(),
                            severity: Severity::Error,
                            message: format!(
                                "Hook '{}' cannot be called inside a loop or conditional block",
                                hook
                            ),
                            line: line_num,
                            col: trimmed.find(hook).unwrap_or(0) + 1,
                            fix: None,
                        });
                    }
                }
            }

            // Rule 2: require-keyed-children
            if trimmed.contains(".map(") && lines.get(idx + 1).is_some() {
                // Check if following line creates a JSX element without key=
                let next_line = lines[idx + 1].trim();
                if (next_line.starts_with('<') || next_line.contains("(<"))
                    && !next_line.contains("key=")
                {
                    diagnostics.push(LintDiagnostic {
                        rule: "require-keyed-children".to_string(),
                        severity: Severity::Warning,
                        message: "Elements rendered directly inside an array map must have a unique 'key' prop".to_string(),
                        line: line_num + 1,
                        col: 1,
                        fix: None,
                    });
                }
            }

            // Rule 3: accessibility-pressable
            let is_pressable = trimmed.starts_with("<Pressable") || trimmed.contains("<Pressable ");
            if is_pressable
                && !trimmed.contains("accessibilityLabel=")
                && !trimmed.contains("aria-label=")
            {
                diagnostics.push(LintDiagnostic {
                    rule: "accessibility-pressable".to_string(),
                    severity: Severity::Warning,
                    message: "Interactive <Pressable> element should provide an 'accessibilityLabel' for screen readers".to_string(),
                    line: line_num,
                    col: 1,
                    fix: None,
                });
            }

            // Rule 4: unused-imports
            if trimmed.starts_with("import ") && trimmed.contains(" from ") {
                if let Some(imported_part) = trimmed.strip_prefix("import {") {
                    if let Some((symbols_str, _)) = imported_part.split_once('}') {
                        for sym in symbols_str.split(',') {
                            let sym = sym.trim();
                            if !sym.is_empty() {
                                // Count occurrences of this symbol in the entire source
                                let count = source.matches(sym).count();
                                if count <= 1 {
                                    diagnostics.push(LintDiagnostic {
                                        rule: "unused-imports".to_string(),
                                        severity: Severity::Warning,
                                        message: format!("Unused import '{}' detected", sym),
                                        line: line_num,
                                        col: 1,
                                        fix: Some(LintFix {
                                            replacement: String::new(),
                                            target_line: line_num,
                                        }),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        diagnostics
    }

    /// Automatically applies fixes for fixable diagnostics.
    pub fn apply_fixes(&self, source: &str, diagnostics: &[LintDiagnostic]) -> String {
        let mut lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();

        for diag in diagnostics {
            if let Some(fix) = &diag.fix {
                if fix.target_line > 0 && fix.target_line <= lines.len() {
                    let idx = fix.target_line - 1;
                    if fix.replacement.is_empty() {
                        lines[idx].clear(); // Delete or blank out
                    } else {
                        lines[idx] = fix.replacement.clone();
                    }
                }
            }
        }

        let filtered: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
        filtered.join("\n") + "\n"
    }
}
