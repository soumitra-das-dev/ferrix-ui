# F6 Learnings: Lint, Format & LSP Tooling (`ferrix-fmt`, `ferrix-lint`, `ferrix-lsp`)

## Architecture Decisions & Invariants
Developer ergonomics and code health in Ferrix are enforced through static toolchains operating ahead of time on `.fsx` source code.

### 1. Lossless Formatter (`ferrix-fmt`)
- `.fsx` code formatting must be **idempotent**: `format(format(source)) == format(source)`.
- Formatter preserves comment trivia (`//` line comments, `/* */` block comments) and user blank lines while standardizing indentation (2 spaces) and JSX-like tag spacing.
- Guarantees zero code corruption while providing uniform style across the codebase.

### 2. Static Linter & Rule Engine (`ferrix-lint`)
- Analyzes `.fsx` source code to enforce reactive invariants and accessibility requirements:
  - `rules_of_hooks`: Hooks must only be invoked at the top level of component functions, never inside loops, branches, or nested callbacks.
  - `require_keyed_children`: Elements produced dynamically inside array iterations must provide a unique `key` prop to preserve DOM/View identity across diffs.
  - `accessibility_pressable`: Interactive elements lacking textual children must supply an `accessibilityLabel` for screen readers (TalkBack).
  - `unused_imports`: Flags unused imported symbols to maintain clean bundles.
- Includes `apply_fixes()` supporting automated code fixes directly applied to source files.

### 3. Language Server Protocol (`ferrix-lsp`)
- Standard JSON-RPC 2.0 server integration for IDEs (VS Code, Neovim, JetBrains):
  - `initialize`: Advertises document sync, formatting, and hover capabilities.
  - `textDocument/didOpen` & `textDocument/didChange`: Publishes diagnostics using `ferrix-lint`.
  - `textDocument/formatting`: Invokes `ferrix-fmt` to format code on save.
  - `textDocument/hover`: Delivers type signatures and documentation for built-in hooks (`useState`, `useSignal`, `useRouter`, `useStorage`) and primitive components (`Column`, `Text`, `Button`).

### 4. Verification Evidence
- `ferrix-fmt`: 2 tests passing in `crates/ferrix-fmt/tests/fmt_tests.rs`.
- `ferrix-lint`: 4 tests passing in `crates/ferrix-lint/tests/lint_tests.rs`.
- `ferrix-lsp`: 3 tests passing in `crates/ferrix-lsp/tests/lsp_tests.rs`.
- Total workspace: 108 tests passing across 20 crates, 0 clippy warnings.
