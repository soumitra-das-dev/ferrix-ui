# Agent Learnings Knowledge Base

This directory maintains persistent, verified operational knowledge across development sessions.

## Index of Files

| File | Content |
|---|---|
| [`decisions.md`](./decisions.md) | Architecture Decision Records (ADRs): context, options, decision, consequences |
| [`pitfalls.md`](./pitfalls.md) | Gotchas and failure modes with root causes and prevention rules |
| [`jni-android.md`](./jni-android.md) | Android NDK, JNI boundary, ABI, threading, and lifecycle findings |
| [`compiler.md`](./compiler.md) | FSX grammar, parsing, AST, type-checking, and codegen patterns |
| [`runtime.md`](./runtime.md) | Reactivity, fine-grained signals, hooks, reconciliation, and scheduling |
| [`perf-log.md`](./perf-log.md) | Dated benchmark results: metric, baseline, post-change, delta |
| [`commands.md`](./commands.md) | Verified commands for environment setup, building, testing, and debugging |
| [`backlog.md`](./backlog.md) | Out-of-scope issues and enhancement ideas to triage |
| [`milestone-01.md`](./milestone-01.md) | Retrospective and gate evidence for Milestone 1: Native Foundation |
| [`milestone-02.md`](./milestone-02.md) | Retrospective and gate evidence for Milestone 2: Element System |
| [`milestone-03.md`](./milestone-03.md) | Retrospective and gate evidence for Milestone 3: Reactive Runtime |
| [`milestone-04.md`](./milestone-04.md) | Retrospective and gate evidence for Milestone 4: Compiler & AOT Pipeline |
| [`milestone-05.md`](./milestone-05.md) | Retrospective and gate evidence for Milestone 5: Platform SDK & Lifecycle Cancellation |
| [`milestone-06.md`](./milestone-06.md) | Retrospective and gate evidence for Milestone 6: Network + Advanced UI |

## Standard Entry Format
```markdown
### <yyyy-mm-dd> <short title>
- Context: ...
- What happened / finding: ...
- Root cause: ...
- Rule going forward: ...
- Evidence: <command / test / commit hash>
```
