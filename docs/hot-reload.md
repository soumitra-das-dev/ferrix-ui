# Tiered Hot Reload in Ferrix

Ferrix provides an Expo/Fast-Refresh-inspired **three-tiered reload engine** that selects the fastest possible update mechanism based on what files changed.

---

## Reload Tiers

| Tier | Trigger | Recompilation Required | Latency Target | State Preserved |
| :--- | :--- | :--- | :--- | :--- |
| **Tier 1: Style / Assets / Config** | Modifying Tailwind styles, themes, assets, strings, or config | **None** | **< 300 ms** | **100% Preserved** (all signals, form inputs, navigation stack kept) |
| **Tier 2: Component Fast Refresh** | Editing `.fsx` UI files without native structural changes | Incremental AOT | **< 2.5 s** | **Preserved by Component & Hook Signature** |
| **Tier 3: Native Rebuild** | Modifying `AndroidManifest.xml`, Gradle build scripts, or adding native Rust crates | Full `cargo ndk` + Gradle `assembleDebug` | **~10–25 s** | Persisted KV state and router state restored where possible |

---

## What Preserves State vs. What Resets State

### 1. State Kept (Fast Refresh)
- Signal values (`useState`) remain intact when changing element layout, colors, padding, child ordering, or handler logic within the same component signature.
- Scroll position and focus state in native views are preserved across Tier 1 and Tier 2 updates.

### 2. State Reset
- If a component's hook signature changes (e.g., adding or removing a `useState` or `useQuery` call), that specific component's reactive state resets to its initial value to prevent state corruption.
- Tier 3 updates (re-installing the APK) reset in-memory heap state, though persistent storage (`ferrix-storage`) and SQLite data remain safely on disk.

---

## Error Handling & Error Overlay

- When a compilation error occurs in a `.fsx` file during dev mode:
  1. The running UI on device continues displaying the last known good state.
  2. A clean compiler diagnostic is emitted to the dev server terminal indicating `file:line:col`.
  3. The error auto-clears on the next successful file save without needing a manual refresh.

