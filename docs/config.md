# `ferrix.config.toml` — Single Source of Truth

In Ferrix, `ferrix.config.toml` is the authoritative, declarative single source of truth for all application metadata, device permissions, visual theming (splash and icons), deep links, and build variants. Developers never edit generated Android platform manifests or XML resources by hand.

---

## 1. Schema Reference (Version 1)

```toml
schema_version = 1

[app]
name        = "My App"                  # Launcher label displayed on the home screen
slug        = "my-app"                  # Filesystem and URL-safe identifier
id          = "com.name.sub_name"       # Android applicationId (strictly validated)
version     = "1.4.2"                   # Semver; versionCode derived automatically
description = "A production mobile app"
orientation = "portrait"                # portrait | landscape | default | per-screen
edge_to_edge = true                     # Enable edge-to-edge window insets

[android]
min_sdk           = 26                  # Android 8.0+
target_sdk        = 35                  # Current Google Play Store requirement
compile_sdk       = 35                  # Compile SDK API level
permissions       = ["INTERNET", "CAMERA"] # Android uses-permission entries
features          = [{ name = "android.hardware.camera", required = false }]
allow_backup      = false               # Disable cloud backup for sensitive data
cleartext_domains = []                  # Whitelisted domains for HTTP (dev only)

[icon]
source              = "assets/icon.png" # 1024×1024 base PNG/SVG
adaptive_foreground = "assets/icon-fg.png"
adaptive_background = "#0B1F3A"         # Hex color or image path
monochrome          = "assets/icon-mono.png" # Android 13+ themed icon layer
round               = true              # Generate round icon variant

[splash]
background       = "#0B1F3A"            # Splash background color
dark_background  = "#000000"            # Night mode splash background
icon             = "assets/splash-icon.png"
branding         = "assets/branding.png"
keep_until_ready = true                 # Hold splash until ferrix:ready marker
exit_animation   = "fade"               # none | fade | slide

[deep_links]
schemes = ["myapp"]                     # Custom schemes (e.g. myapp://path)
[[deep_links.hosts]]
host        = "example.com"             # Verified domain for Android App Links
auto_verify = true                      # Enable automatic Android App Links verification
paths       = ["/product/*", { prefix = "/u/" }]

[build.variants.dev]
id_suffix   = ".dev"                    # Yields com.name.sub_name.dev
name_suffix = " (Dev)"                  # Yields My App (Dev)
icon_badge  = "DEV"
env_file    = ".env.dev"

[build.variants.prod]
env_file    = ".env.prod"

[locale]
supported = ["en", "bn", "hi"]          # Supported BCP-47 locale tags
default   = "en"

[signing]
keystore_env = "FERRIX_KEYSTORE"        # Environment variable name holding keystore path
alias_env    = "FERRIX_KEY_ALIAS"       # Environment variable name holding key alias
```

---

## 2. Validation & Security Rules

### Android Application ID
- Must contain $\ge 2$ dot-separated segments.
- Each segment must begin with an ASCII letter and contain only alphanumeric characters or underscores `_`.
- Reserved Java and Kotlin keywords (`class`, `val`, `fun`, `package`, etc.) are forbidden as segments.
- **Hyphens are invalid:** If `com.name.sub-name` is configured, Ferrix halts validation with a diagnostic suggestion: `Did you mean 'com.name.sub_name'?`.

### Decoupled Host Namespace (`dev.ferrix.host`)
Native Kotlin classes reside in `dev.ferrix.host.MainActivity`. Gradle configures `defaultConfig { applicationId = "..." }`. This completely decouples JNI export symbol names (`Java_com_ferrix_runtime_FerrixBridge_...`) from user application IDs, preventing name mangling crashes.

### Deterministic VersionCode Derivation
The integer `versionCode` for Google Play is derived deterministically from semver:
$$\text{versionCode} = \text{major} \times 1\,000\,000 + \text{minor} \times 1\,000 + \text{patch}$$
- `1.0.0` $\to 1\,000\,000$
- `1.4.2` $\to 1\,004\,002$
- `2.10.5` $\to 2\,010\,005$

---

## 3. CLI Commands

### `ferrix config check`
Validates `ferrix.config.toml` against schema version 1, reporting syntax and semantic diagnostics.

```bash
ferrix config check
```

### `ferrix config print`
Prints resolved configuration values for the default or specified build variant:

```bash
ferrix config print --variant dev
ferrix config print --json
```

### `ferrix config explain <key>`
Explains the purpose, validation rules, and default values of any configuration key:

```bash
ferrix config explain app.id
ferrix config explain splash.background
```

---

## 4. Generated Artifacts

Running `ferrix dev`, `ferrix run`, or `ferrix prebuild` automatically derives:
- `android/app/src/main/AndroidManifest.xml`
- `android/app/src/main/res/values/strings.xml`
- `android/app/src/main/res/values/themes.xml` (SplashScreen API light mode)
- `android/app/src/main/res/values-night/themes.xml` (SplashScreen API dark mode)
- `android/app/src/main/res/xml/network_security_config.xml` (Cleartext domains)
- `android/app/src/main/res/xml/locales_config.xml` (Android 13+ per-app locales)
- `android/app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml`
- `android/app/src/main/res/mipmap-anydpi-v26/ic_launcher_round.xml`
- `android/app/src/main/res/drawable/splash_icon.xml`

