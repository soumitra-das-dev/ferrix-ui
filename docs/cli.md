# Ferrix CLI Reference Manual

The `ferrix` command-line interface provides end-to-end tooling for ahead-of-time compiled Android applications built with Ferrix.

## 1. Installation

### Option A: Cargo (Rust)
```bash
cargo install ferrix-cli
```

### Option B: Quick Shell Installer (Linux / macOS)
```bash
curl -fsSL https://raw.githubusercontent.com/soumitra-das-dev/ferrix-ui/master/install.sh | bash
```

### Option C: PowerShell Installer (Windows)
```powershell
irm https://raw.githubusercontent.com/soumitra-das-dev/ferrix-ui/master/install.ps1 | iex
```

### Option D: npm Launcher
```bash
npx create-ferrix-app my-app
```

---

## 2. Command Reference

| Command | Description | Common Flags |
|---|---|---|
| `ferrix new <name>` | Scaffolds a new project | `--template <t>`, `--router`, `--navigation`, `--storage`, `--sqlite`, `--i18n`, `--tailwind` |
| `ferrix doctor` | Diagnoses toolchain prerequisites | `--fix`, `--verbose` |
| `ferrix dev` | Starts live development loop with file watching | `--no-watch`, `--device <id>` |
| `ferrix run android` | Builds and launches on emulator/device | `--release`, `--device <id>` |
| `ferrix build` | Builds production APK or AAB bundle | `--release`, `--apk`, `--aab`, `--abi <aarch64\|x86_64\|all>` |
| `ferrix check [path]` | Runs AST syntax and linter checks on `.fsx` files | |
| `ferrix fmt [path]` | Formats `.fsx` files with lossless trivia preservation | `--check` |
| `ferrix test [filter]` | Runs unit and headless component tests | |
| `ferrix add <module>` | Adds a platform module (`camera`, `sqlite`, etc.) | |
| `ferrix prebuild` | Materializes or updates the native `android/` project | `--clean` |
| `ferrix clean` | Cleans cargo target and Android build caches | |
| `ferrix compile <file>` | Ahead-of-time compiles single `.fsx` to Rust `.rs` | `-o <out>` |

---

## 3. Templates

Ferrix comes with 6 built-in production starter templates:
1. `hello-counter` (default): Interactive stateful counter using `useState` and fine-grained patches.
2. `blank`: Minimal single-screen starter with one button and text.
3. `tabs`: Bottom tab navigation with 3 screens (`Home`, `Explore`, `Profile`).
4. `stack`: Master-detail navigation stack with typed route parameters.
5. `api-client`: Network query client using `useQuery` with loading states.
6. `kitchen-sink`: Comprehensive showcase of primitives, switches, inputs, and storage.
