//! Android device discovery, ABI resolution, and state persistence.

use std::fs;
use std::io::{self, IsTerminal, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    pub serial: String,
    pub model: String,
    pub primary_abi: String,
    pub abi_list: Vec<String>,
    pub is_emulator: bool,
    pub state: String,
}

impl DeviceInfo {
    /// Maps the device's primary ABI to the corresponding Rust NDK target tuple.
    pub fn ndk_target(&self) -> &'static str {
        match self.primary_abi.as_str() {
            "arm64-v8a" => "aarch64-linux-android",
            "x86_64" => "x86_64-linux-android",
            "armeabi-v7a" => "armv7-linux-androideabi",
            "x86" => "i686-linux-android",
            _ => "aarch64-linux-android",
        }
    }
}

/// Parses output from `adb devices -l`.
pub fn parse_adb_devices_output(output: &str) -> Vec<DeviceInfo> {
    let mut devices = Vec::new();

    for line in output.lines().skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let serial = parts[0].to_string();
        let state = parts[1].to_string();

        let mut model = "Android Device".to_string();
        for part in &parts[2..] {
            if let Some(m) = part.strip_prefix("model:") {
                model = m.to_string();
                break;
            }
        }

        let is_emulator = serial.starts_with("emulator-") || model.contains("sdk_gphone");

        devices.push(DeviceInfo {
            serial,
            model,
            primary_abi: "unknown".to_string(),
            abi_list: Vec::new(),
            is_emulator,
            state,
        });
    }

    devices
}

/// Queries `ro.product.cpu.abilist` and `ro.product.cpu.abi` for a connected device.
pub fn query_device_abis(serial: &str) -> (String, Vec<String>) {
    let mut abilist_cmd = Command::new("adb");
    abilist_cmd.args(["-s", serial, "shell", "getprop", "ro.product.cpu.abilist"]);

    let mut list = Vec::new();
    if let Ok(out) = abilist_cmd.output() {
        let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !text.is_empty() {
            for item in text.split(',') {
                let trimmed = item.trim();
                if !trimmed.is_empty() {
                    list.push(trimmed.to_string());
                }
            }
        }
    }

    let mut primary = String::new();
    if !list.is_empty() {
        primary = list[0].clone();
    } else {
        let mut abi_cmd = Command::new("adb");
        abi_cmd.args(["-s", serial, "shell", "getprop", "ro.product.cpu.abi"]);
        if let Ok(out) = abi_cmd.output() {
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !text.is_empty() {
                primary = text.clone();
                list.push(text);
            }
        }
    }

    if primary.is_empty() {
        primary = "x86_64".to_string();
        list.push("x86_64".to_string());
    }

    (primary, list)
}

/// Discovers all connected devices and populates their ABI information.
pub fn list_connected_devices() -> Result<Vec<DeviceInfo>, String> {
    let output = Command::new("adb")
        .args(["devices", "-l"])
        .output()
        .map_err(|e| format!("Failed to run 'adb devices -l': {}", e))?;

    let text = String::from_utf8_lossy(&output.stdout);
    let mut devices = parse_adb_devices_output(&text);

    for dev in &mut devices {
        if dev.state == "device" {
            let (primary, abis) = query_device_abis(&dev.serial);
            dev.primary_abi = primary;
            dev.abi_list = abis;
        }
    }

    Ok(devices)
}

