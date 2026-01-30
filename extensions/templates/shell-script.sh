#!/usr/bin/env bash
# Example shell script extension
# Make executable: chmod +x shell-script.sh

set -euo pipefail

# Environment variables provided by devkit:
# - DEVKIT_REPO_ROOT: Repository root path
# - DEVKIT_QUIET: "1" if quiet mode, "0" otherwise
# - DEVKIT_FEATURE_DOCKER: "1" if Docker available, "0" otherwise
# - DEVKIT_FEATURE_GIT: "1" if Git available, "0" otherwise
# - DEVKIT_FEATURE_CARGO: "1" if Cargo available, "0" otherwise
# - DEVKIT_FEATURE_NODE: "1" if Node.js available, "0" otherwise
# - DEVKIT_FEATURE_DATABASE: "1" if database configured, "0" otherwise

echo "👋 Hello from $DEVKIT_REPO_ROOT!"

if [ "$DEVKIT_FEATURE_DOCKER" = "1" ]; then
    echo "✓ Docker is available"
fi

if [ "$DEVKIT_FEATURE_GIT" = "1" ]; then
    echo "✓ Git is available"
fi

if [ "$DEVKIT_QUIET" = "1" ]; then
    echo "(Running in quiet mode)"
fi

# Your custom logic here
echo "Running custom workflow..."

echo "✓ Done!"
