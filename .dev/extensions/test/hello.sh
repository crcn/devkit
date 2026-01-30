#!/usr/bin/env bash
set -euo pipefail

echo "👋 Hello from devkit external extension!"
echo ""
echo "Repository: $DEVKIT_REPO_ROOT"
echo "Quiet mode: $DEVKIT_QUIET"
echo ""
echo "Features:"
echo "  Docker: $DEVKIT_FEATURE_DOCKER"
echo "  Git: $DEVKIT_FEATURE_GIT"
echo "  Cargo: $DEVKIT_FEATURE_CARGO"
echo "  Node: $DEVKIT_FEATURE_NODE"
echo "  Database: $DEVKIT_FEATURE_DATABASE"
echo ""
echo "✓ Test extension working!"
