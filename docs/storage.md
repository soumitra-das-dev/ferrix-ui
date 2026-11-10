# Ferrix Storage Specification (F3)

## 1. Overview & Architecture
Ferrix Storage provides two complementary storage engines for Android apps under **Architecture A**:
1. **`ferrix-storage`**: High-performance reactive Key-Value store modeled on MMKV / AsyncStorage, with O(1) lock-free synchronous read cache backed by a crash-safe append-only journal file.
2. **`ferrix-sqlite`**: Reactive embedded SQLite database with schema migrations and live queries (`use_live_query`) driven by table-level invalidation channels.

---

## 2. Reactive KV Storage (`ferrix-storage`)

### 2.1 Sync Read / Async Durable Write Invariant
- **Synchronous O(1) Reads**: All keys are held in an in-memory `RwLock<HashMap<String, Vec<u8>>>` cache. Reading a key or reading a signal never incurs thread synchronization pauses, JNI latency, or disk stalls.
- **Append-Only Write Journal**: Writes update the in-memory cache immediately and append a checksummed record `[MAGIC: 4B][CRC32: 4B][KEY_LEN: 2B][KEY][VAL_LEN: 4B][VAL]` to a WAL journal file asynchronously.
- **Compaction**: Periodically or on restart, duplicate key records are compacted into a single snapshot.

### 2.2 API & Hooks
- `KvStore::open(path, namespace)`
- `store.get<T: DeserializeOwned>(&self, key: &str) -> Option<T>`
- `store.set<T: Serialize>(&self, key: &str, value: &T) -> Result<(), StorageError>`
- `store.delete(&self, key: &str) -> Result<(), StorageError>`
- `use_storage<T: Clone + Serialize + DeserializeOwned + 'static>(key: &str, default: T) -> (Signal<T>, Arc<dyn Fn(T)>)`

---

## 3. Reactive SQLite Database (`ferrix-sqlite`)

### 3.1 Dedicated Database Worker & Thread Safety
- SQLite queries and mutations execute on a dedicated single-threaded SQLite worker loop using `rusqlite` with WAL (Write-Ahead Logging) enabled.
- The UI / Rust Runtime Thread never blocks on SQLite transactions.

### 3.2 Schema Migrations
- Migrations are defined as forward-only SQL files or strings: `Migration { version: u32, name: &'static str, sql: &'static str }`.
- Applied atomically in a transaction with migration history recorded in `_ferrix_migrations`.

### 3.3 Live Queries (`use_live_query`)
- Queries register table dependencies (e.g. `SELECT * FROM todos WHERE completed = 0` depends on table `todos`).
- Any `INSERT`, `UPDATE`, or `DELETE` touching table `todos` broadcasts an invalidation notice over an internal channel.
- Registered live queries re-execute on the SQLite worker thread and emit updated result sets into a reactive `Signal<Vec<T>>`.
