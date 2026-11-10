//! # ferrix-style-build
//!
//! Build-time CSS parser, shorthand expander, CSS Modules deterministic scoping,
//! and native style compiler for Ferrix under Architecture A.

pub mod compiler;
pub mod diagnostics;
pub mod modules;
pub mod parser;
pub mod postcss;
pub mod shorthands;
pub mod values;

pub use compiler::{CompiledStylesheet, StyleCompiler};
pub use diagnostics::{DiagnosticSeverity, StyleDiagnostic};
pub use modules::{scope_class_name, ScopedModule};
pub use parser::CssParser;
pub use postcss::{OriginalLocation, PostCssOutput, PostCssRunner, SourceMap};
