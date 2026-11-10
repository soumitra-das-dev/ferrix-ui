//! Ahead-of-time Tailwind CSS compiler for native mobile apps.
//!
//! Converts Tailwind utility classes into Native `StyleIr` and `CascadeRule`s at build time.
//! Guarantees zero web runtime, zero browser engine, and zero runtime CSS parsing in the APK.

pub mod arbitrary;
pub mod compiler;
pub mod matrix;
pub mod palette;
pub mod scanner;

pub use arbitrary::{
    extract_arbitrary_value, parse_arbitrary_color, parse_arbitrary_dimension, parse_arbitrary_f32,
};
pub use compiler::{CompiledTailwind, TailwindCompiler};
pub use matrix::{map_utility, parse_scale_or_arbitrary, spacing_scale};
pub use palette::resolve_tailwind_color;
pub use scanner::scan_candidates;
