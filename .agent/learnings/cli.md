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

### 5. AndroidX Configuration & Project Self-Healing
- Android Gradle Plugin (AGP 8+) requires `android.useAndroidX=true` in `gradle.properties` when AndroidX libraries (`androidx.core`, `androidx.appcompat`, `com.google.android.material`) are included in dependencies.
- Omitting `android.useAndroidX=true` triggers `Execution failed for task ':app:checkDebugAarMetadata'`.
- Scaffolding engine (`scaffold_android_project`) guarantees generation of `android/gradle.properties` with:
  ```properties
  org.gradle.jvmargs=-Xmx2048m -Dfile.encoding=UTF-8
  android.useAndroidX=true
  android.nonTransitiveRClass=true
  ```
- All starter templates under `templates/*/android/gradle.properties` include this configuration out-of-the-box.
- Self-healing invariant: `ferrix run android`, `ferrix dev`, and `ferrix build` automatically execute `ensure_gradle_properties(android_dir)` before invoking Gradle tasks, safely patching missing or legacy project files on the fly.

### 6. Template Discovery, Validation & Scaffolding Invariants
- `ferrix new --list` / `-l`: Displays all 6 official starter templates (`hello-counter`, `blank`, `tabs`, `stack`, `api-client`, `kitchen-sink`) with detailed descriptions.
- `ferrix new <name>` (interactive mode): Prompts developers with a numbered interactive template picker when `--template` is omitted in an interactive terminal.
- Template name normalization: Resolves aliases case-insensitively (`counter` -> `hello-counter`, `tab` -> `tabs`, `api` -> `api-client`, `kitchen` -> `kitchen-sink`, `empty` -> `blank`).
- Fail-fast validation: Any unrecognized template name triggers a descriptive error listing all available templates rather than silently creating an unintended template.
- Compiler & build safety: Every template generates `[build-dependencies] ferrix-compiler = { version = "0.1" }` in `Cargo.toml` and valid `.fsx` component syntax verified by unit tests across all templates.
