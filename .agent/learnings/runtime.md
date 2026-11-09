# Reactive Runtime Knowledge

### 2026-11-09 Reactivity Baseline
- **Context**: Fine-grained reactivity model.
- **What happened / finding**: React-style full-component re-renders cause unnecessary VDOM diffing and JNI patch generation.
- **Root cause**: Fine-grained signals allow subscribing individual node properties directly to reactive signals.
- **Rule going forward**: Signal mutations queue dirty nodes directly into the batch buffer without evaluating unaffected siblings.
- **Evidence**: `ferrix-reactivity` architecture.
