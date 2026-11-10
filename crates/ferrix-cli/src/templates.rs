//! Project template generators and scaffolding engine.

use std::fs;
use std::path::Path;

use crate::args::NewArgs;

/// Scaffolds a complete Ferrix application into `target_dir`.
pub fn scaffold_project(target_dir: &Path, args: &NewArgs) -> std::io::Result<()> {
    fs::create_dir_all(target_dir)?;

    let raw_name = Path::new(&args.name)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| args.name.clone());
    let app_name = &raw_name;
    let slug = app_name
        .to_lowercase()
        .replace([' ', '_', '/'], "-")
        .trim_matches('-')
        .to_string();
    let pkg_name = slug.replace('-', "_");

    // 1. Write ferrix.config.toml
    let config_content = format!(
        r#"# Ferrix Application Configuration
name = "{app_name}"
slug = "{slug}"
version = "0.1.0"

[android]
package = "com.example.{pkg_name}"
version_code = 1
min_sdk = 26
target_sdk = 34
compile_sdk = 34
scheme = "{slug}"

[build]
output_dir = "dist"
"#
    );
    fs::write(target_dir.join("ferrix.config.toml"), config_content)?;

    // 2. Write environment files
    fs::write(
        target_dir.join(".env"),
        "FERRIX_PUBLIC_APP_ENV=development\nFERRIX_PUBLIC_API_URL=https://api.example.com\n",
    )?;
    fs::write(
        target_dir.join(".env.local"),
        "# Local environment overrides (not checked into git)\n",
    )?;

    // 3. Write .gitignore
    fs::write(
        target_dir.join(".gitignore"),
        r#"target/
dist/
*.so
.env.local
.env.*.local

# Android & Gradle
.gradle/
build/
android/.gradle/
android/app/build/
android/local.properties
"#,
    )?;

    // 4. Write Cargo.toml
    let mut cargo_deps = String::from(
        r#"[dependencies]
ferrix-core = { version = "0.1" }
ferrix-reactivity = { version = "0.1" }
ferrix-renderer-protocol = { version = "0.1" }
"#,
    );

    if args.router || args.template == "tabs" || args.template == "stack" {
        cargo_deps.push_str("ferrix-router = { version = \"0.1\" }\n");
    }
    if args.navigation || args.template == "tabs" || args.template == "stack" {
        cargo_deps.push_str("ferrix-navigation = { version = \"0.1\" }\n");
    }
    if args.storage || args.template == "kitchen-sink" {
        cargo_deps.push_str("ferrix-storage = { version = \"0.1\" }\n");
    }
    if args.sqlite || args.template == "kitchen-sink" {
        cargo_deps.push_str("ferrix-sqlite = { version = \"0.1\" }\n");
    }
    if args.i18n || args.template == "kitchen-sink" {
        cargo_deps.push_str("ferrix-i18n = { version = \"0.1\" }\n");
    }

    let cargo_toml = format!(
        r#"[package]
name = "{slug}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

{cargo_deps}
"#
    );
    fs::write(target_dir.join("Cargo.toml"), cargo_toml)?;

    // 5. Write UI source files in src/
    let src_dir = target_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    let component_content = match args.template.as_str() {
        "blank" => {
            r#"import { Column, Text, Button } from "ferrix/ui";

export function App(): Element {
  return (
    <Column padding={24} spacing={16}>
      <Text size={24}>Welcome to Ferrix</Text>
      <Text>Your new native Android app is running on Rust.</Text>
      <Button onClick={() => {}}>Get Started</Button>
    </Column>
  );
}
"#
        }
        "tabs" => {
            r#"import { TabNavigator, Screen, Column, Text } from "ferrix/ui";

export function App(): Element {
  return (
    <TabNavigator>
      <Screen name="Home">
        <Column padding={20}><Text size={20}>Home Screen</Text></Column>
      </Screen>
      <Screen name="Explore">
        <Column padding={20}><Text size={20}>Explore Screen</Text></Column>
      </Screen>
      <Screen name="Profile">
        <Column padding={20}><Text size={20}>Profile Screen</Text></Column>
      </Screen>
    </TabNavigator>
  );
}
"#
        }
        "stack" => {
            r#"import { StackNavigator, Screen, Column, Text, Button } from "ferrix/ui";
import { useRouter } from "ferrix/router";

export function App(): Element {
  const router = useRouter();
  return (
    <StackNavigator>
      <Screen name="Overview">
        <Column padding={20} spacing={12}>
          <Text size={22}>Master Overview</Text>
          <Button onClick={() => router.push("/details")}>View Details</Button>
        </Column>
      </Screen>
      <Screen name="Details">
        <Column padding={20} spacing={12}>
          <Text size={22}>Detail View</Text>
          <Button onClick={() => router.back()}>Go Back</Button>
        </Column>
      </Screen>
    </StackNavigator>
  );
}
"#
        }
        "api-client" => {
            r#"import { Column, Text, Button, ActivityIndicator } from "ferrix/ui";
import { useQuery } from "ferrix/network";

export function App(): Element {
  const query = useQuery("/api/data");

  return (
    <Column padding={20} spacing={12}>
      <Text size={22}>API Client Demo</Text>
      {query.loading && <ActivityIndicator />}
      {query.data && <Text>Result: {query.data}</Text>}
      <Button onClick={() => query.refetch()}>Reload</Button>
    </Column>
  );
}
"#
        }
        "kitchen-sink" => {
            r#"import { Column, Row, Text, Button, Switch, TextInput } from "ferrix/ui";
import { useState } from "ferrix/reactivity";

export function App(): Element {
  const enabled = useState(true);
  const text = useState("");

  return (
    <Column padding={20} spacing={12}>
      <Text size={24}>Ferrix Kitchen Sink</Text>
      <Row spacing={8}>
        <Text>Feature Enabled:</Text>
        <Switch value={enabled.value} onValueChange={(val) => enabled.set(val)} />
      </Row>
      <TextInput placeholder="Enter text here..." value={text.value} onChangeText={(t) => text.set(t)} />
      <Button onClick={() => {}}>Action Button</Button>
    </Column>
  );
}
"#
        }
        _ => {
            // "hello-counter" default
            r#"import { Column, Text, Button } from "ferrix/ui";
import { useState } from "ferrix/reactivity";

export function App(): Element {
  const count = useState(0);

  return (
    <Column padding={20} spacing={12}>
      <Text size={24}>Ferrix Counter</Text>
      <Text>Count: {count.value}</Text>
      <Button onClick={() => count.set(count.value + 1)}>
        Increment
      </Button>
    </Column>
  );
}
"#
        }
    };

    fs::write(src_dir.join("component.fsx"), component_content)?;

    // 6. Write src/lib.rs
    let lib_rs = format!(
        r#"//! {app_name} - Built with Ferrix UI Framework
pub mod component {{
    // Ahead-of-time compiled from src/component.fsx
    include!(concat!(env!("OUT_DIR"), "/component_generated.rs"));
}}
"#
    );
    fs::write(src_dir.join("lib.rs"), lib_rs)?;

    // 7. Write build.rs for AOT compilation
    let build_rs = r#"use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/component.fsx");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("component_generated.rs");

    if let Err(err) = ferrix_compiler::compile_file("src/component.fsx", &dest_path) {
        panic!("Failed to compile .fsx UI file: {}", err);
    }
}
"#;
    fs::write(target_dir.join("build.rs"), build_rs)?;

    // 8. Write README.md
    let readme = format!(
        r#"# {app_name}

An ahead-of-time compiled native Android application built with [Ferrix](https://github.com/soumitra-das-dev/ferrix-ui).

## Quick Start

### 1. Check prerequisites
```bash
ferrix doctor
```

### 2. Start development loop with file watching
```bash
ferrix dev
```

### 3. Run on connected Android device or emulator
```bash
ferrix run android
```

### 4. Build production release APK/AAB
```bash
ferrix build --release --apk
ferrix build --release --aab
```
"#
    );
    fs::write(target_dir.join("README.md"), readme)?;

    // 9. Generate native Android Gradle project
    scaffold_android_project(target_dir, &slug, &pkg_name)?;

    Ok(())
}

