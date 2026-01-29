#!/usr/bin/env bash
# devkit wrapper - auto-installs and runs devkit
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export REPO_ROOT="$SCRIPT_DIR"

need_cmd() { command -v "$1" &> /dev/null; }

# Ensure Rust is installed
if ! need_cmd cargo; then
    echo "📦 Installing Rust toolchain..."
    if ! need_cmd curl; then
        echo "Error: curl is required to install Rust" >&2
        exit 1
    fi
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    # shellcheck disable=SC1091
    [ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env"

    if ! need_cmd cargo; then
        echo "Error: Rust installation failed. Restart your terminal and try again." >&2
        exit 1
    fi
fi

# Install devkit if not present
if ! need_cmd devkit; then
    echo "📦 Installing devkit..."
    cargo install devkit-cli
fi

# Run devkit
exec devkit "$@"
