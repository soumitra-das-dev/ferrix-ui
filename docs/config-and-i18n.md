# Ferrix Config, Environment, Assets & i18n Specification (F5)

## 1. Overview & Architecture
Ferrix application configuration, environment isolation, asset metadata, and internationalization run under **Architecture A**:
- **Strict Schema Validation (`ferrix-config`)**: `ferrix.config.toml` is verified at build time. Invalid package identifiers, unsupported Android SDK limits, or malformed custom URI schemes fail the build with line-accurate diagnostics before compilation begins.
- **Client-Safe Environment Injection**: Sensitive server keys in `.env` are protected. Only variables prefixed with `FERRIX_PUBLIC_` (or `EXPO_PUBLIC_`) are bundled into client Rust binaries.
- **ICU Message Format (`ferrix-i18n`)**: High-performance internationalization supporting pluralization categories, selection branches, and runtime locale toggling integrated with `I18nManager` RTL layout.

---

## 2. Configuration Schema (`ferrix.config.toml`)

```toml
[app]
name = "My Ferrix App"
slug = "my-ferrix-app"
version = "1.0.0"
package_name = "com.example.myferrixapp"
scheme = "myferrixapp"

[android]
min_sdk = 26
target_sdk = 34
version_code = 1
permissions = [
    "android.permission.INTERNET",
    "android.permission.CAMERA"
]

[icons]
icon = "assets/icon.png"
adaptive_foreground = "assets/adaptive-foreground.png"
adaptive_background = "#0f172a"

[splash]
image = "assets/splash.png"
background_color = "#0f172a"
```

---

## 3. Environment Layering

Layer precedence (higher overrides lower):
1. `.env.<variant>.local` (e.g. `.env.production.local`)
2. `.env.<variant>` (e.g. `.env.production`)
3. `.env.local`
4. `.env`

Only variables starting with `FERRIX_PUBLIC_` are accessible via `process.env` / `env!("...")` in UI code.

---

## 4. Internationalization Engine (`ferrix-i18n`)

### 4.1 Message Format
Supports ICU plural and select syntax:
- `t("photos.count", { count: 3 })`: `"You have {count, plural, one {# photo} other {# photos}}"`
- `t("greeting", { name: "Alice" })`: `"Hello, {name}!"`

### 4.2 Reactive Hooks
- `use_translation()` returns `(t, locale_signal, set_locale)`
- Changing locale updates reactive text elements and updates layout direction if RTL changes.

