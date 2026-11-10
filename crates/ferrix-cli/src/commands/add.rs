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

    match module.as_str() {
        "camera" => {
            append_cargo_dependency("ferrix-modules", "0.1")?;
            add_config_permission("CAMERA")?;
            println!("  ✅ Added 'ferrix-modules' to Cargo.toml");
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
            add_config_permission("ACCESS_FINE_LOCATION")?;
            add_config_permission("ACCESS_COARSE_LOCATION")?;
            println!("  ✅ Added 'ferrix-modules' to Cargo.toml");
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

fn add_config_permission(permission: &str) -> Result<(), String> {
    let config_path = Path::new("ferrix.config.toml");
    if !config_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read ferrix.config.toml: {}", e))?;
    let mut config = ferrix_config::FerrixConfig::parse(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    let perm_clean = permission.trim_start_matches("android.permission.");
    if !config
        .android
        .permissions
        .iter()
        .any(|p| p == perm_clean || p == permission)
    {
        config.android.permissions.push(perm_clean.to_string());
        let updated_toml =
            toml::to_string_pretty(&config).map_err(|e| format!("Failed to format toml: {}", e))?;
        fs::write(config_path, updated_toml)
            .map_err(|e| format!("Failed to write ferrix.config.toml: {}", e))?;
        println!("  ✅ Added '{}' to ferrix.config.toml", permission);
    }

    // Regenerate android artifacts if android/ exists
    let android_main = Path::new("android/app/src/main");
    if android_main.exists() {
        let _ = config.write_all_artifacts(android_main, None);
    }

    Ok(())
}

fn print_snippet(title: &str, code: &str) {
    println!("\n💡 Example snippet ({})", title);
    for line in code.lines() {
        println!("   {}", line);
    }
}
