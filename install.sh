#!/usr/bin/env bash
set -euo pipefail

# Ferrix CLI Installer for Linux and macOS
REPO="soumitra-das-dev/ferrix-ui"
VERSION="0.1.0"
INSTALL_DIR="${HOME}/.ferrix/bin"

echo "⚡ Installing Ferrix CLI v${VERSION}..."

OS="$(uname -s)"
ARCH="$(uname -m)"

TARGET=""
case "${OS}" in
    Linux)
        case "${ARCH}" in
            x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
            aarch64|arm64) TARGET="aarch64-unknown-linux-gnu" ;;
            *) echo "❌ Unsupported architecture: ${ARCH}"; exit 1 ;;
        esac
        ;;
    Darwin)
        case "${ARCH}" in
            x86_64) TARGET="x86_64-apple-darwin" ;;
            arm64) TARGET="aarch64-apple-darwin" ;;
            *) echo "❌ Unsupported architecture: ${ARCH}"; exit 1 ;;
        esac
        ;;
    *)
        echo "❌ Unsupported operating system: ${OS}. On Windows, use install.ps1."
        exit 1
        ;;
esac

BINARY_NAME="ferrix-${TARGET}"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/v${VERSION}/${BINARY_NAME}"

mkdir -p "${INSTALL_DIR}"
TEMP_BIN="$(mktemp)"

echo "⬇️ Downloading ${DOWNLOAD_URL}..."
if command -v curl >/dev/null 2>&1; then
    curl -fsSL "${DOWNLOAD_URL}" -o "${TEMP_BIN}" || true
elif command -v wget >/dev/null 2>&1; then
    wget -qO "${TEMP_BIN}" "${DOWNLOAD_URL}" || true
fi

if [ -s "${TEMP_BIN}" ]; then
    mv "${TEMP_BIN}" "${INSTALL_DIR}/ferrix"
    chmod +x "${INSTALL_DIR}/ferrix"
else
    rm -f "${TEMP_BIN}"
    echo "⚠️ Prebuilt binary release not found online."
    if command -v cargo >/dev/null 2>&1; then
        echo "🔨 Compiling locally from source using cargo..."
        cargo install ferrix-cli
    else
        echo "❌ Could not install Ferrix. Please install Rust (https://rustup.rs) and run: cargo install ferrix-cli"
        exit 1
    fi
fi

# Ensure PATH contains install directory
SHELL_CONFIG=""
if [ -n "${ZSH_VERSION:-}" ] || [ -f "${HOME}/.zshrc" ]; then
    SHELL_CONFIG="${HOME}/.zshrc"
elif [ -f "${HOME}/.bashrc" ]; then
    SHELL_CONFIG="${HOME}/.bashrc"
fi

if [ -n "${SHELL_CONFIG}" ]; then
    if ! grep -q ".ferrix/bin" "${SHELL_CONFIG}"; then
        echo 'export PATH="${HOME}/.ferrix/bin:${PATH}"' >> "${SHELL_CONFIG}"
        echo "ℹ️ Added ~/.ferrix/bin to ${SHELL_CONFIG}"
    fi
fi

echo "✨ Ferrix CLI installed successfully!"
echo "Run 'ferrix --help' to get started."
