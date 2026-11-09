//! Test runner command for unit and headless component tests.

use std::process::Command;

use crate::args::TestArgs;

pub fn execute(args: &TestArgs) -> Result<(), String> {
    println!("🧪 Running Ferrix unit and headless component tests...\n");

    let mut cmd = Command::new("cargo");
    cmd.arg("test");

    if let Some(f) = &args.filter {
        cmd.arg(f);
    }

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to run cargo test: {}", e))?;

    if status.success() {
        println!("\n✅ All tests passed successfully!");
        Ok(())
    } else {
        Err("Test suite execution failed.".to_string())
    }
}
