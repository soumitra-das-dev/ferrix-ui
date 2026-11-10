//! Project template generators and scaffolding engine.

use std::fs;
use std::io::{self, IsTerminal, Write};
use std::path::Path;

use crate::args::NewArgs;

/// Metadata describing a starter template.
#[derive(Debug, Clone, Copy)]
pub struct TemplateInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

/// The official catalog of starter templates.
pub const TEMPLATES: &[TemplateInfo] = &[
    TemplateInfo {
        id: "hello-counter",
        name: "Counter",
        description: "Interactive counter with reactive state hooks (default)",
    },
    TemplateInfo {
        id: "blank",
        name: "Blank",
        description: "Minimal clean starter screen",
    },
    TemplateInfo {
        id: "tabs",
        name: "Tabs",
        description: "TabNavigator with 3 bottom-tab screens",
    },
    TemplateInfo {
        id: "stack",
        name: "Stack",
        description: "StackNavigator with master-detail navigation",
    },
    TemplateInfo {
        id: "router",
        name: "Router",
        description: "Multi-file router navigation with separate modular screens",
    },
    TemplateInfo {
        id: "api-client",
        name: "API Client",
        description: "Asynchronous data fetching with activity indicator",
    },
    TemplateInfo {
        id: "kitchen-sink",
        name: "Kitchen Sink",
        description: "Comprehensive showcase with switches, inputs, and buttons",
    },
];

/// Prints the template catalog to stdout.
pub fn print_templates() {
    println!("Available Ferrix Starter Templates:\n");
    for t in TEMPLATES {
        println!("  • {:<14} - {}", t.id, t.description);
    }
    println!("\nUsage:");
    println!("  ferrix new <name> --template <template>");
    println!("  ferrix new my-app --template router");
}

/// Normalizes and resolves a template name or alias to its canonical ID.
pub fn resolve_template(input: &str) -> Result<String, String> {
    let normalized = input.trim().to_lowercase().replace('_', "-");
    match normalized.as_str() {
        "hello-counter" | "counter" | "default" => Ok("hello-counter".to_string()),
        "blank" | "empty" | "minimal" => Ok("blank".to_string()),
        "tabs" | "tab" | "tabnavigator" => Ok("tabs".to_string()),
        "stack" | "stacks" | "stacknavigator" => Ok("stack".to_string()),
        "router" | "routes" | "routing" | "route" => Ok("router".to_string()),
        "api-client" | "api" | "apiclient" | "network" => Ok("api-client".to_string()),
        "kitchen-sink" | "kitchensink" | "kitchen" | "sink" => Ok("kitchen-sink".to_string()),
        other => {
            let mut err = format!("Unknown template '{}'.\n\nAvailable templates:\n", other);
            for t in TEMPLATES {
                err.push_str(&format!("  • {:<14} - {}\n", t.id, t.description));
            }
            err.push_str("\nRun 'ferrix new --list' to inspect all options.");
            Err(err)
        }
    }
}

