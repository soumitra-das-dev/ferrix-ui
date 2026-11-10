# Framework Capabilities (F1 → F8) Implementation Plan

**Date**: 2026-11-09
**Status**: In Progress

---

## 1. Feature Dependency Graph

```mermaid
flowchart TD
    F1[F1: Router (ferrix-router)] --> F2[F2: Navigation (ferrix-navigation + Kotlin containers)]
    F2 --> F3[F3: Storage (ferrix-storage, ferrix-sqlite)]
    F3 --> F4[F4: State Management & Forms (ferrix-store, ferrix-forms)]
    F4 --> F5[F5: Config, Env, Assets, i18n]
    F5 --> F6[F6: Tooling (ferrix-fmt, ferrix-lint, ferrix-lsp)]
    F6 --> F7[F7: Testing Framework (ferrix-test)]
    F7 --> F8[F8: Errors, Logging, DevTools & Services]
```

---

## 2. Architecture Decisions & Design Invariants

### 2.1 F1: Router Architecture
- **State in Rust**: Routing stack, history, active segments, local search params, and global query params reside entirely in Rust `RouterState`.
- **File System Tree Scanning**: Directory scan maps `app/` conventions (`index.fsx`, `_layout.fsx`, `[id].fsx`, `[...rest].fsx`, `(group)/`, `+not-found.fsx`, `+error.fsx`) to a static route manifest at compile time.
- **Deep Linking**: Intent filter patterns generated from `ferrix.config.toml` route cold and warm URL starts to path resolution.

### 2.2 F2: Navigation Container Strategy (ADR)
- **Custom ViewContainers over Android Fragments**:
  - *Context*: Android `Fragment` / `FragmentManager` introduces complex transaction scheduling, asynchronous lifecycle callbacks, and competing back-stack management that conflicts with Rust's single source of truth.
  - *Decision*: Navigation containers are implemented as lightweight custom Android `ViewGroup` classes (`FerrixNavContainer`, `FerrixScreenView`, `FerrixHeaderView`, `FerrixTabBarView`) directly driven by typed binary JNI commands (`Navigate`, `GoBack`, `SetScreenOptions`).
  - *Transitions*: Native frame-by-frame Android property animators (slide, fade, bottom-sheet) execute in Kotlin without blocking Rust.

### 2.3 F3: Storage Backend Strategy (ADR)
- **KV Store**: `ferrix-storage` uses an in-memory lock-free sync read cache backed by a crash-safe append-only binary journal / file storage. Read operations are $O(1)$ and never block on JNI or async I/O; writes are flushed asynchronously to durable storage with checksums.
- **SQLite Engine**: `ferrix-sqlite` uses bundled SQLite on a dedicated worker thread with WAL (Write-Ahead Logging) enabled. Schema migrations are numbered, forward-only, and checksum-verified. Live queries (`use_live_query`) register table observers and re-execute queries automatically on mutations, diffing results into minimal signal updates.

### 2.4 F4: State Management & Forms
- **Global Store**: Redux/Zustand-style reactive stores (`createStore`) with selector subscriptions (`useStore(|state| &state.user)`) updating only dependent nodes.
- **Forms**: Controlled input model with field-level dirty/touched tracking, synchronous and async validation rules, and monotonic version counters preventing cursor jumps.

### 2.5 F5: Config, Env, Assets, i18n
- **Schema Validation**: `ferrix.config.toml` validated against a formal JSON schema with line-accurate diagnostics.
- **Environment**: Build-time injection (`.env`, `.env.<variant>`) ensuring secret variables are never exposed in client binaries.
- **i18n**: ICU message format parser with pluralization, number/date formatting, and runtime locale/RTL toggling.

### 2.6 F6: Lossless CST & Tooling
- **Lossless CST**: Compiler token stream refactored to preserve trivia (leading/trailing whitespace, newlines, comments) enabling idempotent formatting in `ferrix fmt` (`fmt(fmt(x)) == fmt(x)`) and precise source code rewriting in `ferrix lint --fix`.

### 2.7 F7: Testing Framework
- **Headless Test Renderer**: `ferrix-test` executes components on the host platform without Android emulation. Dispatches synthetic events (`press`, `change_text`), handles mock timers, and queries elements by text, role, or testID.

### 2.8 F8: Error Boundaries & DevTools
- **Fault Isolation**: Rust-level `ErrorBoundary` catches component panic/errors during render, displaying a fallback UI instead of crashing the process. In-app dev overlay exposes live signal graphs, frame times, and network inspection.

---

## 3. Execution & Gate Roadmap

| Step | Feature | Gate Target & Verification Command |
|---|---|---|
| **F1** | Router | `examples/router/` tests with nested layouts, dynamic/group routes, typed hrefs: `cargo test -p ferrix-router` |
| **F2** | Navigation | Native Stack & Tabs containers with predictive back & 60fps transitions: `cargo test -p ferrix-navigation && cd android && ./gradlew testDebugUnitTest --tests com.ferrix.runtime.NavigationTest` |
| **F3** | Storage | KV store + SQLite live query + migration tests: `cargo test -p ferrix-storage -p ferrix-sqlite` |
| **F4** | State & Forms | Persisted store + optimistic mutations + validated form test: `cargo test -p ferrix-store -p ferrix-forms` |
| **F5** | Config & i18n | Schema validation + variant env injection + ICU translation test: `cargo test -p ferrix-compiler` |
| **F6** | Lint & Format | CST preservation + formatter idempotency + lint rule suite: `cargo test -p ferrix-fmt -p ferrix-lint` |
| **F7** | Testing | Headless component tests + query assertions: `cargo test -p ferrix-test` |
| **F8** | Errors & DevTools | Error boundary fault isolation + dev inspector: `cargo test --workspace` |

