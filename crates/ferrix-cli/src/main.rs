//! # ferrix-cli
//!
//! Production command-line toolchain for the Ferrix UI framework.
//! Provides project scaffolding, toolchain diagnostics, live dev loop, AOT compilation,
//! code formatting, linter validation, and release packaging.

pub mod args;
pub mod commands;
pub mod templates;

use std::path::Path;
use std::process;

use clap::Parser;

use crate::args::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::New(args) => {
            let target_dir = Path::new(&args.name);
            println!(
                "🚀 Scaffolding new Ferrix app '{}' using template '{}'...\n",
                args.name, args.template
            );

            match templates::scaffold_project(target_dir, args) {
                Ok(()) => {
                    println!("\n✨ Project '{}' created successfully!\n", args.name);
                    println!("Next steps:");
                    println!("  cd {}", args.name);
                    println!("  ferrix doctor       # Verify Android toolchain prerequisites");
                    println!("  ferrix dev          # Start local development loop");
                    println!("  ferrix run android  # Run on emulator or device");
                    Ok(())
                }
                Err(e) => Err(format!("Scaffolding failed: {}", e)),
            }
        }
        Commands::Doctor(args) => commands::doctor::execute(args),
        Commands::Dev(args) => commands::dev::execute(args),
        Commands::Run(args) => commands::run::execute(args),
        Commands::Build(args) => commands::build::execute(args),
        Commands::Check(args) => commands::check::execute(args),
        Commands::Fmt(args) => commands::fmt::execute(args),
        Commands::Test(args) => commands::test::execute(args),
        Commands::Clean(args) => commands::clean::execute(args),
        Commands::Add(args) => commands::add::execute(args),
        Commands::Prebuild(args) => commands::prebuild::execute(args),
        Commands::Compile(args) => commands::compile::execute(args),
    };

    if let Err(err_msg) = result {
        eprintln!("\n❌ Error: {}", err_msg);
        process::exit(1);
    }
}
