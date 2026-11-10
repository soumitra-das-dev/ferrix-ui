//! Reactive live queries driven by table invalidation channels.

use crate::db::{subscribe_table, Database};
use ferrix_reactivity::Signal;
use std::cell::RefCell;

thread_local! {
    static DEFAULT_DB: RefCell<Option<Database>> = const { RefCell::new(None) };
}

/// Sets the active database for the current runtime thread.
pub fn set_default_db(db: Database) {
    DEFAULT_DB.with(|cell| {
        *cell.borrow_mut() = Some(db);
    });
}

/// Retrieves the active database for the current runtime thread.
pub fn get_default_db() -> Option<Database> {
    DEFAULT_DB.with(|cell| cell.borrow().clone())
}

/// Creates a fine-grained reactive `Signal<Vec<T>>` that automatically re-executes
/// and updates whenever the specified `table` is mutated.
pub fn use_live_query<T, F>(
    db: &Database,
    table: &str,
    sql: &'static str,
    mapper: F,
) -> Signal<Vec<T>>
where
    T: Clone + 'static,
    F: Fn(&rusqlite::Row) -> rusqlite::Result<T> + Clone + 'static,
{
    let initial = db.query(sql, &[], mapper.clone()).unwrap_or_default();
    let signal = Signal::new(initial);

    let sig_clone = signal.clone();
    let db_clone = db.clone();
    let mapper_clone = mapper;

    subscribe_table(table, move || {
        if let Ok(updated) = db_clone.query(sql, &[], mapper_clone.clone()) {
            sig_clone.set(updated);
        }
    });

    signal
}
