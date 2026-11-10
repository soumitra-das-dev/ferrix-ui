# Plan: Fix AndroidX Configuration Across Templates and CLI Scaffolding

## 1. Problem Statement
When executing `ferrix dev` or `ferrix run android` in a project scaffolded with `ferrix new`, Gradle fails at `:app:checkDebugAarMetadata`:
```
Execution failed for task ':app:checkDebugAarMetadata'.
> Configuration ':app:debugRuntimeClasspath' contains AndroidX dependencies, but the android.useAndroidX property is not enabled, which may cause runtime issues.
  Set android.useAndroidX=true in the gradle.properties file and retry.
```

The root causes:
1. `crates/ferrix-cli/src/templates.rs` (`scaffold_android_project`) materializes `build.gradle.kts`, `settings.gradle.kts`, `AndroidManifest.xml`, and `MainActivity.kt`, but never generates `android/gradle.properties`.
2. All 6 static starter templates under `templates/` (`hello-counter`, `blank`, `tabs`, `stack`, `api-client`, `kitchen-sink`) lack `android/gradle.properties`.
3. Existing scaffolded projects lack self-healing: if an existing project runs `ferrix dev` or `ferrix run android`, the CLI fails to build unless the developer manually creates `gradle.properties`.

## 2. Implementation Steps

### Step 1: Update CLI Android Scaffolding Engine (`crates/ferrix-cli/src/templates.rs`)
- In `scaffold_android_project`:
  Generate `android/gradle.properties` with:
  ```properties
  # Ferrix Android Application Properties
  org.gradle.jvmargs=-Xmx2048m -Dfile.encoding=UTF-8
  android.useAndroidX=true
  android.nonTransitiveRClass=true
  ```
- Ensure file permissions and directory structure are correctly handled.

### Step 2: Add Self-Healing to CLI Run and Build Commands (`crates/ferrix-cli/src/commands/`)
- In `commands/run.rs` (which powers both `ferrix run android` and `ferrix dev`) and `commands/build.rs`:
  Add a helper `ensure_gradle_properties(android_dir: &Path)` that:
  - If `android/gradle.properties` does not exist, creates it with `android.useAndroidX=true` and `android.nonTransitiveRClass=true`.
  - If `android/gradle.properties` exists but does not contain `android.useAndroidX`, safely appends `android.useAndroidX=true\nandroid.nonTransitiveRClass=true\n`.
  This guarantees that existing scaffolded apps immediately self-heal on next `ferrix dev` or `ferrix run android`.

### Step 3: Update Template Catalog (`templates/*/android/gradle.properties`)
- Create `gradle.properties` with `android.useAndroidX=true` and `android.nonTransitiveRClass=true` in:
  - `templates/hello-counter/android/gradle.properties`
  - `templates/blank/android/gradle.properties`
  - `templates/tabs/android/gradle.properties`
  - `templates/stack/android/gradle.properties`
  - `templates/api-client/android/gradle.properties`
  - `templates/kitchen-sink/android/gradle.properties`

### Step 4: Add Unit and Integration Tests for CLI Scaffolding
- Add unit tests in `crates/ferrix-cli/src/templates.rs` (or a dedicated integration test) verifying:
  - `scaffold_android_project` writes `android/gradle.properties` containing `android.useAndroidX=true`.
  - `ensure_gradle_properties` creates or patches existing files correctly.

### Step 5: Verification
- Run `cargo fmt --check`
- Run `cargo clippy --workspace -- -D warnings`
- Run `cargo test --workspace`
- Reinstall CLI locally: `cargo install --path crates/ferrix-cli`
- Scaffold a test project and run `gradle checkDebugAarMetadata` to verify the build passes cleanly.
- Run `./gradlew testDebugUnitTest` in `android/`.

### Step 6: Commit, Branch Parity & Learnings
- Commit to `main`
- Merge / cherry-pick to `master`
- Update `.agent/learnings/cli.md` and `.agent/learnings/milestones.md`.
