# Deep Links & Android App Links in Ferrix

Ferrix provides declarative deep link and universal app link routing managed entirely from `ferrix.config.toml`. Intent filters are generated with strict isolation (one filter per scheme or verified host) to prevent unintended cross-product combinations.

---

## 1. Declaring Deep Links

In `ferrix.config.toml`:

```toml
[deep_links]
schemes = ["myapp"]

[[deep_links.hosts]]
host = "example.com"
auto_verify = true
paths = ["/product/*", { prefix = "/u/" }]
```

### Path Matching Types
- **Exact Path**: `"/settings"` $\to$ `android:path="/settings"`
- **Wildcard Pattern**: `"/product/*"` $\to$ `android:pathPattern="/product/*"`
- **Prefix Path**: `{ prefix = "/u/" }` $\to$ `android:pathPrefix="/u/"`

---

## 2. Generated Intent Filters

Ferrix compiles deep link configurations into independent `<intent-filter>` blocks in `AndroidManifest.xml`:

```xml
<!-- Custom scheme intent filter -->
<intent-filter>
    <action android:name="android.intent.action.VIEW" />
    <category android:name="android.intent.category.DEFAULT" />
    <category android:name="android.intent.category.BROWSABLE" />
    <data android:scheme="myapp" />
</intent-filter>

<!-- Universal App Link intent filter with autoVerify -->
<intent-filter android:autoVerify="true">
    <action android:name="android.intent.action.VIEW" />
    <category android:name="android.intent.category.DEFAULT" />
    <category android:name="android.intent.category.BROWSABLE" />
    <data android:scheme="https" />
    <data android:scheme="http" />
    <data android:host="example.com" />
    <data android:pathPattern="/product/*" />
    <data android:pathPrefix="/u/" />
</intent-filter>
```

---

## 3. Toolchain & CLI Commands

### Generating Digital Asset Links
Android requires hosting `/.well-known/assetlinks.json` on the target domain for App Link verification. Generate this file with:

```bash
ferrix deeplinks assetlinks --fingerprint "14:6D:E9:FD:..."
```

### Testing Deep Links on Connected Devices
Dispatch synthetic view intents directly to the connected emulator or device via ADB:

```bash
ferrix deeplinks test myapp://home
ferrix deeplinks test https://example.com/product/123
```

### Verifying Domain Links
Check verification state on device using `pm get-app-links`:

```bash
ferrix deeplinks verify
```