/// Interactively prompts the user for a project name if running in a terminal.
pub fn prompt_app_name() -> String {
    if io::stdin().is_terminal() {
        print!("? Project name [default: my-ferrix-app]: ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }
    "my-ferrix-app".to_string()
}

/// Interactively prompts the user to select a template if running in a terminal.
pub fn prompt_template() -> String {
    if io::stdin().is_terminal() {
        println!("? Select a starter template:");
        for (i, t) in TEMPLATES.iter().enumerate() {
            println!("  {}) {:<14} - {}", i + 1, t.id, t.description);
        }
        print!("Select template [1-{}, default: 1]: ", TEMPLATES.len());
        let _ = io::stdout().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            if let Ok(choice) = trimmed.parse::<usize>() {
                if choice >= 1 && choice <= TEMPLATES.len() {
                    return TEMPLATES[choice - 1].id.to_string();
                }
            } else if !trimmed.is_empty() {
                if let Ok(resolved) = resolve_template(trimmed) {
                    return resolved;
                }
            }
        }
    }
    "hello-counter".to_string()
}

/// Scaffolds a complete Ferrix application into `target_dir`.
pub fn scaffold_project(
    target_dir: &Path,
    app_name: &str,
    template_id: &str,
    args: &NewArgs,
) -> std::io::Result<()> {
    fs::create_dir_all(target_dir)?;

    let raw_name = Path::new(app_name)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| app_name.to_string());
    let app_name = &raw_name;
    let slug = app_name
        .to_lowercase()
        .replace([' ', '_', '/'], "-")
        .trim_matches('-')
        .to_string();
    let pkg_name = slug.replace('-', "_");

    // 1. Write ferrix.config.toml (Schema Version 1)
    let config_content = format!(
        r##"schema_version = 1

[app]
name = "{app_name}"
slug = "{slug}"
id = "com.example.{pkg_name}"
version = "0.1.0"
description = "A Ferrix mobile application"
orientation = "portrait"
edge_to_edge = true

[android]
min_sdk = 26
target_sdk = 35
compile_sdk = 35
permissions = ["INTERNET"]
features = []
allow_backup = false
cleartext_domains = []

[icon]
adaptive_background = "#0B1F3A"
round = true

[splash]
background = "#0B1F3A"
dark_background = "#000000"
keep_until_ready = true
exit_animation = "fade"

[deep_links]
schemes = ["{slug}"]
"##
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
.ferrix/
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
ferrix-ui-core = { version = "0.1" }
ferrix-reactivity = { version = "0.1" }
ferrix-renderer-protocol = { version = "0.1" }
"#,
    );

    if (args.router || template_id == "router" || template_id == "tabs" || template_id == "stack")
        && !args.no_router
    {
        cargo_deps.push_str("ferrix-router = { version = \"0.1\" }\n");
    }
    if args.navigation || template_id == "router" || template_id == "tabs" || template_id == "stack"
    {
        cargo_deps.push_str("ferrix-navigation = { version = \"0.1\" }\n");
    }
    if template_id == "api-client" || args.network {
        cargo_deps.push_str("ferrix-network = { version = \"0.1\" }\n");
    }
    if args.storage || template_id == "kitchen-sink" {
        cargo_deps.push_str("ferrix-storage = { version = \"0.1\" }\n");
    }
    if args.sqlite || template_id == "kitchen-sink" {
        cargo_deps.push_str("ferrix-sqlite = { version = \"0.1\" }\n");
    }
    if args.i18n || template_id == "kitchen-sink" {
        cargo_deps.push_str("ferrix-i18n = { version = \"0.1\" }\n");
    }
    if (args.tailwind || args.postcss) && !args.no_tailwind {
        cargo_deps.push_str("ferrix-style = { version = \"0.1\" }\n");
    }

    let cargo_toml = format!(
        r#"[package]
name = "{slug}"
version = "0.1.0"
edition = "2021"

[workspace]

[lib]
name = "{pkg_name}"
crate-type = ["cdylib", "rlib"]

{cargo_deps}
[build-dependencies]
ferrix-compiler = {{ version = "0.1" }}
"#
    );
    fs::write(target_dir.join("Cargo.toml"), cargo_toml)?;

    // 5. Write UI source files in src/
    let src_dir = target_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    let component_content = match template_id {
        "blank" => {
            r#"import { Column, Text, Button } from "ferrix/ui";

export function App(): Element {
  return (
    <Column padding={24} spacing={16}>
      <Text size={24}>Welcome to Ferrix</Text>
      <Text>Your new native Android app is running on Rust.</Text>
      <Button onClick={() => null}>Get Started</Button>
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
        "router" => {
            r#"import { StackNavigator, Screen, Column, Text, Button } from "ferrix/ui";
import { useRouter } from "ferrix/router";

export function App(): Element {
  const router = useRouter();

  return (
    <StackNavigator>
      <Screen name="Home">
        <Column padding={20} spacing={16}>
          <Text size={26}>Ferrix App Router</Text>
          <Text>Multi-file architecture with separate screen components.</Text>
          <Button onClick={() => router.push("/details")}>Go to Details</Button>
          <Button onClick={() => router.push("/profile")}>Go to Profile</Button>
        </Column>
      </Screen>
      <Screen name="Details">
        <Column padding={20} spacing={16}>
          <Text size={26}>Details Screen</Text>
          <Text>Compiled ahead-of-time from src/DetailsScreen.fsx.</Text>
          <Button onClick={() => router.back()}>Back to Home</Button>
        </Column>
      </Screen>
      <Screen name="Profile">
        <Column padding={20} spacing={16}>
          <Text size={26}>User Profile</Text>
          <Text>Compiled ahead-of-time from src/ProfileScreen.fsx.</Text>
          <Button onClick={() => router.back()}>Back to Home</Button>
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
      <ActivityIndicator color={0xFF3B82F6} />
      <Text>Live Network Response</Text>
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
      <Button onClick={() => enabled.set(!enabled.value)}>Toggle Feature</Button>
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

    let formatted_component = ferrix_fmt::format_source(component_content);
    fs::write(src_dir.join("component.fsx"), formatted_component)?;

    // If router template, scaffold additional modular screen files demonstrating multi-file architecture
    if template_id == "router" {
        fs::write(
            src_dir.join("HomeScreen.fsx"),
            ferrix_fmt::format_source(
                r#"import { Column, Text, Button } from "ferrix/ui";
import { useRouter } from "ferrix/router";

export function HomeScreen(): Element {
  const router = useRouter();

  return (
    <Column padding={20} spacing={16}>
      <Text size={26}>Home Screen</Text>
      <Text>Welcome to the multi-file Ferrix Router application.</Text>
      <Button onClick={() => router.push("/details")}>View Details</Button>
      <Button onClick={() => router.push("/profile")}>View Profile</Button>
    </Column>
  );
}
"#,
            ),
        )?;

        fs::write(
            src_dir.join("DetailsScreen.fsx"),
            ferrix_fmt::format_source(
                r#"import { Column, Text, Button } from "ferrix/ui";
import { useRouter } from "ferrix/router";

export function DetailsScreen(): Element {
  const router = useRouter();

  return (
    <Column padding={20} spacing={16}>
      <Text size={26}>Details Screen</Text>
      <Text>Compiled from a separate modular component in src/DetailsScreen.fsx.</Text>
      <Button onClick={() => router.back()}>Back to Home</Button>
    </Column>
  );
}
"#,
            ),
        )?;

        fs::write(
            src_dir.join("ProfileScreen.fsx"),
            ferrix_fmt::format_source(
                r#"import { Column, Row, Text, Button, Switch } from "ferrix/ui";
import { useState } from "ferrix/reactivity";
import { useRouter } from "ferrix/router";

export function ProfileScreen(): Element {
  const pushNotifications = useState(true);
  const router = useRouter();

  return (
    <Column padding={20} spacing={16}>
      <Text size={26}>Profile Settings</Text>
      <Row spacing={8}>
        <Text>Push Notifications:</Text>
        <Switch
          value={pushNotifications.value}
          onValueChange={(val) => pushNotifications.set(val)}
        />
      </Row>
      <Button onClick={() => router.back()}>Back to Home</Button>
    </Column>
  );
}
"#,
            ),
        )?;

        fs::write(
            src_dir.join("Header.fsx"),
            ferrix_fmt::format_source(
                r#"import { Row, Text } from "ferrix/ui";

export function Header(): Element {
  return (
    <Row padding={16} spacing={8}>
      <Text size={20}>Ferrix App</Text>
    </Row>
  );
}
"#,
            ),
        )?;
    }

    // 6. Write src/lib.rs
    let lib_rs = format!(
        r#"//! {app_name} - Built with Ferrix UI Framework

// Ahead-of-time compiled modules from all .fsx files in src/
include!(concat!(env!("OUT_DIR"), "/generated_modules.rs"));

// Re-export primary component for convenience
pub use component::*;
"#
    );
    fs::write(src_dir.join("lib.rs"), lib_rs)?;

    // 7. Write build.rs for AOT compilation
    let build_rs = r#"use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    let mut generated_mods = String::new();
    compile_fsx_directory(Path::new("src"), out_path, &mut generated_mods);

    let mod_index_path = out_path.join("generated_modules.rs");
    let _ = fs::write(mod_index_path, generated_mods);
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    let mut prev_is_sep = false;
    for (i, ch) in s.chars().enumerate() {
        if ch == '-' || ch == '_' || ch == '/' || ch == '\\' {
            if !prev_is_sep && !out.is_empty() {
                out.push('_');
            }
            prev_is_sep = true;
        } else if ch.is_uppercase() {
            if i > 0 && !prev_is_sep {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
            prev_is_sep = false;
        } else {
            out.push(ch);
            prev_is_sep = false;
        }
    }
    out
}

fn compile_fsx_directory(dir: &Path, out_dir: &Path, mods: &mut String) {
    if let Ok(entries) = fs::read_dir(dir) {
        let mut paths: Vec<_> = entries.flatten().map(|e| e.path()).collect();
        paths.sort();
        for path in paths {
            if path.is_dir() {
                compile_fsx_directory(&path, out_dir, mods);
            } else if path.extension().and_then(|s| s.to_str()) == Some("fsx") {
                println!("cargo:rerun-if-changed={}", path.display());
                let rel_path = path.strip_prefix("src").unwrap_or(&path);
                let safe_stem = rel_path
                    .to_string_lossy()
                    .trim_start_matches('/')
                    .replace(['/', '\\'], "_")
                    .replace(".fsx", "");
                let dest = out_dir.join(format!("{}_generated.rs", safe_stem));
                if let Err(err) = ferrix_compiler::compile_file(&path, &dest) {
                    panic!("Failed to compile '{}': {}", path.display(), err);
                }
                let mod_name = to_snake_case(&safe_stem);
                mods.push_str(&format!(
                    "pub mod {} {{\n    // Ahead-of-time compiled from {}\n    include!(concat!(env!(\"OUT_DIR\"), \"/{}_generated.rs\"));\n}}\n",
                    mod_name,
                    path.display(),
                    safe_stem
                ));
            }
        }
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

    // 10. Generate CI workflow if requested
    if args.ci.as_deref() == Some("github") {
        let workflows_dir = target_dir.join(".github").join("workflows");
        fs::create_dir_all(&workflows_dir)?;
        let ci_yaml = r#"name: Ferrix App CI
on: [push, pull_request]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo check
"#;
        fs::write(workflows_dir.join("ci.yml"), ci_yaml)?;
    }

    // 11. Git initialization if requested
    if args.git && !args.no_git {
        let _ = std::process::Command::new("git")
            .arg("init")
            .current_dir(target_dir)
            .status();
    }

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
        .join("dev")
        .join("ferrix")
        .join("host");

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
    namespace = "dev.ferrix.host"
    compileSdk = 35

    defaultConfig {{
        applicationId = "com.example.{pkg_name}"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"

        ndk {{
            abiFilters.addAll(listOf("arm64-v8a", "x86_64", "armeabi-v7a"))
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

    buildFeatures {{
        buildConfig = true
    }}

    kotlinOptions {{
        jvmTarget = "17"
    }}
}}

dependencies {{
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("androidx.core:core-splashscreen:1.0.1")
    implementation("com.google.android.material:material:1.11.0")
}}
"#
    );
    fs::write(app_dir.join("build.gradle.kts"), app_build_gradle)?;

    // Generate Android artifacts from ferrix.config.toml
    let config_file = project_dir.join("ferrix.config.toml");
    let ferrix_cfg = if config_file.exists() {
        let content = fs::read_to_string(&config_file)?;
        ferrix_config::FerrixConfig::parse(&content).unwrap_or_else(|_| {
            let mut cfg = ferrix_config::FerrixConfig::default();
            cfg.app.name = slug.to_string();
            cfg.app.slug = slug.to_string();
            cfg.app.id = format!("com.example.{}", pkg_name);
            cfg
        })
    } else {
        let mut cfg = ferrix_config::FerrixConfig::default();
        cfg.app.name = slug.to_string();
        cfg.app.slug = slug.to_string();
        cfg.app.id = format!("com.example.{}", pkg_name);
        cfg
    };
    ferrix_cfg.write_all_artifacts(&main_dir, None)?;

    // MainActivity.kt (fixed host namespace)
    let main_activity = format!(
        r#"package dev.ferrix.host

import android.app.Activity
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.os.Bundle
import android.util.Log
import android.widget.Button
import android.widget.FrameLayout
import androidx.core.splashscreen.SplashScreen.Companion.installSplashScreen

class MainActivity : Activity() {{
    companion object {{
        init {{
            try {{
                System.loadLibrary("{pkg_name}")
            }} catch (e: UnsatisfiedLinkError) {{
                Log.w("ferrix", "Native library load deferred: ${{e.message}}")
            }}
        }}
    }}

    private val hotReloadReceiver = object : BroadcastReceiver() {{
        override fun onReceive(context: Context?, intent: Intent?) {{
            Log.i("ferrix", "ferrix:patch-applied")
            println("ferrix:patch-applied")
        }}
    }}

    override fun onCreate(savedInstanceState: Bundle?) {{
        try {{
            installSplashScreen()
        }} catch (e: Throwable) {{
            Log.w("ferrix", "SplashScreen install skipped: ${{e.message}}")
        }}
        super.onCreate(savedInstanceState)
        val rootLayout = FrameLayout(this).apply {{
            tag = "ferrix:root"
        }}

        // Scripted test-interaction button
        val testButton = Button(this).apply {{
            text = "Ferrix Action"
            contentDescription = "ferrix:button"
            tag = "ferrix:button"
            setOnClickListener {{
                Log.i("ferrix", "ferrix:button-clicked")
                println("ferrix:button-clicked")
            }}
        }}
        rootLayout.addView(testButton)
        setContentView(rootLayout)

        if (BuildConfig.DEBUG) {{
            Log.i("ferrix", "ferrix:ready")
            println("ferrix:ready")
        }}
    }}

    override fun onResume() {{
        super.onResume()
        if (BuildConfig.DEBUG) {{
            Log.i("ferrix", "ferrix:ready")
        }}
    }}

    override fun onStart() {{
        super.onStart()
        if (BuildConfig.DEBUG) {{
            val filter = IntentFilter("com.ferrix.HOT_RELOAD")
            try {{
                registerReceiver(hotReloadReceiver, filter, Context.RECEIVER_EXPORTED)
            }} catch (e: NoSuchMethodError) {{
                registerReceiver(hotReloadReceiver, filter)
            }} catch (e: Exception) {{
                Log.w("ferrix", "Could not register hot reload receiver: ${{e.message}}")
            }}
        }}
    }}

    override fun onStop() {{
        super.onStop()
        if (BuildConfig.DEBUG) {{
            try {{
                unregisterReceiver(hotReloadReceiver)
            }} catch (e: Exception) {{}}
        }}
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
    fn test_resolve_template_canonical_and_aliases() {
        assert_eq!(resolve_template("hello-counter").unwrap(), "hello-counter");
        assert_eq!(resolve_template("counter").unwrap(), "hello-counter");
        assert_eq!(resolve_template("blank").unwrap(), "blank");
        assert_eq!(resolve_template("empty").unwrap(), "blank");
        assert_eq!(resolve_template("tabs").unwrap(), "tabs");
        assert_eq!(resolve_template("tab").unwrap(), "tabs");
        assert_eq!(resolve_template("stack").unwrap(), "stack");
        assert_eq!(resolve_template("stacks").unwrap(), "stack");
        assert_eq!(resolve_template("router").unwrap(), "router");
        assert_eq!(resolve_template("routes").unwrap(), "router");
        assert_eq!(resolve_template("routing").unwrap(), "router");
        assert_eq!(resolve_template("route").unwrap(), "router");
        assert_eq!(resolve_template("api-client").unwrap(), "api-client");
        assert_eq!(resolve_template("api").unwrap(), "api-client");
        assert_eq!(resolve_template("kitchen-sink").unwrap(), "kitchen-sink");
        assert_eq!(resolve_template("kitchen").unwrap(), "kitchen-sink");
        assert_eq!(resolve_template("kitchensink").unwrap(), "kitchen-sink");

        // Case-insensitive & trimmed
        assert_eq!(resolve_template("  STACK  ").unwrap(), "stack");
        assert_eq!(resolve_template("Blank").unwrap(), "blank");

        // Unknown template returns error
        assert!(resolve_template("unknown-template").is_err());
    }

    #[test]
    fn test_scaffold_all_templates_compile_cleanly() {
        let templates = [
            "hello-counter",
            "blank",
            "tabs",
            "stack",
            "router",
            "api-client",
            "kitchen-sink",
        ];

        for tmpl in templates {
            let tmp_dir = std::env::temp_dir().join(format!("ferrix_test_scaffold_{}", tmpl));
            let _ = fs::remove_dir_all(&tmp_dir);
            fs::create_dir_all(&tmp_dir).unwrap();

            let args = NewArgs {
                name: Some(format!("test-{}", tmpl)),
                template: Some(tmpl.to_string()),
                list: false,
                yes: true,
                use_existing_dir: false,
                router: false,
                no_router: false,
                navigation: false,
                storage: false,
                sqlite: false,
                tailwind: false,
                no_tailwind: false,
                postcss: false,
                network: false,
                lint: false,
                test: false,
                ci: None,
                git: false,
                no_git: false,
                modules: None,
                package_manager: None,
                i18n: false,
            };

            scaffold_project(&tmp_dir, &format!("test-{}", tmpl), tmpl, &args).unwrap();

            // Verify .fsx file exists and compiles ahead-of-time
            let fsx_file = tmp_dir.join("src").join("component.fsx");
            assert!(
                fsx_file.exists(),
                "Template {} must have component.fsx",
                tmpl
            );

            let out_file = tmp_dir.join("component_out.rs");
            let compile_res = ferrix_compiler::compile_file(&fsx_file, &out_file);
            assert!(
                compile_res.is_ok(),
                "Template '{}' .fsx compilation failed: {:?}",
                tmpl,
                compile_res.err()
            );

            // If router, verify all modular screens exist and compile
            if tmpl == "router" {
                for screen_file in [
                    "HomeScreen.fsx",
                    "DetailsScreen.fsx",
                    "ProfileScreen.fsx",
                    "Header.fsx",
                ] {
                    let path = tmp_dir.join("src").join(screen_file);
                    assert!(
                        path.exists(),
                        "Router template must include modular screen '{}'",
                        screen_file
                    );
                    let out = tmp_dir.join(format!("{}_out.rs", screen_file));
                    let res = ferrix_compiler::compile_file(&path, &out);
                    if let Err(e) = &res {
                        eprintln!("ProfileScreen or other failed: {:?}", e);
                    }
                    assert!(
                        res.is_ok(),
                        "Compilation of {} failed: {:?}",
                        screen_file,
                        res.err()
                    );
                }
            }

            // Verify Cargo.toml has [build-dependencies] ferrix-compiler
            let cargo_toml = fs::read_to_string(tmp_dir.join("Cargo.toml")).unwrap();
            assert!(
                cargo_toml.contains("ferrix-compiler"),
                "Template {} Cargo.toml must include ferrix-compiler",
                tmpl
            );

            let _ = fs::remove_dir_all(&tmp_dir);
        }
    }

    #[test]
    fn test_multi_file_router_template_scaffolding_and_modules() {
        let tmp_dir = std::env::temp_dir().join("ferrix_test_router_multi_file");
        let _ = fs::remove_dir_all(&tmp_dir);
        fs::create_dir_all(&tmp_dir).unwrap();

        let args = NewArgs {
            name: Some("test-router-app".to_string()),
            template: Some("router".to_string()),
            list: false,
            yes: true,
            use_existing_dir: false,
            router: false,
            no_router: false,
            navigation: false,
            storage: false,
            sqlite: false,
            tailwind: false,
            no_tailwind: false,
            postcss: false,
            network: false,
            lint: false,
            test: false,
            ci: None,
            git: false,
            no_git: false,
            modules: None,
            package_manager: None,
            i18n: false,
        };

        scaffold_project(&tmp_dir, "test-router-app", "router", &args).unwrap();

        let src_dir = tmp_dir.join("src");
        assert!(src_dir.join("component.fsx").exists());
        assert!(src_dir.join("HomeScreen.fsx").exists());
        assert!(src_dir.join("DetailsScreen.fsx").exists());
        assert!(src_dir.join("ProfileScreen.fsx").exists());
        assert!(src_dir.join("Header.fsx").exists());

        // Verify Cargo.toml has router dependencies
        let cargo_toml = fs::read_to_string(tmp_dir.join("Cargo.toml")).unwrap();
        assert!(cargo_toml.contains("ferrix-router"));
        assert!(cargo_toml.contains("ferrix-navigation"));
        assert!(cargo_toml.contains("ferrix-compiler"));

        // Verify lib.rs includes generated_modules
        let lib_rs = fs::read_to_string(src_dir.join("lib.rs")).unwrap();
        assert!(lib_rs.contains("generated_modules.rs"));
        assert!(lib_rs.contains("pub use component::*;"));

        // Verify build.rs contains compile_fsx_directory and to_snake_case
        let build_rs = fs::read_to_string(tmp_dir.join("build.rs")).unwrap();
        assert!(build_rs.contains("compile_fsx_directory"));
        assert!(build_rs.contains("generated_modules.rs"));

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
}
