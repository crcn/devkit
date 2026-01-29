#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$SCRIPT_DIR"

need_cmd() { command -v "$1" >/dev/null 2>&1; }

export REPO_ROOT

# Kitchen Sink Mode: Use globally installed devkit binary
if need_cmd devkit; then
  exec devkit "$@"
fi

# If devkit not found, provide installation instructions
echo "❌ devkit not found in PATH"
echo
echo "Install devkit by running:"
echo
echo "  curl -fsSL https://raw.githubusercontent.com/crcn/devkit/main/install.sh | sh"
echo
echo "Or if you have devkit installed elsewhere, add it to your PATH:"
echo "  export PATH=\"/path/to/devkit:\$PATH\""
echo

exit 1
