//! Diagnostics for the Ferrix CSS compiler with file:line:col source reporting.

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StyleDiagnostic {
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub file: String,
    pub line: usize,
    pub col: usize,
}

impl StyleDiagnostic {
    pub fn error(
        message: impl Into<String>,
        file: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            message: message.into(),
            file: file.into(),
            line,
            col,
        }
    }

    pub fn warning(
        message: impl Into<String>,
        file: impl Into<String>,
        line: usize,
        col: usize,
    ) -> Self {
        Self {
            severity: DiagnosticSeverity::Warning,
            message: message.into(),
            file: file.into(),
            line,
            col,
        }
    }

    pub fn is_error(&self) -> bool {
        self.severity == DiagnosticSeverity::Error
    }
}

impl fmt::Display for StyleDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = match self.severity {
            DiagnosticSeverity::Error => "error",
            DiagnosticSeverity::Warning => "warning",
        };
        write!(
            f,
            "{}:{}:{}: {}: {}",
            self.file, self.line, self.col, tag, self.message
        )
    }
}
