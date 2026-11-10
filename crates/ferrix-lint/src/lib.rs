//! # ferrix-lint
//!
//! Static analysis and linting engine for Ferrix .fsx files.

pub mod engine;

pub use engine::{LintDiagnostic, LintEngine, LintFix, Severity};

pub mod prelude {
    pub use crate::engine::{LintDiagnostic, LintEngine, LintFix, Severity};
}
