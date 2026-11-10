//! Thread-safe SQLite database connection wrapper and table mutation notification.

use crate::migration::{Migration, MigrationError, MigrationRunner};
use rusqlite::{Connection, Result, ToSql};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

type TableListenerMap = RefCell<HashMap<String, Vec<Box<dyn Fn()>>>>;

thread_local! {
    static TABLE_LISTENERS: TableListenerMap = RefCell::new(HashMap::new());
}

/// Registers a callback invoked whenever the specified table is mutated on the runtime thread.
pub fn subscribe_table<F: Fn() + 'static>(table: &str, callback: F) {
    TABLE_LISTENERS.with(|cell| {
        cell.borrow_mut()
            .entry(table.to_lowercase())
            .or_default()
            .push(Box::new(callback));
    });
}

/// Broadcasts a table mutation notification to runtime thread listeners.
pub fn notify_table_changed(table: &str) {
    TABLE_LISTENERS.with(|cell| {
        if let Some(cbs) = cell.borrow().get(&table.to_lowercase()) {
            for cb in cbs {
                cb();
            }
        }
    });
}

/// Thread-safe SQLite database handle with reactive table change notifications.
#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    /// Opens an in-memory SQLite database.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Self::init(conn)
    }

    /// Opens a persistent SQLite database with WAL mode enabled.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        Self::init(conn)
    }

    fn init(conn: Connection) -> Result<Self> {
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;",
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Applies schema migrations atomically.
    pub fn migrate(&self, migrations: &[Migration]) -> std::result::Result<usize, MigrationError> {
        let mut conn = self.conn.lock().unwrap();
        MigrationRunner::apply(&mut conn, migrations)
    }

    /// Executes an INSERT, UPDATE, or DELETE and notifies table subscribers.
    pub fn execute_mutation(
        &self,
        affected_table: &str,
        sql: &str,
        params: &[&dyn ToSql],
    ) -> Result<usize> {
        let rows = {
            let conn = self.conn.lock().unwrap();
            conn.execute(sql, params)?
        };

        notify_table_changed(affected_table);
        Ok(rows)
    }

    /// Executes a read query synchronously mapping each row.
    pub fn query<T, F>(&self, sql: &str, params: &[&dyn ToSql], mut mapper: F) -> Result<Vec<T>>
    where
        F: FnMut(&rusqlite::Row) -> Result<T>,
    {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params, |row| mapper(row))?;
        let mut result = Vec::new();
        for r in rows {
            result.push(r?);
        }
        Ok(result)
    }
}
