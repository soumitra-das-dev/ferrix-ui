//! kitchen-sink - Built with Ferrix UI Framework

// Ahead-of-time compiled modules from all .fsx files in src/
include!(concat!(env!("OUT_DIR"), "/generated_modules.rs"));

// Re-export primary component for convenience
pub use component::*;
