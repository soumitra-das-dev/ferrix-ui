use ferrix_config::FerrixConfig;
use tempfile::tempdir;

const SAMPLE_CONFIG: &str = r##"
schema_version = 1

[app]
name = "My App"
slug = "my-app"
id = "com.name.sub_name"
version = "1.4.2"
description = "A production mobile app"
orientation = "portrait"
edge_to_edge = true

[android]
min_sdk = 26
target_sdk = 35
compile_sdk = 35
permissions = ["INTERNET", "CAMERA"]
features = [{ name = "android.hardware.camera", required = false }]
allow_backup = false
cleartext_domains = ["api.dev.example.com"]

[icon]
source = "assets/icon.png"
adaptive_foreground = "assets/icon-fg.png"
adaptive_background = "#0B1F3A"
monochrome = "assets/icon-mono.png"
round = true

[splash]
background = "#0B1F3A"
dark_background = "#000000"
icon = "assets/splash-icon.png"
branding = "assets/branding.png"
keep_until_ready = true
exit_animation = "fade"

[deep_links]
schemes = ["myapp"]

[[deep_links.hosts]]
host = "example.com"
auto_verify = true
paths = ["/product/*", { prefix = "/u/" }]

[build.variants.dev]
id_suffix = ".dev"
name_suffix = " (Dev)"
icon_badge = "DEV"
env_file = ".env.dev"

[build.variants.prod]
env_file = ".env.prod"

[locale]
supported = ["en", "bn", "hi"]
default = "en"

[signing]
keystore_env = "FERRIX_KEYSTORE"
alias_env = "FERRIX_KEY_ALIAS"
"##;

#[test]
fn test_golden_manifest_generation() {
    let config = FerrixConfig::parse(SAMPLE_CONFIG).expect("Failed to parse config");
    let manifest = config.generate_android_manifest(None);

    // Verify fixed host namespace decoupling
    assert!(manifest.contains("android:name=\"dev.ferrix.host.MainActivity\""));

    // Verify permissions
    assert!(manifest.contains("<uses-permission android:name=\"android.permission.INTERNET\" />"));
    assert!(manifest.contains("<uses-permission android:name=\"android.permission.CAMERA\" />"));

    // Verify features
    assert!(manifest.contains(
        "<uses-feature android:name=\"android.hardware.camera\" android:required=\"false\" />"
    ));

    // Verify application attributes
    assert!(manifest.contains("android:allowBackup=\"false\""));
    assert!(manifest.contains("android:roundIcon=\"@mipmap/ic_launcher_round\""));
    assert!(manifest.contains("android:theme=\"@style/Theme.App.Starting\""));
    assert!(manifest.contains("android:networkSecurityConfig=\"@xml/network_security_config\""));
    assert!(manifest.contains("android:localeConfig=\"@xml/locales_config\""));
    assert!(manifest.contains("android:screenOrientation=\"portrait\""));

    // Verify custom scheme deep links
    assert!(manifest.contains("<data android:scheme=\"myapp\" />"));

    // Verify App Links (auto_verify=true)
    assert!(manifest.contains("<intent-filter android:autoVerify=\"true\">"));
    assert!(manifest.contains("<data android:host=\"example.com\" />"));
    assert!(manifest.contains("<data android:pathPattern=\"/product/*\" />"));
    assert!(manifest.contains("<data android:pathPrefix=\"/u/\" />"));
}

#[test]
fn test_golden_resources_generation() {
    let config = FerrixConfig::parse(SAMPLE_CONFIG).expect("Failed to parse config");

    // Strings
    let strings_xml = config.generate_strings_xml(None);
    assert!(strings_xml.contains("<string name=\"app_name\">My App</string>"));

    let dev_strings_xml = config.generate_strings_xml(Some("dev"));
    assert!(dev_strings_xml.contains("<string name=\"app_name\">My App (Dev)</string>"));

    // Themes (Splash API)
    let light_theme = config.generate_themes_xml(false);
    assert!(light_theme.contains("<item name=\"windowSplashScreenBackground\">#0B1F3A</item>"));

    let dark_theme = config.generate_themes_xml(true);
    assert!(dark_theme.contains("<item name=\"windowSplashScreenBackground\">#000000</item>"));

    // Network security
    let net_cfg = config
        .generate_network_security_config()
        .expect("Should have net cfg");
    assert!(net_cfg.contains("<domain includeSubdomains=\"true\">api.dev.example.com</domain>"));

    // Locales
    let locales_xml = config.generate_locales_config();
    assert!(locales_xml.contains("<locale android:name=\"en\" />"));
    assert!(locales_xml.contains("<locale android:name=\"bn\" />"));
    assert!(locales_xml.contains("<locale android:name=\"hi\" />"));

    // Asset links
    let assetlinks = config.generate_assetlinks_json(&["14:6D:E9:FD:..."]);
    assert!(assetlinks.contains("\"package_name\": \"com.name.sub_name\""));
    assert!(assetlinks.contains("\"14:6D:E9:FD:...\""));

    // Gradle flavors
    let gradle_flavors = config.generate_gradle_flavors();
    assert!(gradle_flavors.contains("create(\"dev\")"));
    assert!(gradle_flavors.contains("applicationIdSuffix = \".dev\""));

    // Rust config module
    let rust_mod = config.generate_rust_config_mod(Some("dev"));
    assert!(rust_mod.contains("pub const ID: &str = \"com.name.sub_name.dev\";"));
    assert!(rust_mod.contains("pub const NAME: &str = \"My App (Dev)\";"));
    assert!(rust_mod.contains("pub const VERSION: &str = \"1.4.2\";"));
}

#[test]
fn test_write_all_artifacts() {
    let config = FerrixConfig::parse(SAMPLE_CONFIG).expect("Failed to parse config");
    let temp = tempdir().expect("Failed to create tempdir");
    let main_dir = temp.path().join("android/app/src/main");

    config
        .write_all_artifacts(&main_dir, None)
        .expect("write_all_artifacts failed");

    assert!(main_dir.join("AndroidManifest.xml").exists());
    assert!(main_dir.join("res/values/strings.xml").exists());
    assert!(main_dir.join("res/values/themes.xml").exists());
    assert!(main_dir.join("res/values-night/themes.xml").exists());
    assert!(main_dir
        .join("res/xml/network_security_config.xml")
        .exists());
    assert!(main_dir.join("res/xml/locales_config.xml").exists());
    assert!(main_dir.join("res/drawable/splash_icon.xml").exists());
    assert!(main_dir
        .join("res/mipmap-anydpi-v26/ic_launcher.xml")
        .exists());
    assert!(main_dir
        .join("res/mipmap-anydpi-v26/ic_launcher_round.xml")
        .exists());
    assert!(main_dir.join("res/values/colors.xml").exists());
}
