# Ferrix State Management & Forms Specification (F4)

## 1. Overview & Architecture
Ferrix State Management & Forms delivers global application state and validated input pipelines under **Architecture A**:
- **Fine-Grained Selector Subscriptions (`ferrix-store`)**: Reactive stores execute selectors (`use_store(|state| &state.user.name)`). Only components whose selected slice has changed are scheduled for reconciliation. Unrelated state modifications produce zero VDOM diffing or native JNI patches.
- **Controlled Input & Monotonic Cursor Synchronization (`ferrix-forms`)**: Fast typing on native Android `EditText` triggers high-frequency text events. Monotonic version counters track text generations to prevent cursor jumping or echo loops when state is validated.

---

## 2. Global State Engine (`ferrix-store`)

### 2.1 Store Definition & Actions
```rust
pub struct Store<S> {
    pub fn new(initial: S) -> Self;
    pub fn get_state(&self) -> S;
    pub fn update<F: FnOnce(&mut S)>(&self, updater: F);
    pub fn select<T: Clone + PartialEq + 'static, F: Fn(&S) -> T + 'static>(&self, selector: F) -> Signal<T>;
}
```

### 2.2 Middleware Pipeline
- `with_persistence(storage_key, store)`: Automatically rehydrates store from `ferrix-storage` and persists state transitions.
- `with_logging(store)`: Logs dispatched actions and state transitions with `tracing`.

---

## 3. Form Validation Engine (`ferrix-forms`)

### 3.1 Field State & Validation
- **FieldState**: Holds `value: T`, `is_dirty: bool`, `is_touched: bool`, `error: Option<String>`, `version: u64`.
- **Validation Rules**: Synchronous predicate validators (`required`, `min_length`, `email`, `regex`) and async rules returning `Result<(), String>`.

### 3.2 Cursor Jump Prevention
- Traditional React Native architectures suffer from cursor jumping when native `EditText` emits a change event, asynchronous roundtrips update state, and the runtime sets the text back.
- Ferrix tracks a monotonic `version: u64` per field. When native text events arrive, Ferrix updates the internal field value and increments `version`. The native `SetText` patch is emitted only if the sanitized value differs from the user's keystroke, completely eliminating cursor repositioning.