/// Ensures that `android/gradle.properties` exists and specifies `android.useAndroidX=true`.
/// If the file does not exist, it is created. If it exists but lacks `android.useAndroidX`,
/// the setting is appended.
pub fn ensure_gradle_properties(android_dir: &Path) -> std::io::Result<()> {
    let props_path = android_dir.join("gradle.properties");
    if !props_path.exists() {
        fs::write(
            &props_path,
            r#"# Ferrix Android Application Properties
org.gradle.jvmargs=-Xmx2048m -Dfile.encoding=UTF-8
android.useAndroidX=true
android.nonTransitiveRClass=true
"#,
        )?;
    } else {
        let content = fs::read_to_string(&props_path)?;
        if !content.contains("android.useAndroidX") {
            let mut updated = content;
            if !updated.ends_with('\n') {
                updated.push('\n');
            }
            updated.push_str("android.useAndroidX=true\nandroid.nonTransitiveRClass=true\n");
            fs::write(&props_path, updated)?;
        }
    }
    Ok(())
}

/// Generates the standalone native Android Studio project inside `android/`.
pub fn scaffold_android_project(
    project_dir: &Path,
    slug: &str,
    pkg_name: &str,
) -> std::io::Result<()> {
    let android_dir = project_dir.join("android");
    let app_dir = android_dir.join("app");
    let main_dir = app_dir.join("src").join("main");
    let java_pkg_dir = main_dir
        .join("java")
        .join("com")
        .join("example")
        .join(pkg_name);

    fs::create_dir_all(&java_pkg_dir)?;
    fs::create_dir_all(android_dir.join("gradle").join("wrapper"))?;

    // android/gradle.properties
    ensure_gradle_properties(&android_dir)?;

    // android/build.gradle.kts
    fs::write(
        android_dir.join("build.gradle.kts"),
        r#"plugins {
    id("com.android.application") version "8.2.2" apply false
    id("org.jetbrains.kotlin.android") version "1.9.22" apply false
}
"#,
    )?;

    // android/settings.gradle.kts
    fs::write(
        android_dir.join("settings.gradle.kts"),
        r#"pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}
rootProject.name = "FerrixApp"
include(":app")
"#,
    )?;

    // android/gradle/wrapper/gradle-wrapper.properties
    fs::write(
        android_dir
            .join("gradle")
            .join("wrapper")
            .join("gradle-wrapper.properties"),
        r#"distributionBase=GRADLE_USER_HOME
distributionPath=wrapper/dists
distributionUrl=https\://services.gradle.org/distributions/gradle-8.4-bin.zip
zipStoreBase=GRADLE_USER_HOME
zipStorePath=wrapper/dists
"#,
    )?;

    // android/app/build.gradle.kts
    let app_build_gradle = format!(
        r#"plugins {{
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}}

