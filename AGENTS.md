# Ferrix Agent Rules

## Core Architecture
- **Architecture A**: Rust owns application logic, component execution, reactive state, reconciliation, scheduling, and networking. Kotlin owns Android UI lifecycle, native View rendering, frame scheduling, ephemeral interaction state (focus, selection, scroll, animation progress), and platform APIs.
- **No Interpreter / No Reflection / No JS Engine**: Ahead-of-time compilation of `.fsx` UI files into Rust.
- **Three Independent Contracts**:
  1. Developer API (`*.fsx` language and component model)
  2. Runtime API (`ferrix-core`, `ferrix-reactivity`)
  3. Renderer Protocol (`ferrix-renderer-protocol` — depends on nothing else in the workspace)
- **Fine-Grained Reactivity**: State change updates only dependent nodes; never re-render the whole tree.

## JNI & Threading
- **Batched Command Buffers**: All UI mutations are sent as typed binary patch buffers with stable `u64` node IDs. Never make one JNI call per property. No ad-hoc JSON strings.
- **No Boundary Panics**: All Rust JNI entry points must catch panics (`std::panic::catch_unwind`) and translate errors safely to Kotlin.
- **Thread Separation**:
  - Android Main Thread: native View operations and input dispatch only. Never block on Rust work.
  - Rust Runtime Thread: component execution, state graph updates, and reconciliation. Never touch Android Views directly.
  - Rust Async Pool (`tokio`): networking and background I/O. Cancel async work on component unmount; never mutate destroyed instances.
- **Global References**: Release JNI global references deliberately when activities or instances are torn down.

## Layout & Animations
- **Layout Authority**: Rust (Taffy) computes layout geometry; Kotlin applies calculated frames to Views. No competing Kotlin layout pass. Handle density, font scale, system insets, and RTL.
- **Native Animations**: Animations run frame-by-frame on Android; Rust sends target values and animation specifications only.

## Code Quality Standards
- **Rust Standards**:
  - No `unsafe` without a descriptive `// SAFETY:` comment and an accompanying test.
  - Explicit error handling via `thiserror`; no `unwrap()` in production paths.
  - Structured logging via `tracing`.
  - Document all public crates, structs, traits, and functions.
- **Kotlin Standards**:
  - Lifecycle-aware coroutines; null-safe APIs.
  - Trait/interface-abstracted renderer so Jetpack Compose or other targets can be added later.
- **Performance is a Tested Claim**: Check in benchmarks or counters for JNI roundtrips, allocations, and frame times into `benchmarks/`.

## Operational Loop & Discipline
- **Loop**: ORIENT → PLAN → IMPLEMENT (small) → VERIFY → REFLECT → LOG.
- **Orient**: Read `AGENTS.md`, `.agent/milestones.md`, and relevant `.agent/learnings/` files before making changes.
- **Plan**: Create `.agent/plans/<yyyy-mm-dd>-<slug>.md` before touching code for any milestone slice.
- **Verify**: Always verify with real command execution: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --workspace`, and `./gradlew build`.
- **Learnings**: Keep `.agent/learnings/` up to date after every slice. A task is incomplete until learnings are logged.
