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
- **Evidence**: `crates/ferrix-core/tests/element_system_tests.rs`, `FerrixE2ETest.kt`.
