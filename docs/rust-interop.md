# Ferrix Rust Interop & Crate Ecosystem

Ferrix allows developers to import and execute native Rust code directly from `.fsx` declarative UI files under **Architecture A**. All Rust computations run on the Rust runtime thread, compile ahead-of-time (AOT) to native ARM64/x86_64 machine code, and never touch JavaScript runtimes, reflection, or ad-hoc interpreters.

---

## 1. Local Rust Module Imports (`*.rs`)

Local `.rs` files located within the project's `src/` directory can be imported using standard ES-style syntax referencing the `.rs` extension:

```fsx
import { fib, Counter, Status } from "./math.rs";
import { Column, Text, Button } from "ferrix/ui";
import { useState } from "ferrix/reactivity";

export function MathScreen(): Element {
  const count = useState(5);

  return (
    <Column padding={16} spacing={8}>
      <Text size={20}>Fibonacci: {fib(count.value)}</Text>
      <Button onClick={() => count.set(count.value + 1)}>Increment</Button>
    </Column>
  );
}
```

### Generated Rust Output
The Ferrix compiler resolves the relative module path into an AOT module declaration with canonical path resolution and re-exports:

```rust
#[path = "/absolute/project/path/src/math.rs"]
pub mod math;
pub use math::{fib, Counter, Status};
```

---

## 2. Cargo Crate Imports (`rust:<crate>`)

External Rust crates published to [crates.io](https://crates.io) or installed in `Cargo.toml` can be imported directly into `.fsx` using the `rust:` prefix:

```fsx
import { Uuid } from "rust:uuid";
import { Regex } from "rust:regex";
import { json, Value } from "rust:serde_json";
```

### Generated Rust Output
```rust
pub use uuid::{Uuid};
pub use regex::{Regex};
pub use serde_json::{json, Value};
```

Expressions can call crate functions and associated type constructors directly:
```fsx
<Text>UUID: {Uuid::new_v4().to_string()}</Text>
```

---

## 3. Inline `rust { ... }` Blocks

When small utility functions, serialization helpers, or complex algorithmic transforms are needed within a component file without creating a standalone `.rs` module, developers can declare an inline `rust { ... }` block at the top level of `.fsx`:

```fsx
rust {
    pub fn format_greeting(name: &str) -> String {
        format!("Welcome, {}! (from native Rust)", name)
    }

    pub fn parse_meta(raw_json: &str) -> String {
        let val: serde_json::Value = serde_json::from_str(raw_json).unwrap_or(serde_json::Value::Null);
        val.get("role").and_then(|r| r.as_str()).unwrap_or("guest").to_string()
    }
}

export function Profile(): Element {
    const raw = useState("{\"role\":\"admin\"}");
    return (
        <Column>
            <Text>{format_greeting("Alice")}</Text>
            <Text>Role: {parse_meta(raw.value)}</Text>
        </Column>
    );
}
```

Inline Rust blocks are emitted verbatim into the generated Rust source file before component definitions, allowing full Rust syntax including attributes (`#[inline]`), lifetimes (`&'a str`), macros, and pattern matching.

---

## 4. CLI Crate Management

Ferrix CLI provides commands to manage crates and verify their Android target compatibility:

### Adding Crates
```bash
# Add a crate with features
ferrix add crate uuid --features v4

# Add a crate without default features
ferrix add crate reqwest --no-default-features --features json,rustls-tls
```
When running `ferrix add crate`:
1. Warns if the crate has known Android pitfalls (e.g., `openssl`, `native-tls`, `dirs`).
2. Runs `cargo add <crate>`.
3. Verifies Android target compilation with `cargo check --target x86_64-linux-android`.
4. Initializes `deny.toml` for supply-chain vulnerability and license auditing.

### Removing Crates
```bash
ferrix remove crate uuid
```

### Listing Installed Dependencies
```bash
ferrix crates list
```

---

## 5. Panic Safety and Architecture A
1. **No Boundary Panics**: Any functions called from `.fsx` execute on the Rust runtime thread. If an error occurs, the runtime error boundary traps unwinds (`catch_unwind`) and reports errors to the native Android logger or DevTools error overlay, preventing native crashes.
2. **Deterministic Typing**: Types exposed across Rust and `.fsx` compile statically without reflection or JNI crossing overhead.
