#!/usr/bin/env bash
set -euo pipefail

echo "ℹ️ Devkit Context Information"
echo ""
echo "Working Directory: $(pwd)"
echo "Repository Root: $DEVKIT_REPO_ROOT"
echo ""
echo "This extension is located at:"
echo "  $DEVKIT_REPO_ROOT/.dev/extensions/test/"
echo ""
echo "✓ Extension system working correctly!"
