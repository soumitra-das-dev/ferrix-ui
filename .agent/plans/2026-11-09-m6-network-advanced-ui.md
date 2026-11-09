# Plan: Milestone 06 — Network + Advanced UI

## 1. Goal
Implement the networking layer, reactive query/mutation hooks, and advanced UI capabilities:
1. **Network Layer (`ferrix-network`)**:
   - Trait-abstracted HTTP client (`HttpClient`, `HttpRequest`, `HttpResponse`).
   - Honor timeout, cancellation, headers, JSON body.
2. **Data Fetching Hooks (`useQuery`, `useMutation`)**:
   - `use_query`: `Loading` -> `Success(T)` / `Error(String)`.
   - `use_mutation`: Triggerable async mutation with state tracking.
   - Unmount safety: cancels pending async requests on unmount.
3. **Advanced UI**:
   - **Lazy Lists / Recycling**: Virtualized list container (`LazyColumn`) sending item keys and count; Kotlin maps to reusable ViewHolders.
   - **Native Animations**: Declarative animation specs (`AnimationSpec { duration_ms, target_value }`); Kotlin executes on RenderThread frame-by-frame.
   - **Image Loading**: `Image` component with URL source loaded asynchronously in Kotlin.
4. **Milestone 6 Gate**:
   **API-Driven Application**:
   - Fetches remote data using `use_query`.
   - Displays items in a scrollable list with images.
   - Runs native Android animation on item interaction.
   - Mutates data via `use_mutation` and re-renders dynamically.

---

## 2. Architecture & Contracts

### 2.1 Networking (`ferrix-network`)
```rust
#[async_trait::async_trait]
pub trait HttpClient: Send + Sync {
    async fn execute(&self, req: HttpRequest) -> Result<HttpResponse, NetworkError>;
}
```
Hooks:
```rust
pub struct QueryResult<T> {
    pub data: Option<T>,
    pub is_loading: bool,
    pub error: Option<String>,
}

pub fn use_query<T: Clone + Send + 'static>(
    cx: &mut ComponentContext,
    key: &str,
    fetcher: impl Fn() -> Result<T, String> + Send + 'static,
) -> QueryResult<T>
```

### 2.2 Advanced UI Protocol (`ferrix-renderer-protocol`)
- Native animation property:
  `Property::AnimateAlpha { target: f32, duration_ms: u32 }`
  `Property::AnimateScale { target: f32, duration_ms: u32 }`
  `Property::AnimateTranslationY { target: f32, duration_ms: u32 }`
- Image component:
  `Property::ImageUrl(String)`
- List item virtualization:
  `ViewType::LazyColumn`, child templates with stable keys.

### 2.3 Kotlin Renderer Mapping (`android/ferrix-runtime`)
- `FerrixViewFactory`:
  - `ViewType.Image` -> `ImageView`
  - `ViewType.LazyColumn` / `ScrollView` -> `ScrollView` / `LinearLayout` recycling container
- `FerrixRenderer`:
  - Handles `AnimateAlpha`, `AnimateScale`, `AnimateTranslationY` via native `ViewPropertyAnimator` frame-by-frame on Android Main/Render thread.

---

## 3. Files to Touch / Create

### 1. `ferrix-network`
- `crates/ferrix-network/src/lib.rs` (Traits, Request, Response, Client, MockClient)
- `crates/ferrix-network/src/query.rs` (Query & Mutation state machines)
- `crates/ferrix-network/tests/network_tests.rs`

### 2. `ferrix-core`
- `crates/ferrix-core/src/query_hooks.rs` (`use_query`, `use_mutation`)
- `crates/ferrix-core/src/primitives.rs` (Add `Image`, `LazyColumn`, `ScrollView` builders)

### 3. `ferrix-renderer-protocol`
- Update `Property` enum with `ImageUrl`, `AnimateAlpha`, `AnimateScale`, `AnimateTranslationY`

### 4. `android/ferrix-runtime`
- Update `Types.kt`, `CommandDecoder.kt`, `FerrixViewFactory.kt`, `FerrixRenderer.kt` to handle `Image` and native `ViewPropertyAnimator`

### 5. `crates/ferrix-android`
- Add API-Driven App mode (`AppState::ApiApp`) fetching items, loading images, animating on tap, and mutating items.

### 6. Tests & Gate Verification
- `crates/ferrix-network/tests/network_tests.rs`
- `android/ferrix-runtime/src/test/java/com/ferrix/runtime/ApiDrivenAppTest.kt`
- Live Android emulator deployment & screenshot `docs/m6_screen.png`.

---

## 4. Verification Commands
- `cargo fmt --check`
- `cargo clippy --workspace -- -D warnings`
- `cargo test --workspace`
- `cargo ndk -t arm64-v8a -t x86_64 -o android/ferrix-runtime/src/main/jniLibs build --release`
- `cd android && ./gradlew assembleDebug`
- `cd android && ./gradlew test`

