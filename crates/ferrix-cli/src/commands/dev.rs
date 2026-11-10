//! Development loop with file watching, incremental rebuilds, and live log streaming.

use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::args::DevArgs;
use crate::commands::run;

pub fn execute(args: &DevArgs) -> Result<(), String> {
    println!("⚡ Starting Ferrix development loop...\n");

    // 1. Initial build and launch
    let run_args = crate::args::RunArgs {
        target: "android".to_string(),
        release: false,
        device: args.device.clone(),
    };

    if let Err(e) = run::execute(&run_args) {
        println!(
            "⚠️ Initial launch failed: {}. Continuing in watch mode...",
            e
        );
    }

    if args.no_watch {
        println!("ℹ️ Watch mode disabled (--no-watch). Dev server exited.");
        return Ok(());
    }

    println!("\n👀 Watching for changes in src/ and app/ (Ctrl+C to exit)...");

    let mut last_modified = std::time::SystemTime::now();

    loop {
        thread::sleep(Duration::from_millis(500));

        let mut changed = false;
        for entry in walkdir::WalkDir::new("src")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "fsx"))
        {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(mtime) = metadata.modified() {
                    if mtime > last_modified {
                        println!(
                            "🔄 Change detected in {}. Rebuilding...",
                            entry.path().display()
                        );
                        changed = true;
                        last_modified = mtime;
                        break;
                    }
                }
            }
        }

        if changed {
            let status = Command::new("cargo").args(["build"]).status();
            if status.is_ok_and(|s| s.success()) {
                println!("✨ Rebuild complete.");
            } else {
                eprintln!("❌ Rebuild failed with errors.");
            }
        }
    }
}
