# Plan: Robust Template Selection, Scaffolding, and Validation

## 1. Problem Statement
Users attempting to create Ferrix projects with different templates experienced several points of friction:
1. `ferrix new <name>` silently defaulted to `hello-counter` without providing an interactive prompt or discovering other templates.
2. Unrecognized `--template` values (e.g. `foo`, `counter`, `Stack`) silently fell back to `hello-counter` instead of raising an error or resolving aliases.
3. Running `ferrix new --template` or `ferrix new` without arguments failed with standard Clap usage errors rather than an interactive guide or template listing.
4. Several template definitions (`blank`, `kitchen-sink`, `api-client`) contained syntax incompatible with the FSX compiler (`{() => {}}` empty block and `&&` logical operator), and `api-client` was missing `ferrix-network` in `Cargo.toml`.
5. Scaffolded `Cargo.toml` files omitted `[build-dependencies] ferrix-compiler`, causing `build.rs` to fail during compilation.

## 2. Implementation Steps

### Step 1: Update CLI Arguments & Interactive Selection (`crates/ferrix-cli/src/args.rs`)
- Make `name` optional in `NewArgs` (`pub name: Option<String>`).
- Add `--list` / `-l` flag to list all available templates.
- Change `template` in `NewArgs` to `Option<String>` so interactive prompt can trigger when omitted.

### Step 2: Implement Template Catalog & Normalization (`crates/ferrix-cli/src/templates.rs`)
- Define `pub struct TemplateInfo { pub id: &'static str, pub name: &'static str, pub description: &'static str }`.
- Define the official catalog of 6 templates:
  - `hello-counter`: Interactive counter with reactive state hooks
  - `blank`: Minimal clean starter screen
  - `tabs`: TabNavigator with 3 bottom-tab screens
  - `stack`: StackNavigator with master-detail navigation
  - `api-client`: Asynchronous data fetching with activity indicator
  - `kitchen-sink`: Comprehensive showcase with switches, inputs, and buttons
- Implement alias mapping:
  - `"counter"` -> `"hello-counter"`
  - `"tab"` -> `"tabs"`
  - `"stacks"` -> `"stack"`
  - `"api"`, `"apiclient"` -> `"api-client"`
  - `"kitchen"`, `"kitchensink"` -> `"kitchen-sink"`
  - `"empty"` -> `"blank"`
- Reject invalid template names with helpful error listing all templates.
- Implement interactive prompt via stdin when TTY is attached and template/name are omitted.

### Step 3: Fix Template Content & Dependencies
- In `crates/ferrix-cli/src/templates.rs`:
  - Fix `blank` component: `<Button onClick={() => null}>Get Started</Button>`
  - Fix `kitchen-sink` component: `<Button onClick={() => enabled.set(!enabled.value)}>Toggle Feature</Button>`
  - Fix `api-client` component: remove `&&`, use clean `<ActivityIndicator color={0xFF3B82F6} />`
  - In `Cargo.toml` generation:
    - Add `ferrix-network = { version = "0.1" }` for `api-client`.
    - Add `[build-dependencies] ferrix-compiler = { version = "0.1" }` for all templates.
- Update matching files in `templates/` directory:
  - `templates/blank/src/component.fsx`
  - `templates/kitchen-sink/src/component.fsx`
  - `templates/api-client/src/component.fsx`
  - `templates/api-client/Cargo.toml`
  - All `templates/*/Cargo.toml` (`[build-dependencies]`)

### Step 4: Unit & Integration Tests
- Test alias resolution and template validation in `crates/ferrix-cli/src/templates.rs`.
- Test scaffolding of all 6 templates and verify their generated `.fsx` files compile ahead-of-time using `ferrix_compiler::compile_file`.

### Step 5: Verification & Sync
- `cargo fmt --check`
- `cargo clippy --workspace -- -D warnings`
- `cargo test --workspace`
- `./gradlew testDebugUnitTest`
- Reinstall CLI: `cargo install --path crates/ferrix-cli`
- Test `ferrix new --list`, `ferrix new my-app --template tabs`, `ferrix new my-app --template stack`, etc.
- Sync commits across `main` and `master`.
