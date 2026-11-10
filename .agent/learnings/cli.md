# CLI Learnings (`crates/ferrix-cli`)

## Architecture Decisions & Invariants
The `ferrix` CLI serves as the primary developer experience entry point for Ferrix applications, mirroring modern ergonomic meta-framework tooling (`create-next-app`, `cargo`).

### 1. Embedded Zero-Network Scaffolding
- Rather than cloning templates from GitHub or fetching tarballs over the network, `ferrix new` generates project structures from embedded templates.
- Developers can scaffold projects in offline environments without network latency.
- Supported templates:
  - `hello-counter` (default): Reactive state and counter increments.
  - `blank`: Single screen starter.
  - `tabs`: TabNavigator with 3 screens.
  - `stack`: StackNavigator with master-detail screens.
  - `api-client`: useQuery data fetching with loading spinner.
  - `kitchen-sink`: Comprehensive primitive showcase with switches, inputs, and storage.

### 2. Path & Identifier Normalization
- Target names like `my-app` or `/path/to/my-app` must be cleanly separated:
  - `app_name`: User-facing name (e.g. `my-app`).
  - `slug`: Lowercase hyphenated crate identifier (e.g. `my-app`).
  - `pkg_name`: Lowercase underscore-separated Java/Kotlin identifier (e.g. `my_app`).
- Generates valid `ferrix.config.toml` that passes all schema validation invariants in `ferrix-config`.

### 3. Comprehensive Toolchain Doctor & Remediation
- `ferrix doctor` diagnoses:
  - Rust compiler & Cargo versions.
  - Android compilation targets (`aarch64-linux-android`, `x86_64-linux-android`).
  - Android SDK (`ANDROID_HOME` / `ANDROID_SDK_ROOT`).
  - Android NDK (`ndk/*` or `ANDROID_NDK_ROOT`).
  - JDK 17 (`javac -version`).
  - ADB & attached device / emulator enumeration.
- `--fix` flag automatically runs `rustup target add` to install missing compilation targets.

### 4. Integrated Quality Tooling
- `ferrix check`: Type-checks `.fsx` AST parsing and runs static linter rules.
- `ferrix fmt`: Formats `.fsx` files in place or validates via `--check`.
- `ferrix test`: Runs unit and headless component tests.
- `ferrix add <module>`: Injects dependencies, updates AndroidManifest permissions, and prints sample snippets.
