# Milestone 02 Retrospective: Element System

## Objective
Implement the core Element model, builder primitives (`Column`, `Row`, `Text`, `Button`), stable Node ID allocation, initial patch batch generation, and fine-grained reconciliation/diffing in `crates/ferrix-core`. Enhance Android runtime Kotlin renderer to handle all four primitives, nested layout, and dynamic patch applications.

**Gate**: Column / Row / Text / Button rendered from Rust on an Android emulator/device.

---

## Findings & Execution Log

### 2026-11-09 Element System & Diffing Architecture
- **Declarative Element Trees**: `crates/ferrix-core` provides `Element`, `ElementDef`, and builders for `Column`, `Row`, `Text`, and `Button`. Definitions are resolved into concrete `Element` trees with allocated `NodeId` values using `NodeIdAllocator`.
- **Mount Patch Generation**: `generate_mount_patches` performs depth-first serialization into `CreateNode`, `SetProperty`, `SetText`, `AttachEventListener`, and `AppendChild` patches.
- **Incremental Reconciliation**: `diff_elements` compares old and new trees:
  - If node types or IDs mismatch, replaces the node via `DestroyNode` and re-mounts.
  - If matching: diffs text (`SetText`), properties (`SetProperty`), event listeners (`AttachEventListener` / `DetachEventListener`), and reconciles children (`InsertBefore`, `AppendChild`, `RemoveChild`, `DestroyNode`).
  - When only state-bound text changes (e.g. Counter), exactly 1 `SetText` patch is emitted; native Views are neither recreated nor re-parented.
- **Android Spacing in LinearLayout**: Setting `dividerPadding` on `LinearLayout` does nothing without an actual divider drawable. By creating a transparent `ShapeDrawable` with `intrinsicHeight` (for Column) or `intrinsicWidth` (for Row) and enabling `SHOW_DIVIDER_MIDDLE`, Android automatically and cleanly spaces children without altering individual child margins.
- **Robolectric & Live Device Testing**:
  - JUnit/Robolectric test `FerrixE2ETest` dynamically loads the compiled host `libferrix_android.so` and validates all 4 view types, tree hierarchy, and interactive click event diffing.
  - Live Android emulator (`emulator-5554`) verification validates rendering of all 4 view types, spacing, and interaction, documented in `docs/m2_screen.png`.

## Gate Evaluation: PASSED 🟢
- Column / Row / Text / Button rendered from Rust: **Verified**.
  - Root `LinearLayout` (Column, vertical)
  - Sub-views: `TextView` ("Ferrix Element System (M2)"), `TextView` (subtitle), `LinearLayout` (Row, horizontal with 2 `Button`s), and `TextView` ("Counter: 0").
- Interactive click testing: **Verified** (Increment -> 2, Reset -> 0).
- Evidence: UI hierarchy dump `/data/local/tmp/dump_m2.xml`, screenshot `docs/m2_screen.png`.
