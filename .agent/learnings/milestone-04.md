# Milestone 04 Retrospective: AOT Compiler & End-to-End Counter Gate

## Objective
Build the AOT compiler for `.fsx` files (`ferrix-compiler`), CLI (`ferrix-cli`), compile `examples/counter/component.fsx` ahead-of-time to Rust, and verify end-to-end execution on Android: exactly 1 JNI call per click, 1 `SetText` patch, and existing `TextView` instance reused.

## What Worked
1. **Ahead-of-Time JSX-to-Rust Pipeline**:
   The compiler pipeline (Lexer → Parser → AST → Semantic → Codegen) cleanly converts declarative `.fsx` files directly into Rust struct builders using `ferrix_core::prelude::*`.
2. **Dual-Mode Lexing for JSX**:
   Lexing normal expressions vs raw JSX children (text content up to `<` or `{`) requires careful state management in recursive descent parsers. Handling the transition upon `>` and element closing via `tag_depth` tracking proved 100% deterministic and avoided lookahead token stealing.
3. **Build.rs Integration**:
   By adding `ferrix-compiler` as a `build-dependency` in `ferrix-android`, UI files can be compiled automatically at `cargo build` time into `$OUT_DIR`, matching modern compile-time ergonomics.
4. **Tested Claims**:
   The gate asserts:
   - Exactly 1 JNI call per click.
   - Exactly 1 `SetText` patch.
   - Reused Android `TextView` instance (`assertSame` and identity hash code equality).
   Both Robolectric and live Android 34 emulator tests passed with screenshots saved to `docs/m4_screen.png`.

## What Didn't Work / Gotchas
- **Lexer Token Consumption on Lookahead**:
  Calling `expect(&TokenKind::Greater)` would call `advance()`, which fetched the next standard token from the lexer (skipping whitespace and consuming the first identifier of raw text). Transitioning immediately to `next_jsx_child()` when `self.current` is `>` without advancing the lexer resolved this cleanly.
- **Whitespace Handling**:
  Multi-line JSX templates with indentation need newlines and outer whitespace trimmed, while single-line interpolated text (e.g. `"Count: "`) must preserve trailing spaces for string formatting.

## Rule Going Forward
- Maintain `tag_depth` stack to switch lexer modes cleanly at JSX boundaries.
- Keep compiler independent of the runtime; codegen emits standard Rust using the public `ferrix-core` builder API.

