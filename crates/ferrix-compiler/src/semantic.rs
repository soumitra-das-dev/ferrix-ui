//! Semantic analysis, hook invariant verification, and diagnostics formatting.

use crate::ast::*;
use crate::token::Span;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DiagnosticSeverity {
    #[error("error")]
    Error,
    #[error("warning")]
    Warning,
}

#[derive(Debug, PartialEq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub message: String,
    pub span: Span,
    pub note: Option<String>,
    pub file_path: String,
}

impl Diagnostic {
    pub fn new(
        code: &'static str,
        message: impl Into<String>,
        span: Span,
        note: Option<impl Into<String>>,
        file_path: impl Into<String>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            span,
            note: note.map(|n| n.into()),
            file_path: file_path.into(),
        }
    }

    /// Formats the diagnostic in rustc-style ASCII format.
    pub fn render(&self, source: &str) -> String {
        let lines: Vec<&str> = source.lines().collect();
        let line_num = self.span.line;
        let col_num = self.span.col;

        let source_line = lines.get(line_num.saturating_sub(1)).copied().unwrap_or("");
        let underline_len = (self.span.end.saturating_sub(self.span.start)).max(1);
        let padding = " ".repeat(col_num.saturating_sub(1));
        let underline = "^".repeat(
            underline_len.min(
                source_line
                    .len()
                    .saturating_sub(col_num.saturating_sub(1))
                    .max(1),
            ),
        );

        let mut out = format!(
            "error[{}]: {}\n --> {}:{}:{}\n  |\n{} | {}\n  | {}{}\n",
            self.code,
            self.message,
            self.file_path,
            line_num,
            col_num,
            line_num,
            source_line,
            padding,
            underline
        );

        if let Some(note) = &self.note {
            out.push_str(&format!("  |\n  = note: {}\n", note));
        }

        out
    }
}

pub struct SemanticAnalyzer {
    file_path: String,
    diagnostics: Vec<Diagnostic>,
}

impl SemanticAnalyzer {
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
            diagnostics: Vec::new(),
        }
    }

    pub fn analyze(&mut self, source_file: &SourceFile) -> Result<(), Vec<Diagnostic>> {
        for component in &source_file.components {
            self.analyze_component(component);
        }

        if self.diagnostics.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.diagnostics))
        }
    }

    fn analyze_component(&mut self, component: &ComponentDecl) {
        let mut seen_non_hook = false;

        for stmt in &component.statements {
            match stmt {
                Statement::Hook(hook) => {
                    if seen_non_hook {
                        self.diagnostics.push(Diagnostic::new(
                            "FSX0002",
                            format!("hook '{}' called after non-hook statements", hook.hook_name),
                            hook.span,
                            Some("all hooks must be declared unconditionally at the very top of the component"),
                            &self.file_path,
                        ));
                    }
                }
                Statement::Let { .. } | Statement::Expr(_) => {
                    seen_non_hook = true;
                }
            }
        }
    }
}
