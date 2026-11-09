# Compiler & FSX Knowledge

### 2026-11-09 FSX Architecture Baseline
- **Context**: Compiling `.fsx` files to native Rust ahead of time.
- **What happened / finding**: `.fsx` must support JSX-like elements embedded directly in functional component bodies with static typing.
- **Root cause**: Eliminating JavaScript runtime requires generating direct Rust AST nodes calling `ferrix-core` reactive primitives.
- **Rule going forward**: The specification in `docs/fsx-spec.md` is the formal grammar and semantic contract; the parser must strictly adhere to it.
- **Evidence**: `docs/fsx-spec.md`
