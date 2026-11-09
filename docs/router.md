# Ferrix Router Specification (F1)

## 1. Overview & Architecture
Ferrix Router provides a file-based, statically typed routing and deep-linking system modeled on Expo Router / Next.js App Router, engineered under **Architecture A**:
- **Single Source of Truth in Rust**: Router state (navigation stack, history, active segments, search parameters, focus state) is owned exclusively by the Rust runtime. State is fully serializable (`serde`) and survives Android Activity recreation and process death.
- **Ahead-Of-Time (AOT) Route Manifest**: At compile time, the `app/` directory hierarchy is parsed into a deterministic, validated `RouteManifest`. Ambiguous, conflicting, or duplicate route patterns emit compile-time diagnostics.
- **Typed Navigation**: Route paths, dynamic segments (`[id]`), and query parameters are statically typed in generated Rust code, ensuring invalid URLs or missing parameters fail at compile time.
- **Lifecycle Integration**: Screens execute fine-grained reactive lifecycles (`mount`, `focus`, `blur`, `unmount`). Navigating away triggers `blur`; unmounting cancels in-flight async operations and signals cooperative cancellation tokens.

---

## 2. Directory Conventions (`app/`)

| File / Folder Convention | Meaning | Example Path | URL Route |
|---|---|---|---|
| `index.fsx` | Route entry for the current folder | `app/index.fsx` | `/` |
| `[id].fsx` | Dynamic route segment | `app/users/[id].fsx` | `/users/42` (`params.id = "42"`) |
| `[...rest].fsx` | Catch-all wildcard segment | `app/docs/[...rest].fsx` | `/docs/guides/intro` |
| `_layout.fsx` | Nested layout wrapping sub-routes | `app/users/_layout.fsx` | Wraps all `/users/*` screens |
| `(group)/` | Route group (organizes files without adding a URL segment) | `app/(auth)/login.fsx` | `/login` |
| `+not-found.fsx` | Fallback screen for unmatched routes | `app/+not-found.fsx` | Any 404 URL |
| `+error.fsx` | Route-level error boundary screen | `app/+error.fsx` | Rendered on uncaught route error |
| `(modal)/` or `modal.fsx` | Modal presentation route | `app/compose.fsx` | Overlays parent screen |

---

## 3. Router API & Hooks

### 3.1 `use_router()`
Provides imperative navigation commands:
```rust
pub struct Router {
    pub fn push(&self, href: &str);
    pub fn replace(&self, href: &str);
    pub fn back(&self);
    pub fn dismiss(&self);
    pub fn navigate(&self, href: &str);
    pub fn can_go_back(&self) -> bool;
}
```

### 3.2 URL Inspection Hooks
- `use_pathname() -> Signal<String>`: Returns current pathname (e.g., `"/users/42"`).
- `use_segments() -> Signal<Vec<String>>`: Returns segment list (e.g., `["users", "42"]`).
- `use_local_search_params() -> Signal<HashMap<String, String>>`: Returns params extracted from the current route pattern (e.g. `{ "id": "42" }`).
- `use_global_search_params() -> Signal<HashMap<String, String>>`: Returns global query string parameters (e.g. `?tab=settings`).

### 3.3 Screen Focus Hooks
- `use_is_focused() -> Signal<bool>`: Returns true if the screen is currently active and focused.
- `use_focus_effect(callback)`: Runs an effect when the screen gains focus; runs cleanup when the screen loses focus or unmounts.

### 3.4 Declarative Elements
- `<Link href="/profile">`: Declarative navigation component wrapping interactive elements.
- `<Redirect href="/login">`: Immediately redirects navigation when rendered in a route guard / middleware.

---

## 4. Deep Linking & State Restoration
- **Intent Filters**: `ferrix.config.toml` specifies custom URI schemes (e.g., `my-app://`) and Android App Link domains (e.g., `https://my-app.com`).
- **Warm / Cold Start**: When an Android intent arrives, the deep link URL is normalized and matched against the route manifest.
- **Process Death**: `RouterState::serialize()` saves the full navigation stack to disk/Bundle; `RouterState::restore()` recovers the exact user screen stack on restart.