android {{
    namespace = "com.example.{pkg_name}"
    compileSdk = 34

    defaultConfig {{
        applicationId = "com.example.{pkg_name}"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "0.1.0"

        ndk {{
            abiFilters.addAll(listOf("arm64-v8a", "x86_64"))
        }}
    }}

    buildTypes {{
        release {{
            isMinifyEnabled = false
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }}
    }}

    sourceSets {{
        getByName("main") {{
            jniLibs.srcDirs("src/main/jniLibs")
        }}
    }}

    compileOptions {{
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }}

    kotlinOptions {{
        jvmTarget = "17"
    }}
}}

dependencies {{
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("com.google.android.material:material:1.11.0")
}}
"#
    );
    fs::write(app_dir.join("build.gradle.kts"), app_build_gradle)?;

    // android/app/src/main/AndroidManifest.xml
    let manifest_xml = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
    <uses-permission android:name="android.permission.INTERNET" />

    <application
        android:allowBackup="true"
        android:label="{slug}"
        android:supportsRtl="true"
        android:theme="@android:style/Theme.Material.Light.NoActionBar">
        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:configChanges="orientation|screenSize|screenLayout|keyboardHidden">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
"#
    );
    fs::write(main_dir.join("AndroidManifest.xml"), manifest_xml)?;

    // MainActivity.kt
    let main_activity = format!(
        r#"package com.example.{pkg_name}

import android.app.Activity
import android.os.Bundle
import android.widget.FrameLayout

class MainActivity : Activity() {{
    companion object {{
        init {{
            System.loadLibrary("{slug}")
        }}
    }}

    override fun onCreate(savedInstanceState: Bundle?) {{
        super.onCreate(savedInstanceState)
        val rootLayout = FrameLayout(this)
        setContentView(rootLayout)
    }}
}}
"#
    );
    fs::write(java_pkg_dir.join("MainActivity.kt"), main_activity)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_gradle_properties_creates_file_if_missing() {
        let tmp_dir = std::env::temp_dir().join("ferrix_test_ensure_gradle_props_new");
        let _ = fs::remove_dir_all(&tmp_dir);
        fs::create_dir_all(&tmp_dir).unwrap();

        ensure_gradle_properties(&tmp_dir).unwrap();
        let props = fs::read_to_string(tmp_dir.join("gradle.properties")).unwrap();
        assert!(props.contains("android.useAndroidX=true"));
        assert!(props.contains("android.nonTransitiveRClass=true"));

        let _ = fs::remove_dir_all(&tmp_dir);
    }

    #[test]
    fn test_ensure_gradle_properties_appends_if_missing_androidx() {
        let tmp_dir = std::env::temp_dir().join("ferrix_test_ensure_gradle_props_append");
        let _ = fs::remove_dir_all(&tmp_dir);
        fs::create_dir_all(&tmp_dir).unwrap();

        let props_file = tmp_dir.join("gradle.properties");
        fs::write(&props_file, "org.gradle.jvmargs=-Xmx1024m\n").unwrap();

        ensure_gradle_properties(&tmp_dir).unwrap();
        let props = fs::read_to_string(&props_file).unwrap();
        assert!(props.contains("org.gradle.jvmargs=-Xmx1024m"));
        assert!(props.contains("android.useAndroidX=true"));
        assert!(props.contains("android.nonTransitiveRClass=true"));

        let _ = fs::remove_dir_all(&tmp_dir);
    }

    #[test]
    fn test_scaffold_android_project_generates_gradle_properties() {
        let tmp_dir = std::env::temp_dir().join("ferrix_test_scaffold_android");
        let _ = fs::remove_dir_all(&tmp_dir);
        fs::create_dir_all(&tmp_dir).unwrap();

        scaffold_android_project(&tmp_dir, "my-app", "my_app").unwrap();
        let props_file = tmp_dir.join("android").join("gradle.properties");
        assert!(props_file.exists());
        let props = fs::read_to_string(&props_file).unwrap();
        assert!(props.contains("android.useAndroidX=true"));
        assert!(props.contains("android.nonTransitiveRClass=true"));

        let _ = fs::remove_dir_all(&tmp_dir);
    }
}
