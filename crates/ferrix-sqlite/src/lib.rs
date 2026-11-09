//! # ferrix-sqlite
//!
//! Embedded reactive SQLite database engine for Ferrix.
//! Features schema migrations, WAL journaling, and reactive live queries (`use_live_query`).

pub mod db;
pub mod live_query;
pub mod migration;

pub use db::Database;
pub use live_query::{get_default_db, set_default_db, use_live_query};
pub use migration::{Migration, MigrationError, MigrationRunner};

pub mod prelude {
    pub use crate::db::Database;
    pub use crate::live_query::use_live_query;
    pub use crate::migration::Migration;
}