/// Resolves the target device to use, handling explicit flag, persistence, and interactive selection.
pub fn resolve_device(explicit_device: Option<&str>) -> Result<DeviceInfo, String> {
    let devices = list_connected_devices()?;

    if devices.is_empty() {
        return Err("No Android devices or running emulators found via ADB.\nStart an emulator or connect a device with USB debugging enabled.".to_string());
    }

    // 1. If explicit device specified via --device
    if let Some(target) = explicit_device {
        if let Some(dev) = devices
            .iter()
            .find(|d| d.serial == target || d.model == target)
        {
            save_device_state(&dev.serial);
            return Ok(dev.clone());
        }
        return Err(format!(
            "Specified device '{}' not found in connected devices: {:?}",
            target,
            devices.iter().map(|d| &d.serial).collect::<Vec<_>>()
        ));
    }

    // Check for unauthorized devices
    for dev in &devices {
        if dev.state == "unauthorized" {
            eprintln!(
                "⚠️ Device '{}' is UNAUTHORIZED. Please accept the USB debugging RSA prompt on the device screen.",
                dev.serial
            );
        }
    }

    let ready_devices: Vec<&DeviceInfo> = devices.iter().filter(|d| d.state == "device").collect();
    if ready_devices.is_empty() {
        return Err("Connected devices found, but none are in 'device' state (all unauthorized or offline).".to_string());
    }

    // 2. Single device available
    if ready_devices.len() == 1 {
        let dev = ready_devices[0].clone();
        save_device_state(&dev.serial);
        return Ok(dev);
    }

    // 3. Check previously persisted selection
    if let Some(saved_serial) = load_device_state() {
        if let Some(dev) = ready_devices.iter().find(|d| d.serial == saved_serial) {
            return Ok((*dev).clone());
        }
    }

    // 4. Interactive selection if terminal is interactive
    if io::stdin().is_terminal() {
        println!("? Multiple Android devices detected. Select a target:");
        for (i, dev) in ready_devices.iter().enumerate() {
            let kind = if dev.is_emulator {
                "Emulator"
            } else {
                "Physical"
            };
            println!(
                "  {}) {} [{}] (ABI: {}, Serial: {})",
                i + 1,
                dev.model,
                kind,
                dev.primary_abi,
                dev.serial
            );
        }
        print!("Select device [1-{}, default: 1]: ", ready_devices.len());
        let _ = io::stdout().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            if let Ok(choice) = trimmed.parse::<usize>() {
                if choice >= 1 && choice <= ready_devices.len() {
                    let chosen = ready_devices[choice - 1].clone();
                    save_device_state(&chosen.serial);
                    return Ok(chosen);
                }
            }
        }
    }

    // Default to first ready device
    let dev = ready_devices[0].clone();
    save_device_state(&dev.serial);
    Ok(dev)
}

fn save_device_state(serial: &str) {
    let dir = Path::new(".ferrix");
    let _ = fs::create_dir_all(dir);
    let state_file = dir.join("dev-state.json");
    let json = format!("{{\"selected_device\": \"{}\"}}\n", serial);
    let _ = fs::write(state_file, json);
}

fn load_device_state() -> Option<String> {
    let state_file = Path::new(".ferrix").join("dev-state.json");
    if let Ok(content) = fs::read_to_string(state_file) {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(serial) = val.get("selected_device").and_then(|s| s.as_str()) {
                return Some(serial.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_adb_devices_output_multi() {
        let sample = "List of devices attached\nemulator-5554          device product:sdk_gphone64_x86_64 model:sdk_gphone64_x86_64 device:emu64xa transport_id:5\nZY22H8L5GQ             device usb:1-1 product:kiev model:moto_g34_5g device:kiev transport_id:6\n192.168.1.100:5555     unauthorized transport_id:7\n";
        let devs = parse_adb_devices_output(sample);
        assert_eq!(devs.len(), 3);
        assert_eq!(devs[0].serial, "emulator-5554");
        assert!(devs[0].is_emulator);
        assert_eq!(devs[0].state, "device");

        assert_eq!(devs[1].serial, "ZY22H8L5GQ");
        assert_eq!(devs[1].model, "moto_g34_5g");
        assert!(!devs[1].is_emulator);
        assert_eq!(devs[1].state, "device");

        assert_eq!(devs[2].serial, "192.168.1.100:5555");
        assert_eq!(devs[2].state, "unauthorized");
    }

    #[test]
    fn test_ndk_target_mapping() {
        let mut dev = DeviceInfo {
            serial: "test".to_string(),
            model: "test".to_string(),
            primary_abi: "arm64-v8a".to_string(),
            abi_list: vec!["arm64-v8a".to_string()],
            is_emulator: false,
            state: "device".to_string(),
        };
        assert_eq!(dev.ndk_target(), "aarch64-linux-android");

        dev.primary_abi = "x86_64".to_string();
        assert_eq!(dev.ndk_target(), "x86_64-linux-android");

        dev.primary_abi = "armeabi-v7a".to_string();
        assert_eq!(dev.ndk_target(), "armv7-linux-androideabi");
    }
}
