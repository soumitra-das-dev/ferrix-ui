#!/usr/bin/env node

/**
 * create-ferrix-app
 * Fast project bootstrapper for the Ferrix UI framework.
 */

const { spawnSync, execSync } = require("child_process");
const fs = require("fs");
const path = require("path");
const os = require("os");
const https = require("https");

const FERRIX_REPO = "soumitra-das-dev/ferrix-ui";
const VERSION = "0.1.0";

function checkLocalBinary() {
  const isWindows = os.platform() === "win32";
  const binName = isWindows ? "ferrix.exe" : "ferrix";

  // 1. Check ~/.cargo/bin (standard cargo install location)
  const cargoBin = path.join(os.homedir(), ".cargo", "bin", binName);
  if (fs.existsSync(cargoBin)) {
    return cargoBin;
  }

  // 2. Check ~/.ferrix/bin
  const ferrixBin = path.join(os.homedir(), ".ferrix", "bin", binName);
  if (fs.existsSync(ferrixBin)) {
    return ferrixBin;
  }

  // 3. Check system PATH
  try {
    const cmd = isWindows ? "where ferrix" : "which ferrix";
    const out = execSync(cmd, { stdio: ["ignore", "pipe", "ignore"] }).toString().trim();
    if (out && fs.existsSync(out)) {
      return out;
    }
  } catch (_) {}

  return null;
}

function getBinaryName() {
  const platform = os.platform();
  const arch = os.arch();

  let target = "";
  if (platform === "linux") {
    target = arch === "arm64" ? "aarch64-unknown-linux-gnu" : "x86_64-unknown-linux-gnu";
  } else if (platform === "darwin") {
    target = arch === "arm64" ? "aarch64-apple-darwin" : "x86_64-apple-darwin";
  } else if (platform === "win32") {
    target = "x86_64-pc-windows-msvc.exe";
  } else {
    throw new Error(`Unsupported platform: ${platform} (${arch})`);
  }

  return `ferrix-${target}`;
}

async function downloadBinary(destDir) {
  const binaryName = getBinaryName();
  const destPath = path.join(destDir, os.platform() === "win32" ? "ferrix.exe" : "ferrix");

  if (fs.existsSync(destPath)) {
    return destPath;
  }

  fs.mkdirSync(destDir, { recursive: true });

  const url = `https://github.com/${FERRIX_REPO}/releases/download/v${VERSION}/${binaryName}`;
  console.log(`⬇️ Downloading Ferrix CLI binary from ${url}...`);

  return new Promise((resolve, reject) => {
    https.get(url, (res) => {
      if (res.statusCode === 302 || res.statusCode === 301) {
        https.get(res.headers.location, (redirectRes) => {
          if (redirectRes.statusCode !== 200) {
            reject(new Error(`Failed to download binary: HTTP ${redirectRes.statusCode}`));
            return;
          }
          const file = fs.createWriteStream(destPath);
          redirectRes.pipe(file);
          file.on("finish", () => {
            file.close();
            fs.chmodSync(destPath, 0o755);
            resolve(destPath);
          });
        }).on("error", reject);
      } else if (res.statusCode === 200) {
        const file = fs.createWriteStream(destPath);
        res.pipe(file);
        file.on("finish", () => {
          file.close();
          fs.chmodSync(destPath, 0o755);
          resolve(destPath);
        });
      } else {
        reject(new Error(`Release binary not available (HTTP ${res.statusCode}). Install via 'cargo install ferrix-cli'.`));
      }
    }).on("error", reject);
  });
}

async function main() {
  const rawArgs = process.argv.slice(2);
  let binaryPath = checkLocalBinary();

  if (!binaryPath) {
    const ferrixHome = path.join(os.homedir(), ".ferrix", "bin");
    const cachedBinary = path.join(ferrixHome, os.platform() === "win32" ? "ferrix.exe" : "ferrix");
    if (fs.existsSync(cachedBinary)) {
      binaryPath = cachedBinary;
    } else {
      try {
        binaryPath = await downloadBinary(ferrixHome);
      } catch (err) {
        console.warn(`\n⚠️ Prebuilt binary download failed: ${err.message}`);
        console.log("ℹ️ Attempting to invoke cargo install ferrix-cli...");
        try {
          execSync("cargo install ferrix-cli", { stdio: "inherit" });
          binaryPath = checkLocalBinary();
        } catch (_) {}
      }
    }
  }

  if (!binaryPath) {
    console.error("\n❌ Could not locate or download the 'ferrix' CLI binary.");
    console.error("Please install Rust and run: cargo install ferrix-cli");
    process.exit(1);
  }

  // Prepend 'new' if not provided
  let finalArgs = rawArgs;
  if (finalArgs.length === 0) {
    finalArgs = ["new"];
  } else if (!["new", "create", "doctor", "dev", "run", "build", "check", "fmt", "test", "clean", "add", "prebuild", "--help", "-h", "--version", "-V"].includes(finalArgs[0])) {
    finalArgs = ["new", ...finalArgs];
  }

  const result = spawnSync(binaryPath, finalArgs, { stdio: "inherit" });
  process.exit(result.status ?? 0);
}

main().catch((err) => {
  console.error("Error:", err.message);
  process.exit(1);
});
