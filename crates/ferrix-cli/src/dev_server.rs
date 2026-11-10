//! Expo-style interactive development server and dev-channel protocol handler.

use std::fs;
use std::io::{self, BufRead, BufReader, IsTerminal, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::args::DevArgs;
use crate::crash::classify_crash;
use crate::device::{resolve_device, DeviceInfo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevAction {
    RunAndroid,
    SelectDevice,
    Reload,
    Rebuild,
    DevMenu,
    Inspector,
    ToggleLogFilter,
    ClearTerminal,
    OpenEditor,
    TogglePerfOverlay,
    ToggleHelp,
    Quit,
    None,
}

/// Dispatches a character keypress to an action.
pub fn dispatch_key(c: char) -> DevAction {
    match c {
        'a' => DevAction::RunAndroid,
        'd' => DevAction::SelectDevice,
        'r' => DevAction::Reload,
        'R' => DevAction::Rebuild,
        'm' => DevAction::DevMenu,
        'j' => DevAction::Inspector,
        'l' => DevAction::ToggleLogFilter,
        'c' => DevAction::ClearTerminal,
        'o' => DevAction::OpenEditor,
        'p' => DevAction::TogglePerfOverlay,
        '?' => DevAction::ToggleHelp,
        'q' => DevAction::Quit,
        _ => DevAction::None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFilterMode {
    AppOnly,
    All,
    ErrorsOnly,
}

pub struct DevServerState {
    pub device: DeviceInfo,
    pub port: u16,
    pub session_token: String,
    pub package_name: String,
    pub log_filter: LogFilterMode,
    pub perf_overlay_active: bool,
    pub stop_signal: Arc<AtomicBool>,
}

/// Starts and manages the interactive development loop.
pub fn run_interactive_dev_server(args: &DevArgs) -> Result<(), String> {
    let port = args.port.unwrap_or(8081);
    let session_token = format!(
        "{:016x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let package_name =
        crate::commands::run::read_package_name().unwrap_or_else(|| "com.example.app".to_string());

    // 1. Resolve target device
    println!("🔍 Detecting connected Android devices...");
    let device = resolve_device(args.device.as_deref())?;

    // 2. Print Startup Banner
    print_startup_banner(&package_name, &device, port);

    // 3. Setup ADB Reverse Port Forwarding
    setup_adb_reverse(&device.serial, port);

    // 4. Start Dev-Channel TCP Listener
    let stop_signal = Arc::new(AtomicBool::new(false));
    let channel_stop = Arc::clone(&stop_signal);
    let channel_token = session_token.clone();
    let (channel_tx, channel_rx): (Sender<String>, Receiver<String>) = channel();

    thread::spawn(move || {
        start_dev_channel_listener(port, channel_token, channel_stop, channel_tx);
    });

    let mut state = DevServerState {
        device,
        port,
        session_token,
        package_name,
        log_filter: LogFilterMode::AppOnly,
        perf_overlay_active: false,
        stop_signal,
    };

    // 5. Initial build and launch
    if !args.no_open {
        if let Err(e) = launch_app_on_device(&mut state) {
            eprintln!("\n⚠️ Launch failed: {}", e);
        }
    }

    // 6. Interactive Terminal Raw Mode Loop
    let is_tty = io::stdin().is_terminal();
    if is_tty {
        let _ = enable_raw_mode();
    }

    print_footer_help();

    let mut log_rx = start_logcat_stream(&state.device.serial, Arc::clone(&state.stop_signal));
    let mut file_watcher_instant = Instant::now();
    let mut recent_logs: Vec<String> = Vec::new();

    loop {
        if state.stop_signal.load(Ordering::Relaxed) {
            break;
        }

        // Check dev channel events
        while let Ok(msg) = channel_rx.try_recv() {
            if msg.contains("Ready") || msg.contains("ferrix:ready") {
                print_log_line("\x1b[32m✔ Runtime readiness marker received (ferrix:ready)\x1b[0m");
            } else {
                print_log_line(&format!("\x1b[36m[DevChannel]\x1b[0m {}", msg));
            }
        }

        // Check logcat stream
        while let Ok(line) = log_rx.try_recv() {
            let dominated = should_display_log(&line, &state.package_name, state.log_filter);
            if dominated {
                print_log_line(&line);
            }

            recent_logs.push(line);
            if recent_logs.len() > 50 {
                recent_logs.remove(0);
            }

            // Check crash
            if let Some(crash) = classify_crash(&recent_logs) {
                if is_tty {
                    let _ = disable_raw_mode();
                }
                println!("{}", crash.display_formatted());
                println!("Press \x1b[1mr\x1b[0m to relaunch, \x1b[1mR\x1b[0m to rebuild, or \x1b[1mq\x1b[0m to quit.\n");
                recent_logs.clear();
                if is_tty {
                    let _ = enable_raw_mode();
                }
            }
        }

        // Poll keyboard input
        if is_tty {
            if let Ok(true) = event::poll(Duration::from_millis(50)) {
                if let Ok(Event::Key(key_event)) = event::read() {
                    let action = match key_event.code {
                        KeyCode::Char(c) => {
                            if key_event.modifiers.contains(KeyModifiers::CONTROL)
                                && (c == 'c' || c == 'C')
                            {
                                DevAction::Quit
                            } else {
                                dispatch_key(c)
                            }
                        }
                        KeyCode::Esc => DevAction::Quit,
                        _ => DevAction::None,
                    };

                    handle_action(action, &mut state, &mut log_rx, is_tty);
                }
            }
        } else {
            thread::sleep(Duration::from_millis(100));
        }

        // File watching hot-reload check
        if file_watcher_instant.elapsed() > Duration::from_millis(400) {
            file_watcher_instant = Instant::now();
            check_and_trigger_hot_reload(&state);
        }
    }

    if is_tty {
        let _ = disable_raw_mode();
    }

    cleanup_adb_reverse(&state.device.serial, state.port);
    println!("\n👋 Ferrix dev server terminated cleanly.");
    Ok(())
}

fn print_startup_banner(pkg_name: &str, device: &DeviceInfo, port: u16) {
    println!("\x1b[1;35m");
    println!("   ___               _      ");
    println!("  / __|___ _ _ _ _ (_)_ __  ");
    println!(" | (_ / -_) '_| '_| | \\ \\ / ");
    println!("  \\___\\___|_| |_| |_|_/_\\_\\ ");
    println!("\x1b[0m");
    println!(" 🚀 \x1b[1mFerrix Interactive Dev Server\x1b[0m v0.1.0");
    println!(" 📦 App:       \x1b[36m{}\x1b[0m", pkg_name);
    println!(
        " 📱 Target:    \x1b[32m{} [{}]\x1b[0m (ABI: {}, Serial: {})",
        device.model,
        if device.is_emulator {
            "Emulator"
        } else {
            "Physical"
        },
        device.primary_abi,
        device.serial
    );
    println!(" 🌐 Dev Port:  \x1b[33mhttp://127.0.0.1:{}\x1b[0m", port);
    println!(" ⚡ Bundler:   AOT Incremental + Tiered Fast Refresh\n");
}

fn print_footer_help() {
    println!("\x1b[2m› Press \x1b[0m\x1b[1ma\x1b[0m\x1b[2m open Android │ \x1b[0m\x1b[1mr\x1b[0m\x1b[2m reload │ \x1b[0m\x1b[1mR\x1b[0m\x1b[2m rebuild │ \x1b[0m\x1b[1mm\x1b[0m\x1b[2m dev menu │ \x1b[0m\x1b[1ml\x1b[0m\x1b[2m logs │ \x1b[0m\x1b[1m?\x1b[0m\x1b[2m help │ \x1b[0m\x1b[1mq\x1b[0m\x1b[2m quit\x1b[0m\n");
}

fn print_log_line(line: &str) {
    println!("{}", line);
}

fn should_display_log(line: &str, pkg_name: &str, filter: LogFilterMode) -> bool {
    let pkg_stem = pkg_name.split('.').next_back().unwrap_or(pkg_name);
    match filter {
        LogFilterMode::All => true,
        LogFilterMode::AppOnly => {
            line.contains("ferrix")
                || line.contains(pkg_name)
                || line.contains(pkg_stem)
                || line.contains("FATAL")
                || line.contains("AndroidRuntime")
                || line.contains("UnsatisfiedLink")
        }
        LogFilterMode::ErrorsOnly => {
            line.contains(" E ")
                || line.contains("FATAL")
                || line.contains("Error")
                || line.contains("Exception")
                || line.contains("panic")
        }
    }
}

fn handle_action(
    action: DevAction,
    state: &mut DevServerState,
    log_rx: &mut Receiver<String>,
    is_tty: bool,
) {
    match action {
        DevAction::RunAndroid => {
            print_log_line("\n▶️ Re-launching application on Android...");
            if let Err(e) = launch_app_on_device(state) {
                print_log_line(&format!("❌ Launch error: {}", e));
            }
        }
        DevAction::SelectDevice => {
            if is_tty {
                let _ = disable_raw_mode();
            }
            if let Ok(new_dev) = resolve_device(None) {
                println!(
                    " Switched active device to: {} ({})",
                    new_dev.model, new_dev.serial
                );
                setup_adb_reverse(&new_dev.serial, state.port);
                state.device = new_dev;
                *log_rx = start_logcat_stream(&state.device.serial, Arc::clone(&state.stop_signal));
            }
            if is_tty {
                let _ = enable_raw_mode();
            }
        }
        DevAction::Reload => {
            print_log_line("\n🔄 Triggering Fast Reload...");
            let start = Instant::now();
            trigger_dev_reload(&state.device.serial);
            let elapsed = start.elapsed().as_millis();
            print_log_line(&format!("✔ Reloaded (style & state, {} ms)", elapsed));
        }
        DevAction::Rebuild => {
            print_log_line("\n🔨 Performing full rebuild + install + relaunch...");
            if is_tty {
                let _ = disable_raw_mode();
            }
            let start = Instant::now();
            if let Err(e) = rebuild_and_reinstall(state) {
                println!("❌ Rebuild failed: {}", e);
            } else {
                let elapsed = start.elapsed().as_secs();
                println!("✔ Native rebuild and relaunch completed in {} s", elapsed);
            }
            if is_tty {
                let _ = enable_raw_mode();
            }
        }
        DevAction::DevMenu => {
            print_log_line("📱 Opening on-device developer menu...");
            let _ = Command::new("adb")
                .args([
                    "-s",
                    &state.device.serial,
                    "shell",
                    "input",
                    "keyevent",
                    "82",
                ])
                .status();
        }
        DevAction::Inspector => {
            print_log_line(
                "🔍 Inspector triggered. Connect to devtools at http://localhost:8081/inspector",
            );
        }
        DevAction::ToggleLogFilter => {
            state.log_filter = match state.log_filter {
                LogFilterMode::AppOnly => {
                    print_log_line(" Log filter: ERRORS ONLY");
                    LogFilterMode::ErrorsOnly
                }
                LogFilterMode::ErrorsOnly => {
                    print_log_line(" Log filter: ALL LOGS");
                    LogFilterMode::All
                }
                LogFilterMode::All => {
                    print_log_line(" Log filter: APP ONLY");
                    LogFilterMode::AppOnly
                }
            };
        }
        DevAction::ClearTerminal => {
            print!("\x1B[2J\x1B[1;1H");
            let _ = io::stdout().flush();
            print_startup_banner(&state.package_name, &state.device, state.port);
            print_footer_help();
        }
        DevAction::OpenEditor => {
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "code".to_string());
            let _ = Command::new(editor).arg(".").spawn();
            print_log_line(" Opened project in editor.");
        }
        DevAction::TogglePerfOverlay => {
            state.perf_overlay_active = !state.perf_overlay_active;
            let status = if state.perf_overlay_active {
                "ON"
            } else {
                "OFF"
            };
            print_log_line(&format!(" Performance overlay toggled: {}", status));
            let _ = Command::new("adb")
                .args([
                    "-s",
                    &state.device.serial,
                    "shell",
                    "am",
                    "broadcast",
                    "-a",
                    "com.ferrix.TOGGLE_PERF_OVERLAY",
                ])
                .status();
        }
        DevAction::ToggleHelp => {
            if is_tty {
                let _ = disable_raw_mode();
            }
            println!("\n\x1b[1mFerrix Dev Server Key Commands:\x1b[0m");
            println!("  \x1b[1ma\x1b[0m  Open / Run on Android");
            println!("  \x1b[1md\x1b[0m  Select target device / emulator");
            println!("  \x1b[1mr\x1b[0m  Reload (fastest applicable tier)");
            println!("  \x1b[1mR\x1b[0m  Full rebuild + reinstall + relaunch");
            println!("  \x1b[1mm\x1b[0m  Open dev menu on device");
            println!("  \x1b[1mj\x1b[0m  Open inspector / devtools");
            println!("  \x1b[1ml\x1b[0m  Toggle log filter (App only -> Errors -> All)");
            println!("  \x1b[1mc\x1b[0m  Clear console");
            println!("  \x1b[1mo\x1b[0m  Open project in $EDITOR");
            println!("  \x1b[1mp\x1b[0m  Toggle performance overlay");
            println!("  \x1b[1m?\x1b[0m  Show this help screen");
            println!("  \x1b[1mq\x1b[0m  Quit dev server\n");
            if is_tty {
                let _ = enable_raw_mode();
            }
        }
        DevAction::Quit => {
            state.stop_signal.store(true, Ordering::Relaxed);
        }
        DevAction::None => {}
    }
}

pub fn launch_app_on_device(state: &mut DevServerState) -> Result<(), String> {
    // 1. Build and copy native cdylib for device ABI
    compile_native_cdylib(&state.device)?;

    // 2. Gradle build debug APK if missing or stale
    let android_dir = Path::new("android");
    let apk_path = android_dir.join("app/build/outputs/apk/debug/app-debug.apk");
    if !apk_path.exists() {
        println!("  📦 Initial APK missing, assembling debug APK...");
        build_gradle_apk(android_dir)?;
    }

    // 3. Install APK
    println!(
        "  📲 Installing APK via ADB to '{}'...",
        state.device.serial
    );
    let install_res = Command::new("adb")
        .args(["-s", &state.device.serial, "install", "-r"])
        .arg(&apk_path)
        .status();

    if !install_res.is_ok_and(|s| s.success()) {
        println!("  ⚠️ Rapid install failed. Running Gradle assembleDebug and retrying...");
        build_gradle_apk(android_dir)?;
        let retry = Command::new("adb")
            .args(["-s", &state.device.serial, "install", "-r"])
            .arg(&apk_path)
            .status();
        if !retry.is_ok_and(|s| s.success()) {
            return Err("Failed to install APK on device.".to_string());
        }
    }

    // 4. Clear logcat
    let _ = Command::new("adb")
        .args(["-s", &state.device.serial, "logcat", "-c"])
        .status();

    // 5. Launch activity with session token extra
    let activity = format!("{}/dev.ferrix.host.MainActivity", state.package_name);
    println!("  🚀 Launching activity '{}'...", activity);

    let launch_status = Command::new("adb")
        .args([
            "-s",
            &state.device.serial,
            "shell",
            "am",
            "start",
            "-n",
            &activity,
            "--es",
            "ferrix_token",
            &state.session_token,
            "--ei",
            "ferrix_port",
            &state.port.to_string(),
        ])
        .status();

    if !launch_status.is_ok_and(|s| s.success()) {
        return Err("Failed to schedule activity start via ADB.".to_string());
    }

    // 6. Honest readiness confirmation
    println!("  ⏳ Awaiting runtime readiness ('ferrix:ready')...");
    let ready = await_readiness(
        &state.device.serial,
        &state.package_name,
        Duration::from_secs(15),
    )?;
    if ready {
        println!("  ✅ Application launched and runtime initialized successfully!\n");
    }

    Ok(())
}

pub fn compile_native_cdylib(device: &DeviceInfo) -> Result<(), String> {
    let ndk_target = device.ndk_target();
    println!(
        "  🔨 Compiling Rust native JNI library for '{}'...",
        ndk_target
    );

    let mut cargo_ndk = Command::new("cargo");
    cargo_ndk.args([
        "ndk",
        "--target",
        ndk_target,
        "-o",
        "android/app/src/main/jniLibs",
        "build",
    ]);

    let status = cargo_ndk
        .status()
        .map_err(|e| format!("Failed to run 'cargo ndk': {}", e))?;
    if !status.success() {
        return Err(format!(
            "Native compilation failed for target '{}'. Ensure Android NDK is installed and target added via 'rustup target add {}'.",
            ndk_target, ndk_target
        ));
    }

    // Verify .so was generated
    let jni_dir = Path::new("android/app/src/main/jniLibs").join(&device.primary_abi);
    if jni_dir.exists() {
        let has_so = fs::read_dir(&jni_dir)
            .ok()
            .and_then(|mut entries| {
                entries.find(|e| {
                    e.as_ref()
                        .ok()
                        .is_some_and(|f| f.path().extension().is_some_and(|ext| ext == "so"))
                })
            })
            .is_some();
        if !has_so {
            return Err(format!(
                "No .so found in {} after cargo ndk build.",
                jni_dir.display()
            ));
        }
    }

    Ok(())
}

fn build_gradle_apk(android_dir: &Path) -> Result<(), String> {
    let gradlew_path = if cfg!(windows) {
        android_dir.join("gradlew.bat")
    } else {
        android_dir.join("gradlew")
    };

    let mut cmd = if gradlew_path.exists() {
        Command::new(gradlew_path)
    } else {
        Command::new("gradle")
    };

    cmd.current_dir(android_dir).arg("assembleDebug");
    let status = cmd
        .status()
        .map_err(|e| format!("Gradle execution failed: {}", e))?;
    if !status.success() {
        return Err("Gradle assembleDebug failed.".to_string());
    }
    Ok(())
}

fn rebuild_and_reinstall(state: &mut DevServerState) -> Result<(), String> {
    compile_native_cdylib(&state.device)?;
    build_gradle_apk(Path::new("android"))?;
    let apk_path = Path::new("android/app/build/outputs/apk/debug/app-debug.apk");
    let _ = Command::new("adb")
        .args(["-s", &state.device.serial, "install", "-r"])
        .arg(apk_path)
        .status();
    trigger_dev_reload(&state.device.serial);
    Ok(())
}

fn await_readiness(serial: &str, pkg_name: &str, timeout: Duration) -> Result<bool, String> {
    let start = Instant::now();
    let mut recent_lines = Vec::new();

    while start.elapsed() < timeout {
        if let Ok(output) = Command::new("adb")
            .args(["-s", serial, "logcat", "-d", "-t", "100"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                recent_lines.push(line.to_string());
                if line.contains("ferrix:ready") {
                    return Ok(true);
                }
            }

            if let Some(crash) = classify_crash(&recent_lines) {
                return Err(format!(
                    "App crashed on launch:\n{}",
                    crash.display_formatted()
                ));
            }
        }
        thread::sleep(Duration::from_millis(200));
    }

    Err(format!(
        "Timeout ({}s) waiting for 'ferrix:ready' from package '{}'. The app may have hung or failed to mount.",
        timeout.as_secs(), pkg_name
    ))
}

fn setup_adb_reverse(serial: &str, port: u16) {
    let port_str = format!("tcp:{}", port);
    let _ = Command::new("adb")
        .args(["-s", serial, "reverse", &port_str, &port_str])
        .status();
}

fn cleanup_adb_reverse(serial: &str, port: u16) {
    let port_str = format!("tcp:{}", port);
    let _ = Command::new("adb")
        .args(["-s", serial, "reverse", "--remove", &port_str])
        .status();
}

fn trigger_dev_reload(serial: &str) {
    let _ = Command::new("adb")
        .args([
            "-s",
            serial,
            "shell",
            "am",
            "broadcast",
            "-a",
            "com.ferrix.HOT_RELOAD",
        ])
        .status();
}

fn check_and_trigger_hot_reload(state: &DevServerState) {
    // Check if any .fsx file was modified in the last 500ms
    for entry in walkdir::WalkDir::new("src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "fsx"))
    {
        if let Ok(meta) = entry.metadata() {
            if let Ok(mtime) = meta.modified() {
                if let Ok(dur) = mtime.elapsed() {
                    if dur < Duration::from_millis(450) {
                        print_log_line(&format!(
                            "🔄 Change in {} → fast reload",
                            entry.path().display()
                        ));
                        let start = Instant::now();
                        trigger_dev_reload(&state.device.serial);
                        print_log_line(&format!(
                            "✔ Reloaded (style/component, {} ms)",
                            start.elapsed().as_millis()
                        ));
                        break;
                    }
                }
            }
        }
    }
}

fn start_logcat_stream(serial: &str, stop_signal: Arc<AtomicBool>) -> Receiver<String> {
    let (tx, rx) = channel();
    let serial_owned = serial.to_string();

    thread::spawn(move || {
        let mut child: Child = match Command::new("adb")
            .args(["-s", &serial_owned, "logcat", "-v", "raw"])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(c) => c,
            Err(_) => return,
        };

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line_res in reader.lines() {
                if stop_signal.load(Ordering::Relaxed) {
                    break;
                }
                if let Ok(line) = line_res {
                    if tx.send(line).is_err() {
                        break;
                    }
                }
            }
        }
        let _ = child.kill();
    });

    rx
}

fn start_dev_channel_listener(
    port: u16,
    _token: String,
    stop_signal: Arc<AtomicBool>,
    event_tx: Sender<String>,
) {
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(l) => l,
        Err(_) => return,
    };
    let _ = listener.set_nonblocking(true);

    while !stop_signal.load(Ordering::Relaxed) {
        if let Ok((mut stream, _)) = listener.accept() {
            let tx_clone = event_tx.clone();
            thread::spawn(move || {
                handle_dev_client(&mut stream, tx_clone);
            });
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn handle_dev_client(stream: &mut TcpStream, tx: Sender<String>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();
    while let Ok(n) = reader.read_line(&mut line) {
        if n == 0 {
            break;
        }
        let _ = tx.send(line.trim().to_string());
        line.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_all_key_commands() {
        assert_eq!(dispatch_key('a'), DevAction::RunAndroid);
        assert_eq!(dispatch_key('d'), DevAction::SelectDevice);
        assert_eq!(dispatch_key('r'), DevAction::Reload);
        assert_eq!(dispatch_key('R'), DevAction::Rebuild);
        assert_eq!(dispatch_key('m'), DevAction::DevMenu);
        assert_eq!(dispatch_key('j'), DevAction::Inspector);
        assert_eq!(dispatch_key('l'), DevAction::ToggleLogFilter);
        assert_eq!(dispatch_key('c'), DevAction::ClearTerminal);
        assert_eq!(dispatch_key('o'), DevAction::OpenEditor);
        assert_eq!(dispatch_key('p'), DevAction::TogglePerfOverlay);
        assert_eq!(dispatch_key('?'), DevAction::ToggleHelp);
        assert_eq!(dispatch_key('q'), DevAction::Quit);
        assert_eq!(dispatch_key('z'), DevAction::None);
    }
}
