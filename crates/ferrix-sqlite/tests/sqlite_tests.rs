//! Unit and integration tests for ferrix-sqlite (F3).

use ferrix_sqlite::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TodoItem {
    id: i64,
    title: String,
    completed: bool,
}

#[test]
fn test_database_initialization_and_migrations() {
    let db = Database::open_in_memory().expect("Failed to open SQLite in memory");

    let migrations = vec![
        Migration {
            version: 1,
            name: "create_todos_table",
            sql: "CREATE TABLE todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0
            );",
        },
        Migration {
            version: 2,
            name: "add_priority_column",
            sql: "ALTER TABLE todos ADD COLUMN priority INTEGER NOT NULL DEFAULT 0;",
        },
    ];

    // Apply migrations
    let applied = db.migrate(&migrations).expect("Migrations failed");
    assert_eq!(applied, 2);

    // Re-applying is idempotent
    let applied_again = db
        .migrate(&migrations)
        .expect("Idempotent migration failed");
    assert_eq!(applied_again, 0);
}

#[test]
fn test_database_crud_operations() {
    let db = Database::open_in_memory().unwrap();
    let migrations = vec![Migration {
        version: 1,
        name: "create_users",
        sql: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT NOT NULL UNIQUE);",
    }];
    db.migrate(&migrations).unwrap();

    // Insert
    let inserted = db
        .execute_mutation(
            "users",
            "INSERT INTO users (id, name, email) VALUES (?1, ?2, ?3)",
            &[&1i64, &"Alice", &"alice@ferrix.dev"],
        )
        .unwrap();
    assert_eq!(inserted, 1);

    // Read
    let users: Vec<(i64, String)> = db
        .query(
            "SELECT id, name FROM users WHERE id = ?1",
            &[&1i64],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0], (1, "Alice".to_string()));

    // Update
    db.execute_mutation(
        "users",
        "UPDATE users SET name = ?1 WHERE id = ?2",
        &[&"Alice Wonder", &1i64],
    )
    .unwrap();

    let updated: String = db
        .query("SELECT name FROM users WHERE id = 1", &[], |row| row.get(0))
        .unwrap()
        .pop()
        .unwrap();
    assert_eq!(updated, "Alice Wonder");

    // Delete
    db.execute_mutation("users", "DELETE FROM users WHERE id = 1", &[])
        .unwrap();
    let count: i64 = db
        .query("SELECT COUNT(*) FROM users", &[], |row| row.get(0))
        .unwrap()
        .pop()
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_live_query_reactive_invalidation() {
    let db = Database::open_in_memory().unwrap();
    let migrations = vec![Migration {
        version: 1,
        name: "create_tasks",
        sql: "CREATE TABLE tasks (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, completed BOOLEAN NOT NULL);",
    }];
    db.migrate(&migrations).unwrap();

    // Register live query for active tasks
    let active_tasks = use_live_query(
        &db,
        "tasks",
        "SELECT id, title, completed FROM tasks WHERE completed = 0 ORDER BY id ASC",
        |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        },
    );

    assert_eq!(active_tasks.get().len(), 0);

    // Insert task 1 (active)
    db.execute_mutation(
        "tasks",
        "INSERT INTO tasks (title, completed) VALUES (?1, 0)",
        &[&"Buy milk"],
    )
    .unwrap();

    // Live query signal automatically updated
    assert_eq!(active_tasks.get().len(), 1);
    assert_eq!(active_tasks.get()[0].title, "Buy milk");

    // Insert task 2 (completed) - shouldn't appear in active tasks query
    db.execute_mutation(
        "tasks",
        "INSERT INTO tasks (title, completed) VALUES (?1, 1)",
        &[&"Read docs"],
    )
    .unwrap();
    assert_eq!(active_tasks.get().len(), 1);

    // Insert task 3 (active)
    db.execute_mutation(
        "tasks",
        "INSERT INTO tasks (title, completed) VALUES (?1, 0)",
        &[&"Write tests"],
    )
    .unwrap();
    assert_eq!(active_tasks.get().len(), 2);
    assert_eq!(active_tasks.get()[1].title, "Write tests");

    // Mark task 1 as completed
    db.execute_mutation(
        "tasks",
        "UPDATE tasks SET completed = 1 WHERE title = ?1",
        &[&"Buy milk"],
    )
    .unwrap();
    assert_eq!(active_tasks.get().len(), 1);
    assert_eq!(active_tasks.get()[0].title, "Write tests");
}
