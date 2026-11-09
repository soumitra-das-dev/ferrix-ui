# Compiler & FSX Knowledge

### 2026-11-09 FSX Architecture Baseline
- **Context**: Compiling `.fsx` files to native Rust ahead of time.
- **What happened / finding**: `.fsx` must support JSX-like elements embedded directly in functional component bodies with static typing.
- **Root cause**: Eliminating JavaScript runtime requires generating direct Rust AST nodes calling `ferrix-core` reactive primitives.
- **Rule going forward**: The specification in `docs/fsx-spec.md` is the formal grammar and semantic contract; the parser must strictly adhere to it.
- **Evidence**: `docs/fsx-spec.md`

### 2026-11-09 Dual-Mode JSX Lexing and Lookahead Synchronization
- **Context**: Parsing nested JSX tags like `<Column><Text size={24}>Ferrix Counter</Text>...</Column>`.
- **What happened / finding**: Standard `advance()` calls on closing `>` consumed the next token in standard expression mode, eating the first token of raw text content and misaligning closing tags.
- **Root cause**: Standard 1-token lookahead parser buffers the next token eagerly. At JSX element body boundaries, the next token must be fetched with `next_jsx_child()` instead of `next_token()`.
- **Rule going forward**: Track `tag_depth: usize`. When consuming `>`, immediately transition to `next_jsx_child()`. When `tag_depth == 0` upon closing the root JSX tag, transition back to `next_token()`.
- **Evidence**: `crates/ferrix-compiler/src/parser.rs`, `crates/ferrix-compiler/tests/compiler_tests.rs`.

