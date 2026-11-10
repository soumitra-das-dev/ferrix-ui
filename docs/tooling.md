# Ferrix Tooling Specification: Format, Lint & LSP (F6)

## 1. Overview & Architecture
Ferrix Developer Tooling (`ferrix fmt`, `ferrix lint`, `ferrix lsp`) operates directly on `.fsx` source files under **Architecture A**:
- **Lossless CST & Trivia Preservation**: The lexical analyzer preserves trivia (leading/trailing whitespace, newlines, line comments `//`, block comments `/* */`).
- **Idempotency Invariant (`ferrix-fmt`)**: The formatter satisfies `fmt(fmt(x)) == fmt(x)` for all valid inputs. It standardizes indentation, JSX tag wrapping, prop alignment, and brace positioning without stripping developer comments.
- **Static Analysis & Safety Rules (`ferrix-lint`)**:
  - `rules-of-hooks`: Prohibits hooks inside `if` statements, loops, or nested callbacks.
  - `require-keyed-children`: Requires `key` props on array iteration expressions.
  - `accessibility-pressable`: Requires `accessibilityLabel` or text content on interactive elements.
  - `unused-imports`: Detects dead import symbols.
  - `auto-fix`: Rewrites source code accurately using span ranges.
- **Language Server Protocol (`ferrix-lsp`)**: Standard JSON-RPC protocol exposing live diagnostics, hover docs, and formatting to IDEs (VS Code, Antigravity IDE, Neovim).

---

## 2. Formatter Contract (`ferrix-fmt`)
- Indentation: 4 spaces.
- JSX: Self-closing tags `<Tag />` when no children; multi-line props indented when exceeding line width.
- Comments: Preserved at original AST attachment locations.

---

## 3. Linter Rules & Diagnostics (`ferrix-lint`)
| Rule ID | Severity | Description | Fixable |
|---|---|---|:---:|
| `rules-of-hooks` | Error | Hooks cannot be called inside conditionals or loops | ❌ |
| `require-keyed-children` | Warning | Elements inside array maps require unique `key` prop | ❌ |
| `accessibility-pressable` | Warning | Pressable elements must have accessibilityLabel | ❌ |
| `unused-imports` | Warning | Unused import identifier detected | ✅ |
