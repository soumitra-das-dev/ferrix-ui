# F5 Learnings: Config, Env & ICU i18n (`ferrix-config`, `ferrix-i18n`)

## Architecture Decisions & Invariants
Under **Architecture A**, build configuration, security boundaries, and localized text delivery are evaluated before or during compilation.

### 1. Build-Time Schema Validation (`ferrix-config`)
- `ferrix.config.toml` enforces strict structural and semantic invariants before code generation starts:
  - Package names must be valid multi-segment identifiers (e.g. `com.example.app`).
  - Android `min_sdk` cannot exceed `target_sdk`.
  - Custom URI schemes must start with a lowercase letter and contain only alphanumeric characters or safe separators.
- Syntax or semantic violations fail fast at build time with descriptive errors (`ConfigError`), preventing runtime crashes.

### 2. Client-Safe Environment Layering
- Environment files are loaded with strict precedence:
  `.env.<variant>.local` > `.env.<variant>` > `.env.local` > `.env`
- Server secrets (e.g. `DATABASE_URL`, `STRIPE_SECRET_KEY`) must never be leaked into client APK binaries.
- `filter_public_env()` acts as an allowlist gate, including only variables prefixed with `FERRIX_PUBLIC_` or `EXPO_PUBLIC_` in client builds.

### 3. ICU Message Formatting & Reactive Localization (`ferrix-i18n`)
- Implements standard ICU MessageFormat parsing:
  - Simple interpolation: `"{name}"`
  - Plural categories (`zero`, `one`, `two`, `other`) with `#` count replacement
  - Selection branches: `"{gender, select, male {He} female {She} other {They}}"`
- `use_translation()` returns `(t, locale_signal, set_locale)`. Switching locales updates the fine-grained signal, automatically triggering re-render on dependent text elements.

### 4. Verification Evidence
- `ferrix-config`: 5 unit tests passing in `crates/ferrix-config/tests/config_tests.rs` (schema validation, invalid package rejection, SDK range rejection, env parsing, public prefix filtering).
- `ferrix-i18n`: 6 unit tests passing in `crates/ferrix-i18n/tests/i18n_tests.rs` (interpolation, plurals, select, fallback resolution, reactive locale switching).
- Workspace: 98 Rust tests passing across 17 crates, all Android Robolectric tests passing.
