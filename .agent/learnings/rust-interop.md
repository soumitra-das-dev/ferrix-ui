# Learnings: Rust Interop from `.fsx` and Crate Management (Part 2)

## 1. Architectural Patterns
- **Architecture A Preservation**: All Rust functions and crate invocations execute entirely in the native Rust runtime thread without reflection, JS interpreters, or dynamic marshalling overhead.
- **Ahead-of-Time Compilation of Interop**:
  - `import { fib } from "./math.rs"` compiles to `#[path = "..."] pub mod math; pub use math::{fib};`.
  - `import { Uuid } from "rust:uuid"` compiles to `pub use uuid::{Uuid};`.
  - Raw `rust { ... }` blocks are extracted verbatim at compile-time and emitted before component definitions in the generated Rust files.
- **Path and Method Call Disambiguation**:
  - In `.fsx` expressions, `Uuid.new_v4()` or `Uuid::new_v4()` where the receiver starts with an uppercase letter represents an associated function/type constructor (`Uuid::new_v4()`).
  - Method chains on expressions (like `Uuid::new_v4().to_string()`) use dot notation (`.to_string()`).
  - In codegen, formatting `Expr::Member` as `::` is strictly restricted to cases where the parent is an `Expr::Ident` beginning with an ASCII uppercase character.

## 2. Compiler & Lexer Nuances
- **Raw Rust Blocks vs FSX Lexer**:
  - Raw Rust code frequently contains characters invalid or ambiguous in JSX/FSX (`&str`, `|x|`, `->`, `*const`, `#[inline]`, `<T>`).
  - Attempting to tokenize raw Rust blocks with the standard FSX lexer causes lexer errors (e.g. `Unexpected character '&'`).
  - Solution: When the parser encounters `rust {`, it delegates to `read_raw_rust_block()` on the lexer. This method reads raw characters until matching brace depth reaches 0, properly ignoring braces within comments and string literals, and returns the raw string without attempting to tokenize Rust syntax as FSX tokens.
- **Module Path Resolution in Cargo `OUT_DIR`**:
  - When `build.rs` compiles `.fsx` files to `$OUT_DIR/<stem>_generated.rs`, any `#[path = "./math.rs"]` attribute is resolved by `rustc` relative to `$OUT_DIR`, which fails because `math.rs` is in the project's `src/` directory.
  - Solution: In `Codegen`, provide an optional `base_dir: Option<PathBuf>`. When provided (such as in `build.rs` via `input.parent()`), relative `.rs` module paths are canonicalized against `base_dir` into absolute paths (`#[path = "/abs/path/src/math.rs"]`), ensuring clean compilation across arbitrary output directories.

## 3. String Literal Escaping in Codegen
- When emitting string literals (`Expr::StringLit(s, _)`), formatting with `format!("\"{}\"", s)` fails when `s` contains unescaped quotes (e.g. JSON payloads like `{"role":"admin"}`).
- Solution: Format using debug representation `format!("{:?}", s)` which standardizes escaping of internal quotes, newlines, and escape characters for Rust string literals.

## 4. CLI Crate Management & Supply-Chain Security
- `ferrix add crate <name> [--features <f>] [--no-default-features]`:
  - Scans against known Android pitfalls (`openssl` needing platform `.so`s, `native-tls` failing to link on Android, `dirs` violating app sandbox).
  - Runs `cargo add <name>`.
  - Performs an immediate Android cross-compilation check (`cargo check --target x86_64-linux-android`).
  - Ensures a `deny.toml` file exists with standard licenses (`MIT`, `Apache-2.0`, `BSD-3-Clause`) and security advisory checks.
- `ferrix remove crate <name>` and `ferrix crates list`:
  - Safely manages and inspects dependencies in `Cargo.toml`.

## 5. Verification Evidence
- Tested compiler unit and integration tests:
  - `crates/ferrix-compiler/tests/rust_interop_compiler_tests.rs`: verified local `.rs` module imports, crate imports, and inline `rust { ... }` blocks.
  - `crates/ferrix-cli/tests/crate_cli_tests.rs`: verified pitfall warnings, dependency parsing, and `deny.toml` generation.
- Full live smoke test on `emulator-5554`:
  - Golden sample app `examples/rust-interop/` containing `math.rs` + `uuid` + `regex` + `serde_json` + inline `rust { ... }`.
  - Built debug APK, installed via ADB, validated `ferrix:ready`, passed synthetic button click, verified hot-reload patch in 47.22s with zero errors.
