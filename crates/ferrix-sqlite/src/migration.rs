//! Schema migration engine for SQLite databases.

use rusqlite::{params, Connection, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Migration version mismatch: expected {expected}, found {found}")]
    VersionMismatch { expected: u32, found: u32 },
}

/// A versioned, forward-only schema migration.
#[derive(Clone, Debug, PartialEq)]
pub struct Migration {
    pub version: u32,
    pub name: &'static str,
    pub sql: &'static str,
}

pub struct MigrationRunner;

impl MigrationRunner {
    /// Applies unapplied migrations in ascending version order inside transactions.
    pub fn apply(conn: &mut Connection, migrations: &[Migration]) -> Result<usize, MigrationError> {
        // Ensure migration tracking table exists
        conn.execute(
            "CREATE TABLE IF NOT EXISTS _ferrix_migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
            [],
        )?;

        let mut applied_count = 0;

        for m in migrations {
            let already_applied: bool = conn
                .query_row(
                    "SELECT 1 FROM _ferrix_migrations WHERE version = ?1",
                    params![m.version],
                    |_| Ok(true),
                )
                .unwrap_or(false);

            if !already_applied {
                let tx = conn.transaction()?;
                tx.execute_batch(m.sql)?;
                tx.execute(
                    "INSERT INTO _ferrix_migrations (version, name) VALUES (?1, ?2)",
                    params![m.version, m.name],
                )?;
                tx.commit()?;
                applied_count += 1;
            }
        }

        Ok(applied_count)
    }

    /// Returns list of applied migration versions.
    pub fn applied_versions(conn: &Connection) -> Result<Vec<u32>, rusqlite::Error> {
        let mut stmt =
            conn.prepare("SELECT version FROM _ferrix_migrations ORDER BY version ASC")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        let mut versions = Vec::new();
        for r in rows {
            versions.push(r?);
        }
        Ok(versions)
    }
}
