//! Module installation and permission configuration.

use std::fs;
use std::path::Path;

use crate::args::AddArgs;

pub fn execute(args: &AddArgs) -> Result<(), String> {
    let module = args.module.to_lowercase();
    println!("📦 Adding module '{}' to Ferrix project...\n", module);

    let cargo_path = Path::new("Cargo.toml");
    if !cargo_path.exists() {
        return Err("No Cargo.toml found in current directory. Run from project root.".to_string());
    }

    let manifest_path = Path::new("android/app/src/main/AndroidManifest.xml");

    match module.as_str() {
        "camera" => {
            append_cargo_dependency("ferrix-modules", "0.1")?;
            inject_android_permission(manifest_path, "android.permission.CAMERA")?;
            println!("  ✅ Added 'ferrix-modules' to Cargo.toml");
            println!("  ✅ Added 'android.permission.CAMERA' to AndroidManifest.xml");
            print_snippet(
                "Camera Usage",
                r#"import { useCamera } from "ferrix/modules";

const camera = useCamera();
// camera.startPreview();
"#,
            );
        }
        "location" => {
            append_cargo_dependency("ferrix-modules", "0.1")?;
            inject_android_permission(manifest_path, "android.permission.ACCESS_FINE_LOCATION")?;
            inject_android_permission(manifest_path, "android.permission.ACCESS_COARSE_LOCATION")?;
            println!("  ✅ Added 'ferrix-modules' to Cargo.toml");
            println!("  ✅ Added location permissions to AndroidManifest.xml");
            print_snippet(
                "Location Usage",
                r#"import { useLocation } from "ferrix/modules";

const location = useLocation();
// location.start();
"#,
            );
        }
        "storage" => {
            append_cargo_dependency("ferrix-storage", "0.1")?;
            println!("  ✅ Added 'ferrix-storage' to Cargo.toml");
            print_snippet(
                "Storage Usage",
                r#"import { useStorage } from "ferrix/storage";

const [value, setValue] = useStorage("user_token", "");
"#,
            );
        }
        "sqlite" => {
            append_cargo_dependency("ferrix-sqlite", "0.1")?;
            println!("  ✅ Added 'ferrix-sqlite' to Cargo.toml");
            print_snippet(
                "SQLite Usage",
                r#"import { useLiveQuery } from "ferrix/sqlite";

const items = useLiveQuery("SELECT * FROM items ORDER BY id DESC", []);
"#,
            );
        }
        "router" => {
            append_cargo_dependency("ferrix-router", "0.1")?;
            println!("  ✅ Added 'ferrix-router' to Cargo.toml");
            print_snippet(
                "Router Usage",
                r#"import { useRouter, usePathname } from "ferrix/router";

const router = useRouter();
// router.push("/profile/123");
"#,
            );
        }
        "navigation" => {
            append_cargo_dependency("ferrix-navigation", "0.1")?;
            println!("  ✅ Added 'ferrix-navigation' to Cargo.toml");
            print_snippet(
                "Navigation Usage",
                r#"import { StackNavigator, Screen } from "ferrix/ui";

<StackNavigator>
  <Screen name="Home">...</Screen>
</StackNavigator>
"#,
            );
        }
        "i18n" => {
            append_cargo_dependency("ferrix-i18n", "0.1")?;
            println!("  ✅ Added 'ferrix-i18n' to Cargo.toml");
            print_snippet(
                "i18n Usage",
                r#"import { useTranslation } from "ferrix/i18n";

const [t, locale, setLocale] = useTranslation();
<Text>{t("greeting", { name: "Alice" })}</Text>
"#,
            );
        }
        "devtools" => {
            append_cargo_dependency("ferrix-devtools", "0.1")?;
            println!("  ✅ Added 'ferrix-devtools' to Cargo.toml");
            print_snippet(
                "DevTools Usage",
                r#"import { DevToolsOverlay } from "ferrix/devtools";

<DevToolsOverlay />
"#,
            );
        }
        other => {
            return Err(format!(
                "Unknown module '{}'. Available: camera, location, storage, sqlite, router, navigation, i18n, devtools",
                other
            ));
        }
    }

    Ok(())
}

fn append_cargo_dependency(name: &str, version: &str) -> Result<(), String> {
    let mut content = fs::read_to_string("Cargo.toml")
        .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;

    if content.contains(&format!("{} =", name)) {
        println!("  ℹ️ '{}' already present in Cargo.toml", name);
        return Ok(());
    }

    if let Some(pos) = content.find("[dependencies]") {
        let insert_idx = pos + "[dependencies]".len();
        content.insert_str(insert_idx, &format!("\n{} = \"{}\"", name, version));
        fs::write("Cargo.toml", content)
            .map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;
    } else {
        content.push_str(&format!("\n[dependencies]\n{} = \"{}\"\n", name, version));
        fs::write("Cargo.toml", content)
            .map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;
    }

    Ok(())
}

fn inject_android_permission(manifest_path: &Path, permission: &str) -> Result<(), String> {
    if !manifest_path.exists() {
        return Ok(()); // Native android not materialized, skip silently
    }

    let mut content =
        fs::read_to_string(manifest_path).map_err(|e| format!("Failed to read manifest: {}", e))?;

    let perm_tag = format!("<uses-permission android:name=\"{}\" />", permission);
    if content.contains(&perm_tag) {
        return Ok(());
    }

    if let Some(pos) = content.find("<manifest") {
        if let Some(tag_end) = content[pos..].find('>') {
            let insert_pos = pos + tag_end + 1;
            content.insert_str(insert_pos, &format!("\n    {}", perm_tag));
            fs::write(manifest_path, content)
                .map_err(|e| format!("Failed to update manifest: {}", e))?;
        }
    }

    Ok(())
}

fn print_snippet(title: &str, code: &str) {
    println!("\n💡 Example snippet ({})", title);
    for line in code.lines() {
        println!("   {}", line);
    }
}
