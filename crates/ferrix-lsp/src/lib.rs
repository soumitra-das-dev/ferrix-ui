//! # ferrix-lsp
//!
//! Language Server Protocol (LSP) implementation for Ferrix .fsx files.

pub mod server;

pub use server::{JsonRpcMessage, LspServer};

pub mod prelude {
    pub use crate::server::{JsonRpcMessage, LspServer};
}
