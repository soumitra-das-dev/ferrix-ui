# Ferrix Crate Compatibility Guide for Android

When selecting crates from the Rust ecosystem to run inside a Ferrix mobile application, developers must consider Android platform boundaries, target ABIs, dynamic linker constraints, and sandboxing.

---

## 1. Verified Compatible Crates (Tier 1)

The following popular crates have been verified and tested for Android targets (`aarch64-linux-android`, `x86_64-linux-android`, `armv7-linux-androideabi`):

| Crate | Recommended Features | Purpose |
|---|---|---|
| `uuid` | `features = ["v4"]` | Cryptographically secure random UUID generation |
| `regex` | `default-features` | Regular expression parsing and matching |
| `serde` / `serde_json` | `features = ["derive"]` | JSON serialization and deserialization |
| `reqwest` | `default-features = false, features = ["rustls-tls", "json"]` | Async HTTP networking with pure-Rust TLS |
| `chrono` | `features = ["serde"]` | Date, time, and timezone calculations |
| `url` | `default-features` | URL parsing and normalization |
| `image` | `default-features = false, features = ["png", "jpeg"]` | Pure-Rust image decoding and manipulation |
| `rusqlite` | `features = ["bundled"]` | Bundled SQLite C engine compiled via NDK |

---

## 2. Known Pitfalls & Incompatible Crates

| Incompatible Crate | Reason for Failure on Android | Recommended Replacement |
|---|---|---|
| `openssl` | Requires dynamic linking against external OpenSSL `.so` libraries not bundled with modern Android SDKs. | `rustls` or `ring` |
| `native-tls` | Attempts to load platform-native TLS libraries (Schannel/Security Framework/OpenSSL) which fail to link or resolve certificates on Android. | `rustls-tls` |
| `dirs` | Uses desktop XDG paths (`~/.config`, `~/.local`) which violate Android application sandbox structure. | Ferrix Storage APIs (`use_storage`) |
| `winit` / `sdl2` | Tries to take over the native Android Activity lifecycle and window surface, conflicting with Ferrix's native Android View hierarchy. | Ferrix UI primitives (`ferrix/ui`) |

---

## 3. Automated Pitfall Detection

The Ferrix CLI automatically checks for known incompatible crates when adding dependencies:

```bash
$ ferrix add crate openssl
📦 Adding crate 'openssl' to Ferrix application...

  ⚠️ Android Compatibility Warning: Crate 'openssl' requires native C toolchain and dynamic libraries not bundled with Android. Recommended: 'rustls' or 'reqwest' with 'rustls-tls'.
```

---

## 4. Supply-Chain Security (`cargo-deny`)

To ensure dependencies meet enterprise compliance, licensing requirements, and advisory checks, Ferrix generates a default `deny.toml` configuration:

```toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"

[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-3-Clause",
    "ISC",
    "Unicode-3.0",
]

[bans]
multiple-versions = "warn"
```

Run supply-chain verification:
```bash
cargo deny check
```
