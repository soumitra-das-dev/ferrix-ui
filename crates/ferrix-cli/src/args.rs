//! Command-line argument definitions for the Ferrix CLI.

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "ferrix",
    author = "Ferrix Developers <https://github.com/soumitra-das-dev/ferrix-ui>",
    version,
    about = "Next-generation ahead-of-time compiled Android UI framework for Rust",
    long_about = "Ferrix CLI provides project scaffolding, toolchain diagnostics, ahead-of-time compilation, live device running, code formatting, linter validation, and release packaging under Architecture A."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scaffold a new Ferrix application
    #[command(alias = "create")]
    New(NewArgs),

    /// Diagnose system toolchain and Android development prerequisites
    Doctor(DoctorArgs),

    /// Start the development loop with file watching and device logging
    Dev(DevArgs),

    /// Build and run the app on an Android device or emulator
    Run(RunArgs),

    /// Build a production release APK or AAB
    Build(BuildArgs),

    /// Run type checks and static linter rules on .fsx components
    #[command(alias = "lint")]
    Check(CheckArgs),

    /// Style checking and Tailwind compilation
    Style(StyleArgs),

    /// Format .fsx files using the lossless AST formatter
    Fmt(FmtArgs),

    /// Run unit and headless component tests
    Test(TestArgs),

    /// Clean build caches and generated Android artifacts
    Clean(CleanArgs),

    /// Add a platform module or capability to the current project
    Add(AddArgs),

    /// Materialize or synchronize the native Android Studio project
    Prebuild(PrebuildArgs),

    /// Compile a single .fsx UI file ahead-of-time into Rust code
    Compile(CompileArgs),
}

#[derive(Args, Debug)]
pub struct NewArgs {
    /// Name of the new application/directory
    pub name: Option<String>,

    /// Starter template: blank, hello-counter, tabs, stack, router, api-client, kitchen-sink
    #[arg(short, long)]
    pub template: Option<String>,

    /// List all available starter templates
    #[arg(short, long)]
    pub list: bool,

    /// Non-interactive mode (accept all default options)
    #[arg(short, long)]
    pub yes: bool,

    /// Force scaffolding into an existing directory
    #[arg(long)]
    pub use_existing_dir: bool,

    /// Include file-based router (F1)
    #[arg(long)]
    pub router: bool,

    /// Explicitly disable router
    #[arg(long)]
    pub no_router: bool,

    /// Include native navigation containers (F2)
    #[arg(long)]
    pub navigation: bool,

    /// Include persistent KV storage (F3)
    #[arg(long)]
    pub storage: bool,

    /// Include embedded SQLite database with live queries (F3)
    #[arg(long)]
    pub sqlite: bool,

    /// Include Tailwind utility styling pipeline
    #[arg(long)]
    pub tailwind: bool,

    /// Disable Tailwind styling pipeline
    #[arg(long)]
    pub no_tailwind: bool,

    /// Include PostCSS pipeline
    #[arg(long)]
    pub postcss: bool,

    /// Include network client and data fetching
    #[arg(long)]
    pub network: bool,

    /// Include linter checks in project config
    #[arg(long)]
    pub lint: bool,

    /// Include headless component test suite
    #[arg(long)]
    pub test: bool,

    /// CI configuration: github or none
    #[arg(long)]
    pub ci: Option<String>,

    /// Initialize git repository
    #[arg(long)]
    pub git: bool,

    /// Do not initialize git repository
    #[arg(long)]
    pub no_git: bool,

    /// Include platform modules (comma-separated: camera,location,storage)
    #[arg(long)]
    pub modules: Option<String>,

    /// Node package manager (npm, pnpm, yarn, bun)
    #[arg(long)]
    pub package_manager: Option<String>,

    /// Include ICU multilingual internationalization (F5)
    #[arg(long)]
    pub i18n: bool,
}

#[derive(Args, Debug)]
pub struct DoctorArgs {
    /// Attempt automatic remediation for missing Rust Android targets
    #[arg(long)]
    pub fix: bool,

    /// Display verbose diagnostic messages and search paths
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug)]
pub struct DevArgs {
    /// Specific Android device or emulator serial
    #[arg(short, long)]
    pub device: Option<String>,

    /// Target platform (android)
    #[arg(long)]
    pub android: bool,

    /// AVD name to launch if no emulator running
    #[arg(long)]
    pub avd: Option<String>,

    /// List connected devices and exit
    #[arg(long)]
    pub list_devices: bool,

    /// Dev server port (default: 8081)
    #[arg(long)]
    pub port: Option<u16>,

    /// Clear build and bundler caches before starting
    #[arg(long)]
    pub clear: bool,

    /// Do not automatically launch the app on device
    #[arg(long)]
    pub no_open: bool,

    /// Run non-interactively without raw terminal UI
    #[arg(long)]
    pub no_interactive: bool,

    /// Output events as newline-delimited JSON
    #[arg(long)]
    pub json: bool,

    /// Build with release optimizations in dev
    #[arg(long)]
    pub release_like: bool,

    /// Disable automatic file watching
    #[arg(long)]
    pub no_watch: bool,

    /// Run in non-interactive smoke test mode and exit on completion
    #[arg(long)]
    pub smoke: bool,

    /// Timeout in seconds for smoke test mode
    #[arg(long, default_value = "300")]
    pub timeout: u64,
}

#[derive(Args, Debug)]
pub struct StyleArgs {
    #[command(subcommand)]
    pub action: Option<StyleAction>,
}

#[derive(Subcommand, Debug)]
pub enum StyleAction {
    /// Check CSS and Tailwind style declarations
    Check,
}

#[derive(Args, Debug)]
pub struct RunArgs {
    /// Target platform (currently 'android')
    #[arg(default_value = "android")]
    pub target: String,

    /// Run in release mode instead of debug
    #[arg(short, long)]
    pub release: bool,

    /// Specific Android device or emulator serial
    #[arg(short, long)]
    pub device: Option<String>,
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// Build release profile with optimizations
    #[arg(short, long)]
    pub release: bool,

    /// Build an Android App Bundle (.aab) for Google Play release
    #[arg(long)]
    pub aab: bool,

    /// Build an Android Package (.apk)
    #[arg(long)]
    pub apk: bool,

    /// Target ABI: aarch64, x86_64, armv7, or all
    #[arg(long, default_value = "aarch64")]
    pub abi: String,

    /// Custom output directory for the built artifact
    #[arg(short, long)]
    pub out: Option<String>,
}

#[derive(Args, Debug)]
pub struct CheckArgs {
    /// Directory or specific file to check (defaults to current directory)
    pub path: Option<String>,
}

#[derive(Args, Debug)]
pub struct FmtArgs {
    /// Directory or file to format (defaults to current directory)
    pub path: Option<String>,

    /// Check formatting without writing modifications (fails if not formatted)
    #[arg(long)]
    pub check: bool,
}

#[derive(Args, Debug)]
pub struct TestArgs {
    /// Filter pattern for test names
    pub filter: Option<String>,
}

#[derive(Args, Debug)]
pub struct CleanArgs {}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Module name to add (e.g., camera, location, storage, sqlite, i18n, devtools)
    pub module: String,
}

#[derive(Args, Debug)]
pub struct PrebuildArgs {
    /// Clean existing android/ directory before regenerating
    #[arg(long)]
    pub clean: bool,
}

#[derive(Args, Debug)]
pub struct CompileArgs {
    /// Path to input .fsx source file
    pub input: String,

    /// Optional output file path (defaults to <input>.rs)
    #[arg(short, long)]
    pub output: Option<String>,
}
