# F3 Learnings: Reactive Storage & SQLite (`ferrix-storage`, `ferrix-sqlite`)

## Architecture Decisions & Storage Invariants
Storage under **Architecture A** enforces that client-side state inspection never incurs synchronous JNI stalls, disk waits, or thread contention.

### 1. Sync Read / Async Durable Write Invariant (`ferrix-storage`)
- **$O(1)$ Synchronous Reads**: All keys are mirrored in an in-memory `RwLock<HashMap<String, Vec<u8>>>` cache. Component render paths and state selectors read from memory without blocking.
- **Append-Only WAL Journal**: Writes are immediately applied to the cache and appended to a crash-safe WAL journal file:
  `[MAGIC: 4B ("FXWL")][CRC32: 4B][OP: 1B (1=SET, 2=DEL)][KEY_LEN: 2B][KEY][VAL_LEN: 4B][VAL]`
- **Power Failure & Crash Recovery**: Reopening the journal scans sequentially, verifying each entry's CRC32 checksum. If a write was interrupted mid-record during power failure, the scanner halts cleanly at the last valid record without data corruption.
- **Compaction**: Periodically or on request, duplicate key mutations are compacted into a clean snapshot file and atomically renamed over the original WAL file.

### 2. Reactive Confinement & `Signal` Binding
- Because UI reactivity in Ferrix is thread-confined to the Rust Runtime Thread (using `Rc<RefCell<...>>`), reactive signals bound to storage keys (`use_storage`) are registered through thread-local storage rather than global cross-thread primitives.
- Modifying a storage key via `set` or `set_json` automatically updates the corresponding `Signal<T>`, triggering reactive rerendering of dependent elements.

### 3. Embedded SQLite & Live Queries (`ferrix-sqlite`)
- **WAL Mode & Pragmas**: Configured automatically on open:
  `PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL; PRAGMA foreign_keys = ON;`
- **Schema Migrations**: Forward-only versioned migrations applied inside atomic transactions with execution history recorded in `_ferrix_migrations`. Migration execution is idempotent.
- **Reactive Live Queries (`use_live_query`)**:
  - Live queries bind a SQL query to a reactive `Signal<Vec<T>>`.
  - Mutations executed via `execute_mutation` notify the table listener registry.
  - The live query re-executes automatically and updates the signal, delivering declarative data reactivity to UI components.

### 4. Verification Evidence
- `ferrix-storage`: 5 tests passing in `crates/ferrix-storage/tests/storage_tests.rs` (CRUD, JSON serialization, namespaces, WAL journal recovery, compaction, `use_storage` hook).
- `ferrix-sqlite`: 3 tests passing in `crates/ferrix-sqlite/tests/sqlite_tests.rs` (migrations, CRUD operations, live query invalidations).
- Workspace: 82 Rust tests passing, all Android Robolectric tests passing.
