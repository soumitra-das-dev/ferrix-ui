# FSX Language Specification (`docs/fsx-spec.md`)

## 1. Introduction & Philosophy
**FSX** (`*.fsx`) is a statically typed, declarative component language designed specifically for Ferrix. It combines TypeScript/Rust-style static type safety with familiar JSX declarative syntax, compiling directly to Rust ahead of time (AOT).

There is **no runtime interpreter**, **no JavaScript virtual machine**, and **no reflection**. Every `.fsx` file compiles down to native Rust structs and reactive functions.

---

## 2. Lexical & Formal Grammar (EBNF)

### 2.1 Lexical Grammar
```ebnf
Identifier      ::= [a-zA-Z_][a-zA-Z0-9_]*
TypeIdentifier  ::= [A-Z][a-zA-Z0-9_]*
IntegerLiteral  ::= [0-9]+ ("i32" | "i64" | "u32" | "u64")?
FloatLiteral    ::= [0-9]+ "." [0-9]+ ("f32" | "f64")?
StringLiteral   ::= '"' [^"\\]* '"'
BooleanLiteral  ::= "true" | "false"
```

### 2.2 Top-Level Declarations
```ebnf
SourceFile      ::= ImportDecl* (TypeDecl | ComponentDecl | FunctionDecl)*

ImportDecl      ::= "import" "{" (Identifier ("," Identifier)*)? "}" "from" StringLiteral ";"
TypeDecl        ::= "type" TypeIdentifier "=" RecordType ";"
RecordType      ::= "{" (Identifier ":" TypeRef ("," Identifier ":" TypeRef)*)? ","? "}"

TypeRef         ::= PrimitiveType | TypeIdentifier | ListType | OptionType
PrimitiveType   ::= "i32" | "i64" | "u32" | "u64" | "f32" | "f64" | "bool" | "String" | "Element"
ListType        ::= "Vec" "<" TypeRef ">" | TypeRef "[]"
OptionType      ::= "Option" "<" TypeRef ">" | TypeRef "?"

ComponentDecl   ::= "export"? "function" TypeIdentifier "(" PropsParam? ")" ":" "Element" Block
PropsParam      ::= Identifier ":" TypeRef
```

### 2.3 Statements & Expressions
```ebnf
Block           ::= "{" Statement* ReturnStmt? "}"
Statement       ::= HookStmt | LetStmt | ExprStmt
HookStmt        ::= "const" Identifier "=" HookCall ";"
LetStmt         ::= ("const" | "let") Identifier (":" TypeRef)? "=" Expr ";"
ReturnStmt      ::= "return" (FsxElement | Expr) ";"

Expr            ::= PrimaryExpr (BinaryOp PrimaryExpr)*
PrimaryExpr     ::= Identifier | Literal | MemberExpr | CallExpr | ArrowFn | "(" Expr ")"
ArrowFn         ::= "(" (Identifier (":" TypeRef)?)? ")" "=>" (Expr | Block)
```

### 2.4 FSX Elements & JSX Syntax
```ebnf
FsxElement      ::= OpeningTag FsxChild* ClosingTag | SelfClosingTag
OpeningTag      ::= "<" ElementName FsxAttr* ">"
ClosingTag      ::= "</" ElementName ">"
SelfClosingTag  ::= "<" ElementName FsxAttr* "/>"
ElementName     ::= TypeIdentifier

FsxAttr         ::= AttrName "=" (AttrLiteral | "{" Expr "}")
AttrName        ::= Identifier

FsxChild        ::= FsxElement | "{" Expr "}" | TextContent
TextContent     ::= [^{}<>]+
```

---

## 3. Semantics & Hooks Model

### 3.1 Hook Ordering & Invariants
Hooks must follow strict structural rules:
1. **Unconditional Execution**: Hooks must be called at the top level of a component block. Never call hooks inside loops, conditions, or nested functions.
2. **Stable Order**: The compiler assigns an incrementing compile-time hook slot index `0, 1, 2, ...` to each hook call within a component.

### 3.2 Supported Hooks
1. **`useState<T>(initial: T): State<T>`**:
   Creates a reactive state variable.
   - `count.value`: Dereferences current value (subscribes current reactive scope).
   - `count.set(newValue)`: Updates value and marks subscribers dirty.
2. **`useSignal<T>(initial: T): Signal<T>`**:
   Low-level reactive signal.
3. **`useMemo<T>(compute: () => T, deps: [..]): Memo<T>`**:
   Memoized derived computation that recalculates only when dependencies change.
4. **`useEffect(effect: () => Option<() => ()>, deps: [..])`**:
   Registers a side effect executed after DOM patch commit, with optional cleanup closure.
5. **`useRef<T>(initial: T): Ref<T>`**:
   Persistent mutable reference that does not trigger re-render on change.
6. **`useContext<T>(context: Context<T>): T`**:
   Subscribes to an inherited ancestor context value.
7. **`useResource<T, E>(fetcher: () => Future<Result<T, E>>, deps: [..]): Resource<T, E>`**:
   Manages async fetching states: `Loading`, `Ready(T)`, `Error(E)`.

### 3.3 Control Flow & Keyed Lists
- **Conditionals**: Handled via ternary expressions `{cond ? <TrueView/> : <FalseView/>}` or early returns.
- **Keyed Lists**: Array mapping expressions require a `key` prop on the top-level returned element for efficient reconciliation:
  ```fsx
  {todos.value.map((todo) => (
      <TodoItem key={todo.id} todo={todo} />
  ))}
  ```

---

## 4. Rust Codegen Mapping

The compiler maps `.fsx` code to native Rust code in the target app crate.

### 4.1 Golden Example Reference
Source `src/component.fsx`:
```fsx
import { Column, Text, Button } from "ferrix/ui";

type CounterProps = { initial: i32 };

export function Counter(props: CounterProps): Element {
    const count = useState(props.initial);

    return (
        <Column padding={20} spacing={12}>
            <Text size={24}>Ferrix Counter</Text>
            <Text>Count: {count.value}</Text>
            <Button onClick={() => count.set(count.value + 1)}>
                Increment
            </Button>
        </Column>
    );
}
```

### 4.2 Generated Rust Code
The compiler generates:
```rust
// AUTO-GENERATED BY FERRIX COMPILER. DO NOT EDIT.
use ferrix_core::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CounterProps {
    pub initial: i32,
}

pub fn Counter(cx: &mut ComponentContext, props: CounterProps) -> Element {
    // Slot 0: useState hook
    let count = cx.use_state(props.initial);

    Column::new()
        .padding(20.0)
        .spacing(12.0)
        .child(
            Text::new("Ferrix Counter")
                .size(24.0)
        )
        .child(
            // Fine-grained binding: subscribes only to count
            Text::new_reactive({
                let count = count.clone();
                move || format!("Count: {}", count.get())
            })
        )
        .child(
            Button::new("Increment")
                .on_click({
                    let count = count.clone();
                    move || count.set(count.get() + 1)
                })
        )
        .build(cx)
}
```

### 4.3 Diagnostics Specification
When compilation fails, the diagnostics engine produces human-readable errors following rustc formatting:
```
error[FSX0023]: hooks cannot be called inside conditional blocks
 --> src/component.fsx:6:9
  |
5 |     if (props.initial > 0) {
6 |         const count = useState(props.initial);
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ hook called conditionally
  |
  = note: hooks must be called unconditionally at the root of the component
```
All diagnostics carry `file:line:col` spans, visual ASCII underlines, error codes, and help notes.
