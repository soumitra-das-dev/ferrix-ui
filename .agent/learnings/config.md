# Learnings: Configuration as Single Source of Truth

### 2026-11-10 Part 1 Config Single Source of Truth & Host Decoupling
- Context: `ferrix.config.toml` schema v1, automated Android artifact generation, and fixed host namespace architecture.
- What happened / finding:
  1. Hyphens in application IDs caused subtle Android Gradle/AAPT failures. Strict validation with diagnostic suggestion (`Did you mean 'com.name.sub_name'?`) prevents invalid IDs upfront.
  2. Decoupling the Kotlin host activity namespace to `dev.ferrix.host.MainActivity` completely eliminates JNI symbol mangling crashes (`Java_dev_ferrix_host_...`) caused by package names with hyphens, underscores, or multiple segments.
  3. Generating Android 12+ SplashScreen theme (`Theme.App.Starting` extending `Theme.SplashScreen`) requires `androidx.core:core-splashscreen:1.0.1` and vector icon assets (`splash_icon.xml`, `ic_launcher.xml`, `ic_launcher_round.xml`) in `res/mipmap-anydpi-v26/` to ensure AAPT resource linking succeeds reliably.
  4. Non-interactive smoke testing (`ferrix dev --smoke`) confirmed deterministic compilation, AAPT resource linking, ADB streaming install, `ferrix:ready` observation, and hot reload verification on `emulator-5554` without manual intervention.
- Root cause: Dynamic package names in host classes tightly coupled JNI symbols to user application IDs; missing default adaptive mipmap drawables caused AAPT resource linking failures.
- Rule going forward:
  1. Always keep host Kotlin code in fixed namespace `dev.ferrix.host`, configuring user `id` only in Gradle `applicationId`.
  2. Always generate adaptive icons and splash vector resources whenever materializing Android assets.
  3. Validate all config changes via `FerrixConfig::parse` before writing or applying.
- Evidence: `cargo test --workspace` (100% green), `config_cli_tests` (5/5 passed), `golden_tests` (3/3 passed), `ferrix dev --smoke` on `emulator-5554` passed with zero errors.

