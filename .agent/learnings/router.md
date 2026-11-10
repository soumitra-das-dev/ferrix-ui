# F1 Learnings: File-Based Router (`ferrix-router`)

## Architecture A Routing Paradigm
Under Architecture A, the routing system is owned entirely by the Rust runtime. There is no JavaScript engine, no dynamic reflection, and no client-side navigation routing loop in Kotlin.

### 1. Single Source of Truth in Rust
- The navigation stack, history, active segments, local search params, and global query params reside exclusively in `RouterState` inside the Rust runtime.
- Kotlin does not maintain an independent fragment back-stack or duplicate route history. When Android back button presses or deep links occur, Kotlin forwards typed events to Rust, which performs state transitions and pushes updated command batches.

### 2. AOT Virtual Directory Scanning (`app/`)
- Directory scanning maps Expo Router / Next.js App Router conventions directly into a statically verified `RouteManifest`:
  - `index.fsx` $\to$ `/`
  - `[id].fsx` $\to$ `Dynamic(id)` segment
  - `[...rest].fsx` $\to$ `CatchAll(rest)` segment
  - `(group)/` $\to$ `Group` pattern (organizes routes without consuming a URL segment)
  - `+not-found.fsx` $\to$ 404 fallback route
  - `+error.fsx` $\to$ Route-level error boundary
- Validation ensures duplicate route definitions or ambiguous segment collisions emit compile-time errors rather than runtime 404s.

### 3. Fine-Grained Reactive Navigation Hooks
- Hooks (`use_pathname`, `use_segments`, `use_local_search_params`, `use_global_search_params`, `use_can_go_back`, `use_is_focused`) return fine-grained `Signal<T>` containers.
- When navigation occurs, only elements depending on the modified signals are scheduled for re-evaluation; unchanged screen headers or sibling components are not invalidated.

### 4. Process Death & Serialization
- `RouterState::snapshot()` serializes the active stack, history, and route matches into a compact, versioned JSON format (`RouterSnapshot`).
- Android Activity recreation or process death recovery restores the complete user stack via `RouterState::restore()`, maintaining seamless continuity without losing navigation context or route parameters.

### 5. Deep Link Normalization
- Deep link parser handles both custom schemes (`myapp://...`) and universal/App Links (`https://...`), extracting path and query parameters into canonical route targets.

### 6. Verification
- 8 tests passing in `crates/ferrix-router` (`cargo test -p ferrix-router`).
- Full workspace tests passing (71 tests across 11 crates).
- Android test suite passing (`./gradlew testDebugUnitTest`).

