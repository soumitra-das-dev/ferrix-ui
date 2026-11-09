# Reactive Runtime Knowledge

### 2026-11-09 Reactivity Baseline
- **Context**: Fine-grained reactivity model.
- **What happened / finding**: React-style full-component re-renders cause unnecessary VDOM diffing and JNI patch generation.
- **Root cause**: Fine-grained signals allow subscribing individual node properties directly to reactive signals.
- **Rule going forward**: Signal mutations queue dirty nodes directly into the batch buffer without evaluating unaffected siblings.
### 2026-11-09 Element Tree Diffing & Patch Generation
- **Context**: Structural diffing between declarative `Element` trees in `ferrix-core`.
- **What happened / finding**: Diffs between matching element trees emit minimal patches (`SetText`, `SetProperty`) directly targeting specific node IDs. Unchanged nodes produce zero patches.
- **Root cause**: Explicit NodeId tracking combined with property comparison ensures only mutated attributes generate binary command buffer instructions.
- **Rule going forward**: Always preserve stable node IDs during element reconciliation so Android View instances are reused and never needlessly destroyed or re-created.
### 2026-11-09 Keyed Reconciliation & Stable Platform IDs
- **Context**: Reconciling dynamic lists with additions, in-place toggles, and deletions in `ferrix-core`.
- **What happened / finding**: If child node IDs are reallocated monotonically per render, items that shift position appear to have new IDs, triggering unnecessary node destruction and recreation.
- **Root cause**: Keyed items represent the same platform View instance even when their position in the child list changes.
- **Rule going forward**: Always propagate the existing `old_child.id` to matching keyed children in `diff_keyed_children`, diffing subtrees using the stable platform ID. This ensures only deleted items emit `RemoveChild` + `DestroyNode`, while surviving items remain in the parent View hierarchy.
- **Evidence**: `crates/ferrix-core/tests/todo_app_test.rs`, `FerrixE2ETest.kt`.
