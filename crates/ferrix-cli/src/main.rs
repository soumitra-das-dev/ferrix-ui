//! # ferrix-cli
//!
//! Production command-line toolchain for the Ferrix UI framework.
//! Provides project scaffolding, toolchain diagnostics, live dev loop, AOT compilation,
//! code formatting, linter validation, and release packaging.

pub mod args;
pub mod commands;
pub mod crash;
pub mod dev_server;
pub mod device;
pub mod jni;
pub mod templates;

use std::path::Path;
use std::process;

use clap::Parser;

use crate::args::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::New(args) => {
            if args.list {
                templates::print_templates();
                return;
            }

            let app_name = match &args.name {
                Some(name) if !name.trim().is_empty() => name.trim().to_string(),
                _ if args.yes => "my-app".to_string(),
                _ => templates::prompt_app_name(),
            };

            let template_id = match &args.template {
                Some(t) => match templates::resolve_template(t) {
                    Ok(id) => id,
                    Err(err) => {
                        eprintln!("\n❌ Error: {}", err);
                        process::exit(1);
                    }
                },
                None => {
                    if args.router {
                        "router".to_string()
                    } else if args.yes {
                        "hello-counter".to_string()
                    } else {
                        templates::prompt_template()
                    }
                }
            };

            let target_dir = Path::new(&app_name);
            if target_dir.exists() && !args.use_existing_dir {
                if let Ok(mut entries) = std::fs::read_dir(target_dir) {
                    if entries.next().is_some() {
                        eprintln!("\n❌ Error: Target directory '{}' already exists and is not empty. Use --use-existing-dir to force scaffolding into an existing directory.", app_name);
                        process::exit(1);
                    }
                }
            }
            println!(
                "🚀 Scaffolding new Ferrix app '{}' using template '{}'...\n",
                app_name, template_id
            );

            match templates::scaffold_project(target_dir, &app_name, &template_id, args) {
                Ok(()) => {
                    println!("\n✨ Project '{}' created successfully!\n", app_name);
                    println!("Next steps:");
                    println!("  cd {}", app_name);
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
        Commands::Style(args) => commands::style::execute(args),
    };

    if let Err(err_msg) = result {
        eprintln!("\n❌ Error: {}", err_msg);
        process::exit(1);
    }
}
